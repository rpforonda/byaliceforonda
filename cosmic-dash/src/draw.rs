//! Everything that paints the canvas. Each frame is drawn from scratch by `Game::render`.

use std::f64::consts::PI;

use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d as Ctx;

use crate::{C, CHARACTERS, Character, GROUND, Game, H, Mode, ObKind, Obstacle, PLAYER_X, Particle, Planet, PlanetKind, Rect, SPAWN_AHEAD, SpecialKind, ZONES, rgba};

type Res = Result<(), JsValue>;

const TAU: f64 = PI * 2.0;

fn font(size: f64) -> String {
    format!("700 {size}px Nunito, system-ui, sans-serif")
}

fn pad(n: f64) -> String {
    format!("{:05}", n.floor() as i64)
}

fn star_path(ctx: &Ctx, cx: f64, cy: f64, r: f64, rot: f64) {
    const INNER: f64 = 0.48;
    ctx.begin_path();
    for i in 0..10 {
        let a = rot - PI / 2.0 + (i as f64 * PI) / 5.0;
        let rr = if i % 2 == 1 { r * INNER } else { r };
        let (px, py) = (cx + a.cos() * rr, cy + a.sin() * rr);
        if i == 0 { ctx.move_to(px, py) } else { ctx.line_to(px, py) }
    }
    ctx.close_path();
}

fn smooth_blob(ctx: &Ctx, cx: f64, cy: f64, pts: &[(f64, f64)]) {
    ctx.begin_path();
    let n = pts.len();
    let mid = |i: usize| {
        let a = pts[i % n];
        let b = pts[(i + 1) % n];
        ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0)
    };
    let m0 = mid(n - 1);
    ctx.move_to(cx + m0.0, cy + m0.1);
    for (i, p) in pts.iter().enumerate() {
        let m = mid(i);
        ctx.quadratic_curve_to(cx + p.0, cy + p.1, cx + m.0, cy + m.1);
    }
    ctx.close_path();
}

fn round_rect(ctx: &Ctx, x: f64, y: f64, w: f64, h: f64, r: f64) -> Res {
    ctx.begin_path();
    ctx.move_to(x + r, y);
    ctx.arc_to(x + w, y, x + w, y + h, r)?;
    ctx.arc_to(x + w, y + h, x, y + h, r)?;
    ctx.arc_to(x, y + h, x, y, r)?;
    ctx.arc_to(x, y, x + w, y, r)?;
    ctx.close_path();
    Ok(())
}

fn circle(ctx: &Ctx, x: f64, y: f64, r: f64) -> Res {
    ctx.arc(x, y, r, 0.0, TAU)
}

impl Game {
    pub(crate) fn render(&mut self) -> Res {
        let Some(ctx) = self.ctx.clone() else { return Ok(()) };
        let ctx = &ctx;
        self.draw_background(ctx)?;
        self.draw_ground(ctx)?;
        if self.mode == Mode::Ready {
            return self.draw_intro(ctx);
        }
        for ob in &self.obstacles {
            self.draw_obstacle(ctx, ob)?;
        }
        self.draw_pickups(ctx);
        self.draw_player(ctx)?;
        self.draw_effects(ctx)?;
        if self.active.kind == Some(SpecialKind::Calm) {
            ctx.set_fill_style_str("rgba(95,224,208,0.08)");
            ctx.fill_rect(0.0, 0.0, self.w, H);
        }
        self.draw_navigator_markers(ctx)?;
        self.draw_hud(ctx)?;
        self.draw_overlay(ctx)
    }

    // parallax position that wraps around a strip wider than the canvas
    fn wrap_x(&self, x: f64, par: f64) -> f64 {
        let span = (self.w + 80.0).max(1000.0);
        (((x - self.distance * par) % span) + span) % span - 40.0
    }

    fn draw_planet(&self, ctx: &Ctx, p: &Planet) -> Res {
        let x = p.x - self.distance * 0.05;
        if x < -60.0 || x > self.w + 60.0 {
            return Ok(());
        }
        let (y, r) = (p.y, p.r);
        match p.kind {
            PlanetKind::Ringed => {
                ctx.set_stroke_style_str("rgba(255,210,141,0.75)");
                ctx.set_line_width(3.0);
                ctx.begin_path();
                ctx.ellipse(x, y, r * 1.9, r * 0.5, -0.3, PI, TAU)?;
                ctx.stroke();
                let g = ctx.create_radial_gradient(x - r * 0.4, y - r * 0.4, r * 0.2, x, y, r)?;
                g.add_color_stop(0.0, "#ffc4dd")?;
                g.add_color_stop(1.0, "#b06ab3")?;
                ctx.set_fill_style_canvas_gradient(&g);
                ctx.begin_path();
                circle(ctx, x, y, r)?;
                ctx.fill();
                ctx.begin_path();
                ctx.ellipse(x, y, r * 1.9, r * 0.5, -0.3, 0.0, PI)?;
                ctx.stroke();
            }
            PlanetKind::Moon => {
                ctx.set_fill_style_str("#d9d2f0");
                ctx.begin_path();
                circle(ctx, x, y, r)?;
                ctx.fill();
                ctx.set_fill_style_str("#b5abd8");
                ctx.begin_path();
                circle(ctx, x - r * 0.3, y - r * 0.2, r * 0.28)?;
                circle(ctx, x + r * 0.35, y + r * 0.3, r * 0.2)?;
                ctx.fill();
            }
            PlanetKind::Blue => {
                let g = ctx.create_radial_gradient(x - r * 0.4, y - r * 0.4, r * 0.1, x, y, r)?;
                g.add_color_stop(0.0, "#c7e9ff")?;
                g.add_color_stop(1.0, "#3a7bd5")?;
                ctx.set_fill_style_canvas_gradient(&g);
                ctx.begin_path();
                circle(ctx, x, y, r)?;
                ctx.fill();
            }
        }
        Ok(())
    }

    fn draw_background(&self, ctx: &Ctx) -> Res {
        let w = self.w;
        let sky = ctx.create_linear_gradient(0.0, 0.0, 0.0, GROUND);
        sky.add_color_stop(0.0, C::SKY_TOP)?;
        sky.add_color_stop(0.55, C::SKY_MID)?;
        sky.add_color_stop(1.0, C::SKY_LOW)?;
        ctx.set_fill_style_canvas_gradient(&sky);
        ctx.fill_rect(0.0, 0.0, w, H);

        // nebulae glow in the current zone's colors
        for n in &self.nebulae {
            let x = self.wrap_x(n.x, 0.02);
            let col = if n.use_a { self.neb_a } else { self.neb_b };
            let g = ctx.create_radial_gradient(x, n.y, 0.0, x, n.y, n.r)?;
            g.add_color_stop(0.0, &rgba(col, 0.32))?;
            g.add_color_stop(1.0, &rgba(col, 0.0))?;
            ctx.set_fill_style_canvas_gradient(&g);
            ctx.fill_rect(x - n.r, n.y - n.r, n.r * 2.0, n.r * 2.0);
        }
        let horizon = ctx.create_linear_gradient(0.0, GROUND - 70.0, 0.0, GROUND);
        horizon.add_color_stop(0.0, &rgba(self.neb_b, 0.0))?;
        horizon.add_color_stop(1.0, &rgba(self.neb_b, 0.28))?;
        ctx.set_fill_style_canvas_gradient(&horizon);
        ctx.fill_rect(0.0, GROUND - 70.0, w, 70.0);

        // twinkling star field, three parallax layers
        ctx.set_fill_style_str("#ffffff");
        for layer in &self.star_layers {
            for s in &layer.stars {
                let x = self.wrap_x(s.x, layer.par);
                if x < -4.0 || x > w + 4.0 {
                    continue;
                }
                ctx.set_global_alpha(0.45 + 0.55 * (self.clock * s.tw + s.ph).sin().abs());
                ctx.begin_path();
                circle(ctx, x, s.y, s.r)?;
                ctx.fill();
            }
        }
        ctx.set_global_alpha(1.0);

        for p in &self.planets {
            self.draw_planet(ctx, p)?;
        }

        // golden storybook stars
        for s in &self.gold_stars {
            let x = self.wrap_x(s.x, 0.09);
            if x < -10.0 || x > w + 10.0 {
                continue;
            }
            let pulse = 0.8 + 0.2 * (self.clock * 2.0 + s.ph).sin();
            ctx.set_shadow_color(C::GOLD);
            ctx.set_shadow_blur(8.0);
            star_path(ctx, x, s.y, s.r * pulse, s.ph);
            ctx.set_fill_style_str(C::TITLE);
            ctx.fill();
            ctx.set_shadow_blur(0.0);
        }

        if let Some(s) = &self.shooting {
            let k = s.t;
            let x = s.x + s.vx * k;
            let y = s.y + s.vy * k;
            let a = 1.0 - k / s.life;
            let (tx, ty) = (x - s.vx * 0.18, y - s.vy * 0.18);
            let g = ctx.create_linear_gradient(x, y, tx, ty);
            g.add_color_stop(0.0, &format!("rgba(255,255,255,{a})"))?;
            g.add_color_stop(1.0, "rgba(255,255,255,0)")?;
            ctx.set_stroke_style_canvas_gradient(&g);
            ctx.set_line_width(2.0);
            ctx.begin_path();
            ctx.move_to(x, y);
            ctx.line_to(tx, ty);
            ctx.stroke();
        }
        Ok(())
    }

    fn ground_y(&self, x: f64) -> f64 {
        let t = x + self.distance;
        GROUND + (t * 0.02).sin() * 1.5 + (t * 0.007).sin() * 1.5
    }

    fn draw_ground(&self, ctx: &Ctx) -> Res {
        let w = self.w;
        let g = ctx.create_linear_gradient(0.0, GROUND, 0.0, H);
        g.add_color_stop(0.0, C::GROUND1)?;
        g.add_color_stop(0.35, C::GROUND2)?;
        g.add_color_stop(1.0, C::GROUND3)?;
        ctx.set_fill_style_canvas_gradient(&g);
        ctx.begin_path();
        ctx.move_to(0.0, H);
        let mut x = 0.0;
        while x <= w {
            ctx.line_to(x, self.ground_y(x));
            x += 8.0;
        }
        ctx.line_to(w, H);
        ctx.close_path();
        ctx.fill();

        ctx.set_stroke_style_str(C::GROUND_EDGE);
        ctx.set_line_width(1.5);
        ctx.begin_path();
        ctx.move_to(0.0, self.ground_y(0.0));
        let mut x = 8.0;
        while x <= w {
            ctx.line_to(x, self.ground_y(x));
            x += 8.0;
        }
        ctx.stroke();

        // craters and moon dust scrolling with the ground
        let spacing = 64.0;
        let shift = self.distance % spacing;
        let mut i = -1.0;
        while i < w / spacing + 2.0 {
            let x = i * spacing - shift;
            i += 1.0;
            let seed = ((self.distance + x) / spacing).floor() as i64;
            let y = GROUND + 12.0 + ((seed * 13) % 20) as f64;
            let rx = 5.0 + ((seed % 3) * 4) as f64;
            if seed % 2 == 0 {
                ctx.set_fill_style_str(C::CRATER);
                ctx.begin_path();
                ctx.ellipse(x, y, rx, rx * 0.35, 0.0, 0.0, TAU)?;
                ctx.fill();
                ctx.set_stroke_style_str(C::CRATER_RIM);
                ctx.set_line_width(1.0);
                ctx.begin_path();
                ctx.ellipse(x, y + 0.6, rx, rx * 0.35, 0.0, 0.1, PI - 0.1)?;
                ctx.stroke();
            } else {
                ctx.set_fill_style_str(C::CRATER_RIM);
                ctx.set_global_alpha(0.7);
                ctx.begin_path();
                circle(ctx, x, y, 1.2)?;
                circle(ctx, x + 9.0, y + 5.0, 0.9)?;
                ctx.fill();
                ctx.set_global_alpha(1.0);
            }
        }
        Ok(())
    }

    fn draw_obstacle(&self, ctx: &Ctx, ob: &Obstacle) -> Res {
        ctx.set_line_width(2.0);
        ctx.set_line_join("round");
        match &ob.kind {
            ObKind::Rocks(parts) => {
                for p in parts {
                    let cx = ob.x + p.dx + p.w / 2.0;
                    let cy = GROUND - p.h / 2.0;
                    smooth_blob(ctx, cx, cy, &p.pts);
                    ctx.set_fill_style_str(C::ROCK);
                    ctx.fill();
                    ctx.set_stroke_style_str(C::INK);
                    ctx.stroke();
                    ctx.set_fill_style_str(C::ROCK_SHADE);
                    for &(dx, dy, r) in &p.craters {
                        ctx.begin_path();
                        circle(ctx, cx + dx, cy + dy, r)?;
                        ctx.fill();
                    }
                    ctx.set_stroke_style_str(C::ROCK_LIGHT);
                    ctx.set_line_width(1.5);
                    ctx.begin_path();
                    ctx.arc(cx, cy, p.w.min(p.h) * 0.32, PI * 1.1, PI * 1.45)?;
                    ctx.stroke();
                    ctx.set_line_width(2.0);
                }
            }
            ObKind::Crystal { shards, hue: (light, dark) } => {
                ctx.set_shadow_color(light);
                ctx.set_shadow_blur(12.0);
                for s in shards {
                    let bx = ob.x + s.dx;
                    let t = s.tilt;
                    ctx.begin_path();
                    ctx.move_to(bx - s.w / 2.0, GROUND);
                    ctx.line_to(bx - s.w / 2.0 + t * 0.7, GROUND - s.h * 0.72);
                    ctx.line_to(bx + t, GROUND - s.h);
                    ctx.line_to(bx + s.w / 2.0 + t * 0.7, GROUND - s.h * 0.72);
                    ctx.line_to(bx + s.w / 2.0, GROUND);
                    ctx.close_path();
                    let g = ctx.create_linear_gradient(0.0, GROUND - s.h, 0.0, GROUND);
                    g.add_color_stop(0.0, light)?;
                    g.add_color_stop(1.0, dark)?;
                    ctx.set_fill_style_canvas_gradient(&g);
                    ctx.fill();
                    ctx.set_stroke_style_str(C::INK);
                    ctx.stroke();
                }
                ctx.set_shadow_blur(0.0);
                ctx.set_stroke_style_str("rgba(255,255,255,0.7)");
                ctx.set_line_width(1.2);
                for s in shards {
                    let bx = ob.x + s.dx;
                    ctx.begin_path();
                    ctx.move_to(bx + s.tilt, GROUND - s.h + 4.0);
                    ctx.line_to(bx + s.tilt * 0.4, GROUND - 4.0);
                    ctx.stroke();
                }
            }
            ObKind::Comet { spin } => {
                let cx = ob.x + 17.0;
                let cy = ob.y + ob.h / 2.0;
                let trail = ctx.create_linear_gradient(cx, 0.0, cx + 56.0, 0.0);
                trail.add_color_stop(0.0, "rgba(255,210,141,0.95)")?;
                trail.add_color_stop(0.5, "rgba(255,143,200,0.5)")?;
                trail.add_color_stop(1.0, "rgba(143,107,255,0)")?;
                ctx.set_fill_style_canvas_gradient(&trail);
                ctx.begin_path();
                ctx.move_to(cx, cy - 9.0);
                ctx.quadratic_curve_to(cx + 34.0, cy - 7.0, cx + 58.0, cy);
                ctx.quadratic_curve_to(cx + 34.0, cy + 7.0, cx, cy + 9.0);
                ctx.fill();
                ctx.set_shadow_color(C::GOLD);
                ctx.set_shadow_blur(14.0);
                star_path(ctx, cx, cy, 13.0, spin.sin() * 0.25);
                ctx.set_fill_style_str(C::TITLE);
                ctx.fill();
                ctx.set_shadow_blur(0.0);
                ctx.set_stroke_style_str(C::TITLE_SHADOW);
                ctx.set_line_width(2.0);
                ctx.stroke();
                // a little face, looking ahead
                ctx.set_fill_style_str(C::TITLE_SHADOW);
                ctx.begin_path();
                circle(ctx, cx - 4.0, cy - 1.0, 1.5)?;
                circle(ctx, cx + 2.0, cy - 1.0, 1.5)?;
                ctx.fill();
            }
        }
        Ok(())
    }

    // Carla's Navigator's Map: markers for things still off screen
    fn draw_navigator_markers(&self, ctx: &Ctx) -> Res {
        if !self.chr().map || self.mode != Mode::Running {
            return Ok(());
        }
        let w = self.w;
        for ob in &self.obstacles {
            if ob.x <= w - 4.0 {
                continue;
            }
            let closeness = 1.0 - ((ob.x - w) / SPAWN_AHEAD).min(1.0);
            let y = ob.y + ob.h / 2.0;
            ctx.set_global_alpha(0.35 + 0.65 * closeness);
            ctx.set_fill_style_str(C::NAVIGATOR);
            ctx.begin_path();
            ctx.move_to(w - 22.0, y);
            ctx.line_to(w - 8.0, y - 8.0);
            ctx.line_to(w - 8.0, y + 8.0);
            ctx.close_path();
            ctx.fill();
            ctx.set_fill_style_str(C::INK);
            ctx.set_font(&font(10.0));
            ctx.set_text_align("center");
            ctx.fill_text("!", w - 12.0, y + 3.5)?;
        }
        ctx.set_global_alpha(1.0);
        Ok(())
    }

    fn draw_pickups(&self, ctx: &Ctx) {
        ctx.set_shadow_color(C::GOLD);
        ctx.set_shadow_blur(10.0);
        for p in &self.pickups {
            let y = p.y + (self.clock * 5.0 + p.phase).sin() * 3.0;
            star_path(ctx, p.x, y, 9.0, (self.clock * 3.0 + p.phase).sin() * 0.3);
            ctx.set_fill_style_str(C::GOLD);
            ctx.fill();
            ctx.set_stroke_style_str(C::TITLE_SHADOW);
            ctx.set_line_width(1.6);
            ctx.stroke();
        }
        ctx.set_shadow_blur(0.0);
    }

    fn draw_player(&mut self, ctx: &Ctx) -> Res {
        let (base_w, _, h, sx, sy) = self.player_metrics();
        let cx = PLAYER_X + base_w / 2.0;
        let clock = self.clock;
        let py = self.player.y;
        let mut rot = 0.0;
        let mut bob = 0.0;

        if self.mode == Mode::Over {
            rot = -0.22;
        } else if self.active.kind == Some(SpecialKind::Rocket) {
            rot = -0.12 + (clock * 20.0).sin() * 0.03;
        } else if !self.player.on_ground {
            rot = if self.player.vy < 0.0 { -0.1 } else { 0.06 };
        } else if self.mode == Mode::Running && !self.player.ducking {
            let step = (clock * 16.0).sin();
            bob = step.abs() * 3.0;
            rot = step * 0.045;
        }

        // soft shadow shrinks as the character rises
        let lift = (GROUND - py).max(0.0);
        let shadow_scale = (1.0 - lift / 170.0).max(0.3);
        ctx.set_fill_style_str("rgba(36, 29, 74, 0.35)");
        ctx.begin_path();
        ctx.ellipse(cx + 4.0, GROUND + 2.0, base_w * 0.36 * shadow_scale * sx, 4.0 * shadow_scale, 0.0, 0.0, TAU)?;
        ctx.fill();

        let mid_y = py - h / 2.0;

        match self.active.kind {
            Some(SpecialKind::Brave) => {
                let pr = 46.0 + (clock * 10.0).sin() * 4.0;
                let g = ctx.create_radial_gradient(cx, mid_y, 4.0, cx, mid_y, pr)?;
                g.add_color_stop(0.0, "rgba(255,226,140,0.75)")?;
                g.add_color_stop(0.6, "rgba(255,190,90,0.3)")?;
                g.add_color_stop(1.0, "rgba(255,190,90,0)")?;
                ctx.set_fill_style_canvas_gradient(&g);
                ctx.begin_path();
                circle(ctx, cx, mid_y, pr)?;
                ctx.fill();
                if self.rng.chance(0.5) {
                    let particle = Particle {
                        x: cx + self.rng.range(-20.0, 10.0),
                        y: mid_y + self.rng.range(-25.0, 25.0),
                        vx: self.rng.range(-200.0, -120.0),
                        vy: self.rng.range(-30.0, 30.0),
                        life: 0.4,
                        max: 0.4,
                        color: C::TITLE,
                        star: true,
                    };
                    self.particles.push(particle);
                }
            }
            Some(SpecialKind::Calm) => {
                ctx.set_stroke_style_str(C::NAVIGATOR);
                ctx.set_line_width(2.0);
                for i in 0..2 {
                    let t = (clock * 0.9 + i as f64 * 0.5) % 1.0;
                    ctx.set_global_alpha(1.0 - t);
                    ctx.begin_path();
                    ctx.ellipse(cx, mid_y, 20.0 + t * 50.0, 14.0 + t * 34.0, 0.0, 0.0, TAU)?;
                    ctx.stroke();
                }
                ctx.set_global_alpha(1.0);
            }
            Some(SpecialKind::Rocket) => {
                // jet flame under the rocket pack
                let fx = cx - base_w * 0.28;
                let fy = py - h * 0.3;
                let fl = 16.0 + self.rng.next() * 10.0;
                ctx.set_fill_style_str("#ffb347");
                ctx.begin_path();
                ctx.move_to(fx - 6.0, fy);
                ctx.line_to(fx + 6.0, fy);
                ctx.line_to(fx - 4.0, fy + fl);
                ctx.close_path();
                ctx.fill();
                ctx.set_fill_style_str(C::TITLE);
                ctx.begin_path();
                ctx.move_to(fx - 3.0, fy);
                ctx.line_to(fx + 3.0, fy);
                ctx.line_to(fx - 2.0, fy + fl * 0.6);
                ctx.close_path();
                ctx.fill();
                ctx.set_fill_style_str("#8fa3c8");
                ctx.set_stroke_style_str(C::INK);
                ctx.set_line_width(1.5);
                round_rect(ctx, fx - 8.0, fy - 22.0, 14.0, 24.0, 5.0)?;
                ctx.fill();
                ctx.stroke();
            }
            None => {}
        }

        // flicker while the post-special grace period runs out
        if self.grace > 0.0 && (clock * 14.0).floor() as i64 % 2 == 0 {
            ctx.set_global_alpha(0.45);
        }

        let c = self.chr();
        let full_h = h / sy;
        ctx.save();
        ctx.translate(cx, py - bob + 1.0)?;
        ctx.rotate(rot)?;
        ctx.scale(sx * if c.flip { -1.0 } else { 1.0 }, sy)?;
        match &self.sprites[self.sel] {
            Some(img) if img.complete() && img.natural_width() > 0 => {
                ctx.draw_image_with_html_image_element_and_dw_and_dh(img, -base_w / 2.0, -full_h, base_w, full_h)?;
            }
            _ => {
                ctx.set_fill_style_str(c.accent);
                ctx.begin_path();
                ctx.ellipse(0.0, -full_h / 2.0, base_w / 2.4, full_h / 2.2, 0.0, 0.0, TAU)?;
                ctx.fill();
            }
        }
        ctx.restore();
        ctx.set_global_alpha(1.0);

        if self.mode == Mode::Over {
            // dizzy stars
            for i in 0..3 {
                let a = clock * 4.0 + (i as f64 * TAU) / 3.0;
                star_path(ctx, cx + a.cos() * 18.0, py - h - 6.0 + a.sin() * 5.0, 5.0, 0.0);
                ctx.set_fill_style_str(C::TITLE);
                ctx.fill();
                ctx.set_stroke_style_str(C::TITLE_SHADOW);
                ctx.set_line_width(1.2);
                ctx.stroke();
            }
        }
        Ok(())
    }

    fn draw_effects(&self, ctx: &Ctx) -> Res {
        for p in &self.particles {
            ctx.set_global_alpha((p.life / p.max).max(0.0));
            ctx.set_fill_style_str(p.color);
            if p.star {
                star_path(ctx, p.x, p.y, 4.0, 0.0);
            } else {
                ctx.begin_path();
                circle(ctx, p.x, p.y, 2.0)?;
            }
            ctx.fill();
        }
        ctx.set_global_alpha(1.0);
        ctx.set_font(&font(13.0));
        ctx.set_text_align("center");
        ctx.set_fill_style_str(C::TITLE);
        for p in &self.popups {
            ctx.set_global_alpha((p.life / 0.8).max(0.0));
            ctx.fill_text(&p.text, p.x, p.y)?;
        }
        ctx.set_global_alpha(1.0);
        Ok(())
    }

    fn draw_hud(&self, ctx: &Ctx) -> Res {
        let w = self.w;
        let clock = self.clock;
        ctx.set_text_baseline("alphabetic");

        // score
        ctx.set_font(&font(15.0));
        ctx.set_text_align("right");
        let flashing = clock < self.flash_until && (clock * 8.0).floor() as i64 % 2 == 0;
        ctx.set_fill_style_str(C::TEXT);
        if !flashing {
            ctx.fill_text(&pad(self.score), w - 16.0, 24.0)?;
        }
        ctx.set_global_alpha(0.6);
        ctx.fill_text(&format!("HI {}", pad(self.hi_score)), w - 72.0, 24.0)?;
        ctx.set_global_alpha(1.0);

        // special meter
        let cost = self.chr().special_cost;
        let name = self.chr().special_name;
        let active = self.active.kind.is_some();
        let ready = self.charge >= cost;
        ctx.set_text_align("right");
        ctx.set_font(&font(12.0));
        ctx.set_fill_style_str(if active || ready { C::TITLE } else { C::TEXT });
        let label = if active {
            format!("{name}!")
        } else if ready {
            format!("{name} ready — {}", if self.touch { "tap ★" } else { "press X" })
        } else {
            name.to_string()
        };
        ctx.fill_text(&label, w - 16.0, 46.0)?;

        let pip = if cost > 7 { 10.0 } else { 13.0 };
        let lit = if active {
            (self.active.t / self.active.max * cost as f64).ceil() as u32
        } else {
            self.charge
        };
        for i in 0..cost {
            let x = w - 22.0 - (cost - 1 - i) as f64 * pip;
            star_path(ctx, x, 60.0, pip * 0.42, 0.0);
            if i < lit {
                let blink = ready && (clock * 4.0).floor() as i64 % 2 == 1;
                ctx.set_fill_style_str(if blink { "#fff3c4" } else { C::TITLE });
                ctx.fill();
            } else {
                ctx.set_stroke_style_str("rgba(255,246,227,0.45)");
                ctx.set_line_width(1.0);
                ctx.stroke();
            }
        }

        if self.zone_banner_t > 0.0 {
            ctx.set_global_alpha(self.zone_banner_t.min(1.0));
            ctx.set_text_align("center");
            ctx.set_font(&font(16.0));
            ctx.set_fill_style_str(C::TEXT);
            ctx.fill_text(&format!("✦ entering the {} ✦", ZONES[self.zone_index].name), w / 2.0, 92.0)?;
            ctx.set_global_alpha(1.0);
        }
        Ok(())
    }

    fn title_text(ctx: &Ctx, text: &str, x: f64, y: f64, size: f64) -> Res {
        ctx.set_font(&font(size));
        ctx.set_text_align("center");
        ctx.set_fill_style_str(C::TITLE_SHADOW);
        ctx.fill_text(text, x + 2.0, y + 3.0)?;
        ctx.set_fill_style_str(C::TITLE);
        ctx.fill_text(text, x, y)
    }

    fn body_text(ctx: &Ctx, text: &str, x: f64, y: f64, size: f64, alpha: f64) -> Res {
        ctx.set_font(&font(size));
        ctx.set_text_align("center");
        ctx.set_global_alpha(alpha);
        ctx.set_fill_style_str(C::TEXT);
        ctx.fill_text(text, x, y)?;
        ctx.set_global_alpha(1.0);
        Ok(())
    }

    fn draw_overlay(&self, ctx: &Ctx) -> Res {
        if self.mode == Mode::Running {
            return Ok(());
        }
        let w = self.w;
        let cx = w / 2.0 + 30.0;
        let pw = (w - 150.0).min(440.0);
        ctx.set_fill_style_str("rgba(18,15,51,0.6)");
        let ph = if self.mode == Mode::Paused { 88.0 } else { 156.0 };
        round_rect(ctx, cx - pw / 2.0, 52.0, pw, ph, 18.0)?;
        ctx.fill();

        match self.mode {
            Mode::Over => {
                Self::title_text(ctx, "OH NO!", cx, 90.0, 34.0)?;
                let msg = format!("{} bumped into {}", self.chr().name, self.bumped_into);
                Self::body_text(ctx, &msg, cx, 120.0, 15.0, 1.0)?;
                let again = if self.touch { "tap to try again" } else { "space or click to try again" };
                Self::body_text(ctx, again, cx, 146.0, 13.0, 0.75)?;
                let b = self.friends_button();
                ctx.set_fill_style_str("rgba(255,246,227,0.14)");
                round_rect(ctx, b.x, b.y, b.w, b.h, b.h / 2.0)?;
                ctx.fill();
                ctx.set_stroke_style_str("rgba(255,246,227,0.5)");
                ctx.set_line_width(1.0);
                ctx.stroke();
                let label = if self.touch { "◀ choose a friend" } else { "◀ choose a friend (← →)" };
                Self::body_text(ctx, label, b.x + b.w / 2.0, b.y + 18.0, 13.0, 1.0)?;
            }
            Mode::Paused => {
                Self::title_text(ctx, "PAUSED", cx, 94.0, 30.0)?;
                Self::body_text(ctx, "tap or press space to keep going", cx, 124.0, 14.0, 1.0)?;
            }
            Mode::Ready | Mode::Running => {}
        }
        Ok(())
    }

    // ---------- intro / character select ----------

    fn overlay_center(&self) -> f64 {
        self.w / 2.0 + 30.0
    }

    /// The crash screen's "choose a friend" button (also used to hit-test taps).
    pub(crate) fn friends_button(&self) -> Rect {
        let w = if self.touch { 170.0 } else { 210.0 };
        Rect { x: self.overlay_center() - w / 2.0, y: 164.0, w, h: 28.0 }
    }

    /// Where friend `i`'s card sits on the intro screen (also used to hit-test taps).
    pub(crate) fn chip_rect(&self, i: usize) -> Rect {
        let narrow = self.w < 600.0;
        let (gap, cw) = if narrow { (124.0, 112.0) } else { (150.0, 128.0) };
        let cx = self.w / 2.0 + (i as f64 - 1.0) * gap;
        Rect { x: cx - cw / 2.0, y: 50.0, w: cw, h: 96.0 }
    }

    /// How to play, worded for touch or keys.
    pub(crate) fn controls_text(&self) -> &'static str {
        if self.touch {
            "tap a friend to choose, tap again to start · tap to jump (hold for higher) · swipe down to duck · ★ button for your special"
        } else {
            "← → choose a friend · space to start · ↑ jump (hold for higher) · ↓ duck · X special · P pause"
        }
    }

    fn draw_intro(&self, ctx: &Ctx) -> Res {
        let w = self.w;
        let narrow = w < 600.0;
        ctx.set_fill_style_str("rgba(18,15,51,0.72)");
        round_rect(ctx, 8.0, 8.0, w - 16.0, H - 16.0, 18.0)?;
        ctx.fill();
        ctx.set_text_baseline("alphabetic");

        Self::title_text(ctx, "CROSBY'S COSMIC DASH", w / 2.0, 38.0, if narrow { 22.0 } else { 28.0 })?;
        if self.hi_score > 0.0 {
            ctx.set_font(&font(if narrow { 14.0 } else { 11.0 }));
            ctx.set_text_align("right");
            ctx.set_global_alpha(0.7);
            ctx.set_fill_style_str(C::TEXT);
            ctx.fill_text(&format!("HI {}", pad(self.hi_score)), w - 22.0, 30.0)?;
            ctx.set_global_alpha(1.0);
        }

        for (i, c) in CHARACTERS.iter().enumerate() {
            self.draw_chip(ctx, i, c)?;
        }

        // Scaled down to a phone's width, paragraphs here would be too small to read, so the
        // page shows the friend's moves and the controls as text below the game instead.
        if narrow {
            let hint = if self.touch { "tap a friend to choose · tap again to start" } else { "← → choose a friend · space to start" };
            ctx.set_font(&font(17.0));
            let lines = wrap(ctx, hint, w - 48.0, w - 48.0)?;
            ctx.set_text_align("center");
            ctx.set_fill_style_str(C::GOLD);
            let mut y = 196.0;
            for line in &lines {
                ctx.fill_text(line, w / 2.0, y)?;
                y += 22.0;
            }
            return Self::body_text(ctx, "moves and controls are below the game ↓", w / 2.0, y + 8.0, 15.0, 0.75);
        }

        // the chosen friend's story and moves
        let c = self.chr();
        let block_w = (w - 56.0).min(600.0);
        let left = (w - block_w) / 2.0;
        let mut y = 164.0;
        let hero = if c.hero { " · main hero" } else { "" };
        Self::segments_centered(ctx, &[(c.name, C::TITLE), (&format!(" · {}{hero}", c.role.to_lowercase()), C::TEXT)], w / 2.0, y, 13.0)?;
        y += 17.0;
        let size = if narrow { 11.0 } else { 12.0 };
        let passive = format!("Passive · {}:", c.passive_name);
        y = Self::labeled_paragraph(ctx, &passive, c.passive_text, left, y, block_w, size)?;
        let special = format!("Special · {} ({} ★):", c.special_name, c.special_cost);
        Self::labeled_paragraph(ctx, &special, c.special_text, left, y, block_w, size)?;

        // controls, anchored to the bottom of the panel
        ctx.set_font(&font(11.0));
        let lines = wrap(ctx, self.controls_text(), w - 48.0, w - 48.0)?;
        ctx.set_text_align("center");
        ctx.set_fill_style_str(C::GOLD);
        let mut y = H - 20.0 - (lines.len() as f64 - 1.0) * 14.0;
        for line in &lines {
            ctx.fill_text(line, w / 2.0, y)?;
            y += 14.0;
        }
        Ok(())
    }

    fn draw_chip(&self, ctx: &Ctx, i: usize, c: &Character) -> Res {
        let r = self.chip_rect(i);
        let chosen = i == self.sel;
        round_rect(ctx, r.x, r.y, r.w, r.h, 14.0)?;
        if chosen {
            ctx.set_global_alpha(0.28);
            ctx.set_fill_style_str(c.accent);
            ctx.fill();
            ctx.set_global_alpha(1.0);
            ctx.set_stroke_style_str(c.accent);
            ctx.set_line_width(2.5);
            ctx.stroke();
        } else {
            ctx.set_fill_style_str("rgba(255,246,227,0.07)");
            ctx.fill();
        }

        // portrait, standing on a little glow
        let cx = r.x + r.w / 2.0;
        let foot = r.y + 60.0;
        let h = 50.0;
        let pw = h * c.aspect;
        ctx.set_fill_style_str(if chosen { "rgba(255,226,140,0.35)" } else { "rgba(36,29,74,0.45)" });
        ctx.begin_path();
        ctx.ellipse(cx, foot, pw * 0.4, 3.5, 0.0, 0.0, TAU)?;
        ctx.fill();
        if !chosen {
            ctx.set_global_alpha(0.7);
        }
        match &self.sprites[i] {
            Some(img) if img.complete() && img.natural_width() > 0 => {
                ctx.save();
                ctx.translate(cx, foot)?;
                ctx.scale(if c.flip { -1.0 } else { 1.0 }, 1.0)?;
                ctx.draw_image_with_html_image_element_and_dw_and_dh(img, -pw / 2.0, -h, pw, h)?;
                ctx.restore();
            }
            _ => {
                ctx.set_fill_style_str(c.accent);
                ctx.begin_path();
                ctx.ellipse(cx, foot - h / 2.0, pw / 2.4, h / 2.2, 0.0, 0.0, TAU)?;
                ctx.fill();
            }
        }
        ctx.set_global_alpha(1.0);

        ctx.set_font(&font(if self.w < 600.0 { 15.0 } else { 13.0 }));
        ctx.set_text_align("center");
        ctx.set_fill_style_str(if chosen { C::TITLE } else { C::TEXT });
        ctx.fill_text(c.name, cx, r.y + 77.0)?;

        // power rating, out of 5
        for k in 0..5 {
            let sx = cx + (k as f64 - 2.0) * 10.0;
            star_path(ctx, sx, r.y + 87.0, 4.0, 0.0);
            if k < c.power {
                ctx.set_fill_style_str(C::GOLD);
                ctx.fill();
            } else {
                ctx.set_stroke_style_str("rgba(255,246,227,0.4)");
                ctx.set_line_width(1.0);
                ctx.stroke();
            }
        }
        Ok(())
    }

    /// Draw differently colored pieces of text as one centered line.
    fn segments_centered(ctx: &Ctx, parts: &[(&str, &str)], cx: f64, y: f64, size: f64) -> Res {
        ctx.set_font(&font(size));
        ctx.set_text_align("left");
        let widths = parts.iter().map(|(t, _)| ctx.measure_text(t).map(|m| m.width())).collect::<Result<Vec<_>, _>>()?;
        let mut x = cx - widths.iter().sum::<f64>() / 2.0;
        for ((text, color), width) in parts.iter().zip(widths) {
            ctx.set_fill_style_str(color);
            ctx.fill_text(text, x, y)?;
            x += width;
        }
        Ok(())
    }

    /// A gold label followed by wrapped body text; returns the y for whatever comes next.
    fn labeled_paragraph(ctx: &Ctx, label: &str, body: &str, left: f64, y: f64, width: f64, size: f64) -> Result<f64, JsValue> {
        let line_h = size + 3.0;
        ctx.set_font(&font(size));
        ctx.set_text_align("left");
        ctx.set_fill_style_str(C::GOLD);
        ctx.fill_text(label, left, y)?;
        let indent = ctx.measure_text(label)?.width() + size * 0.4;
        let lines = wrap(ctx, body, width - indent, width)?;
        ctx.set_fill_style_str(C::TEXT);
        let mut y = y;
        for (i, line) in lines.iter().enumerate() {
            let x = if i == 0 { left + indent } else { left };
            ctx.fill_text(line, x, y)?;
            y += line_h;
        }
        Ok(y + 2.0)
    }
}

/// Split `text` into lines that fit, measured in the context's current font. The first line
/// can be narrower (when it follows a label).
fn wrap(ctx: &Ctx, text: &str, first_width: f64, width: f64) -> Result<Vec<String>, JsValue> {
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in text.split(' ') {
        let candidate = if line.is_empty() { word.to_string() } else { format!("{line} {word}") };
        let limit = if lines.is_empty() { first_width } else { width };
        if !line.is_empty() && ctx.measure_text(&candidate)?.width() > limit {
            lines.push(std::mem::replace(&mut line, word.to_string()));
        } else {
            line = candidate;
        }
    }
    if !line.is_empty() {
        lines.push(line);
    }
    Ok(lines)
}
