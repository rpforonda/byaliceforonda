//! Crosby's Cosmic Dash — the game engine.
//!
//! Endless runner in the spirit of Chrome's offline dino game, set in the starry world of
//! Crosby's Cosmic Adventure. Each friend has a passive perk and a special move charged by
//! collecting stars — Crosby's are the strongest.
//!
//! This crate owns the whole simulation (physics, spawning, collisions, specials, scoring)
//! and draws every frame onto a Canvas 2D context. The Svelte component (`CosmicDash.svelte`)
//! owns everything around it: the character table, sprites, input events, the
//! `requestAnimationFrame` loop, `localStorage`, and screen-reader announcements. It drives
//! the engine through the `Game` methods below and reads back a few values for its UI.

use std::f64::consts::{PI, TAU};

use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

mod draw;

// Palette drawn from the book cover and the nighttime spreads.
mod color {
    pub const SKY_TOP: &str = "#120f33";
    pub const SKY_MID: &str = "#2b2466";
    pub const SKY_LOW: &str = "#4f3a8c";
    pub const TITLE: &str = "#f3d36f";
    pub const TITLE_SHADOW: &str = "#7c2b28";
    pub const INK: &str = "#241d4a";
    pub const TEXT: &str = "#fff6e3";
    pub const GOLD: &str = "#ffd28d";
    pub const GROUND1: &str = "#a99ad8";
    pub const GROUND2: &str = "#7d6cb8";
    pub const GROUND3: &str = "#4a3d86";
    pub const GROUND_EDGE: &str = "#e9e1ff";
    pub const CRATER: &str = "#6a5aa6";
    pub const CRATER_RIM: &str = "#c4b8ee";
    pub const ROCK: &str = "#8e83bd";
    pub const ROCK_SHADE: &str = "#6f63a3";
    pub const ROCK_LIGHT: &str = "#c9bff0";
    pub const NAVIGATOR: &str = "#5fe0d0";
}
use color as C;

struct Zone {
    name: &'static str,
    a: [f64; 3],
    b: [f64; 3],
}

// Each zone tints the nebulae; a new zone every ZONE_EVERY points.
const ZONES: [Zone; 4] = [
    Zone { name: "Purple Nebula", a: [255.0, 143.0, 200.0], b: [143.0, 107.0, 255.0] },
    Zone { name: "Teal Galaxy", a: [95.0, 224.0, 208.0], b: [58.0, 123.0, 213.0] },
    Zone { name: "Rosy Starfield", a: [255.0, 180.0, 160.0], b: [255.0, 111.0, 160.0] },
    Zone { name: "Golden Comet Belt", a: [255.0, 210.0, 141.0], b: [240.0, 154.0, 88.0] },
];
const ZONE_EVERY: f64 = 500.0;

const H: f64 = 280.0;
const GROUND: f64 = 236.0;
const PLAYER_X: f64 = 48.0;
const GRAVITY: f64 = 2600.0;
const JUMP_V: f64 = -700.0;
const AIR_JUMP_V: f64 = -640.0;
const FAST_FALL: f64 = 2.4;
const DUCK_SX: f64 = 1.18;
const DUCK_SY: f64 = 0.6;
const START_SPEED: f64 = 340.0;
const MAX_SPEED: f64 = 760.0;
const ACCEL: f64 = 9.0;
const SPAWN_AHEAD: f64 = 260.0;
const GRACE: f64 = 0.9;
const ROCKET_Y: f64 = GROUND - 128.0;
/// After a crash, ignore restart presses this long so a held key doesn't skip the screen.
const OVER_LOCK_MS: f64 = 450.0;
/// How long the crash burst keeps animating after game over.
const OVER_TAIL_MS: f64 = 700.0;

// ---------- small helpers ----------

/// xorshift64* — plenty for scenery and spawn rolls, and avoids a JS call per random number.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed | 1)
    }
    fn next(&mut self) -> f64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        (x.wrapping_mul(0x2545_f491_4f6c_dd1d) >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, a: f64, b: f64) -> f64 {
        a + self.next() * (b - a)
    }
    fn chance(&mut self, p: f64) -> bool {
        self.next() < p
    }
}

#[derive(Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl Rect {
    fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h
    }
    fn overlaps(&self, o: &Rect) -> bool {
        self.x < o.x + o.w && self.x + self.w > o.x && self.y < o.y + o.h && self.y + self.h > o.y
    }
}

fn rgba(c: [f64; 3], a: f64) -> String {
    format!("rgba({},{},{},{})", c[0] as i32, c[1] as i32, c[2] as i32, a)
}

fn approach(cur: [f64; 3], target: [f64; 3], k: f64) -> [f64; 3] {
    [0, 1, 2].map(|i| cur[i] + (target[i] - cur[i]) * k)
}

// ---------- characters ----------

#[derive(Clone, Copy, PartialEq)]
enum SpecialKind {
    Brave,
    Calm,
    Rocket,
}

/// One playable friend: how they look and move, and what the intro screen says about them.
/// Each has a passive perk and a special move charged by collecting stars.
struct Character {
    id: &'static str,
    name: &'static str,
    role: &'static str,
    hero: bool,
    power: u32, // out of 5, shown on the intro screen
    aspect: f64, // sprite width / height
    height: f64,
    flip: bool,
    hit: [f64; 4], // l, r, t, b hitbox insets as fractions of the sprite box
    accent: &'static str,
    jumps: u32,
    hold_time: f64,
    hold_gravity: f64,
    magnet: f64,
    map: bool,
    passive_name: &'static str,
    passive_text: &'static str,
    special_kind: SpecialKind,
    special_name: &'static str,
    special_cost: u32,
    special_duration: f64,
    special_text: &'static str,
}

// Crosby, the main hero, is deliberately the strongest.
const CHARACTERS: [Character; 3] = [
    Character {
        id: "crosby",
        name: "Crosby",
        role: "Gentle dreamer",
        hero: true,
        power: 5,
        aspect: 288.0 / 240.0,
        height: 72.0,
        flip: false,
        hit: [0.3, 0.1, 0.1, 0.03],
        accent: "#7dbd55",
        jumps: 2,
        hold_time: 0.17,
        hold_gravity: 0.55,
        magnet: 0.0,
        map: false,
        passive_name: "Starlight Double Jump",
        passive_text: "Leap again in mid-air on a burst of starlight.",
        special_kind: SpecialKind::Brave,
        special_name: "Brave Heart",
        special_cost: 6,
        special_duration: 5.0,
        special_text: "5 seconds of glowing courage: smash through anything, pull in every star and score double.",
    },
    Character {
        id: "carla",
        name: "Carla",
        role: "Fearless navigator",
        hero: false,
        power: 3,
        aspect: 172.0 / 240.0,
        height: 70.0,
        flip: true,
        hit: [0.14, 0.14, 0.06, 0.03],
        accent: "#d68a4c",
        jumps: 1,
        hold_time: 0.17,
        hold_gravity: 0.55,
        magnet: 80.0,
        map: true,
        passive_name: "Navigator's Map",
        passive_text: "Spots what's coming before it's on screen, and nearby stars drift her way.",
        special_kind: SpecialKind::Calm,
        special_name: "Calm Compass",
        special_cost: 9,
        special_duration: 4.0,
        special_text: "She sets a steady pace: the whole galaxy slows down for 4 seconds.",
    },
    Character {
        id: "tank",
        name: "Tank",
        role: "Inventor and tinkerer",
        hero: false,
        power: 4,
        aspect: 211.0 / 240.0,
        height: 74.0,
        flip: false,
        hit: [0.1, 0.25, 0.05, 0.03],
        accent: "#e8a24a",
        jumps: 1,
        hold_time: 0.2,
        hold_gravity: 0.45,
        magnet: 0.0,
        map: false,
        passive_name: "Spring Gadget",
        passive_text: "Hold jump to bounce extra high.",
        special_kind: SpecialKind::Rocket,
        special_name: "Rocket Pack",
        special_cost: 9,
        special_duration: 3.0,
        special_text: "Blast off and cruise safely over everything for 3 seconds.",
    },
];

// ---------- world entities ----------

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Ready,
    Running,
    Paused,
    Over,
}

#[derive(Clone, Copy)]
struct Active {
    kind: Option<SpecialKind>,
    t: f64,
    max: f64,
}

const INACTIVE: Active = Active { kind: None, t: 0.0, max: 0.0 };

struct Player {
    y: f64,
    vy: f64,
    on_ground: bool,
    hold_t: f64,
    held: bool,
    ducking: bool,
    jumps_used: u32,
}

impl Player {
    fn new() -> Self {
        Player { y: GROUND, vy: 0.0, on_ground: true, hold_t: 0.0, held: false, ducking: false, jumps_used: 0 }
    }
}

struct RockPart {
    dx: f64,
    w: f64,
    h: f64,
    pts: Vec<(f64, f64)>,
    craters: [(f64, f64, f64); 2],
}

struct Shard {
    dx: f64,
    w: f64,
    h: f64,
    tilt: f64,
}

enum ObKind {
    Rocks(Vec<RockPart>),
    Crystal { shards: Vec<Shard>, hue: (&'static str, &'static str) },
    Comet { spin: f64 },
}

struct Obstacle {
    kind: ObKind,
    label: &'static str,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    hit: Rect, // relative to (x, y)
    extra: f64, // own speed on top of the world's (comets fly at you)
    smashed: bool,
}

struct Pickup {
    x: f64,
    y: f64,
    phase: f64,
}

struct Particle {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    life: f64,
    max: f64,
    color: &'static str,
    star: bool,
}

struct Popup {
    x: f64,
    y: f64,
    life: f64,
    text: String,
}

struct BgStar {
    x: f64,
    y: f64,
    r: f64,
    tw: f64,
    ph: f64,
}

struct StarLayer {
    par: f64,
    stars: Vec<BgStar>,
}

struct GoldStar {
    x: f64,
    y: f64,
    r: f64,
    ph: f64,
}

struct Nebula {
    x: f64,
    y: f64,
    r: f64,
    use_a: bool,
}

#[derive(Clone, Copy)]
enum PlanetKind {
    Ringed,
    Moon,
    Blue,
}

struct Planet {
    x: f64,
    y: f64,
    r: f64,
    kind: PlanetKind,
}

struct Shooting {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    t: f64,
    life: f64,
}

// ---------- the game ----------

#[wasm_bindgen]
pub struct Game {
    ctx: Option<CanvasRenderingContext2d>,
    w: f64,
    sel: usize, // index into CHARACTERS
    sprites: [Option<HtmlImageElement>; 3],
    touch: bool, // last input was touch: word the on-screen hints for it
    rng: Rng,

    mode: Mode,
    speed: f64,
    score: f64,
    hi_score: f64,
    last_score: f64,
    last_is_best: bool,
    distance: f64,
    clock: f64,
    last_time: f64,
    next_spawn: f64,
    over_lock_until: f64,
    over_tail_until: f64,
    flash_until: f64,
    last_milestone: i64,
    bumped_into: &'static str,
    charge: u32,
    active: Active,
    grace: f64,
    zone_index: usize,
    zone_banner_t: f64,
    neb_a: [f64; 3],
    neb_b: [f64; 3],
    player: Player,

    obstacles: Vec<Obstacle>,
    pickups: Vec<Pickup>,
    particles: Vec<Particle>,
    popups: Vec<Popup>,
    star_layers: Vec<StarLayer>,
    gold_stars: Vec<GoldStar>,
    nebulae: Vec<Nebula>,
    planets: Vec<Planet>,
    shooting: Option<Shooting>,
    next_shoot: f64,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        let seed = (js_sys::Math::random() * (1u64 << 53) as f64) as u64;
        let mut game = Game {
            ctx: None,
            w: 720.0,
            sel: 0,
            sprites: [None, None, None],
            touch: false,
            rng: Rng::new(seed),
            mode: Mode::Ready,
            speed: START_SPEED,
            score: 0.0,
            hi_score: 0.0,
            last_score: 0.0,
            last_is_best: false,
            distance: 0.0,
            clock: 0.0,
            last_time: 0.0,
            next_spawn: 0.0,
            over_lock_until: 0.0,
            over_tail_until: 0.0,
            flash_until: 0.0,
            last_milestone: 0,
            bumped_into: "",
            charge: 0,
            active: INACTIVE,
            grace: 0.0,
            zone_index: 0,
            zone_banner_t: 0.0,
            neb_a: ZONES[0].a,
            neb_b: ZONES[0].b,
            player: Player::new(),
            obstacles: Vec::new(),
            pickups: Vec::new(),
            particles: Vec::new(),
            popups: Vec::new(),
            star_layers: Vec::new(),
            gold_stars: Vec::new(),
            nebulae: Vec::new(),
            planets: Vec::new(),
            shooting: None,
            next_shoot: 3.0,
        };
        game.reset();
        game
    }

    /// Point the engine at a (resized) canvas. `width` is the logical width the world is laid
    /// out in (480 or 720; the height is always 280) and `scale` maps it to device pixels.
    pub fn set_context(&mut self, ctx: CanvasRenderingContext2d, width: f64, scale: f64) -> Result<(), JsValue> {
        ctx.set_transform(scale, 0.0, 0.0, scale, 0.0, 0.0)?;
        self.ctx = Some(ctx);
        self.w = width;
        self.draw();
        Ok(())
    }

    /// Hand over a character's loaded sprite (`id` is "crosby", "carla" or "tank").
    pub fn set_sprite(&mut self, id: &str, sprite: HtmlImageElement) {
        if let Some(i) = CHARACTERS.iter().position(|c| c.id == id) {
            self.sprites[i] = Some(sprite);
            self.draw();
        }
    }

    /// Whether the player is using touch, so on-screen hints say "tap" instead of naming keys.
    pub fn set_touch(&mut self, touch: bool) {
        if self.touch != touch {
            self.touch = touch;
            self.draw();
        }
    }

    pub fn set_hi_score(&mut self, score: f64) {
        self.hi_score = score.max(0.0);
    }

    // ----- read-back for the Svelte UI -----

    /// "ready" | "running" | "paused" | "over"
    pub fn mode(&self) -> String {
        match self.mode {
            Mode::Ready => "ready",
            Mode::Running => "running",
            Mode::Paused => "paused",
            Mode::Over => "over",
        }
        .into()
    }

    /// Stars collected toward the special move.
    pub fn charge(&self) -> u32 {
        self.charge
    }

    pub fn special_active(&self) -> bool {
        self.active.kind.is_some()
    }

    pub fn hi_score(&self) -> f64 {
        self.hi_score.floor()
    }

    /// Score of the run that just ended.
    pub fn last_score(&self) -> f64 {
        self.last_score
    }

    pub fn last_is_best(&self) -> bool {
        self.last_is_best
    }

    /// What ended the last run, e.g. "a moon rock".
    pub fn bumped_into(&self) -> String {
        self.bumped_into.into()
    }

    /// Index of the chosen character; changes when the player picks someone else.
    pub fn selected(&self) -> usize {
        self.sel
    }

    /// The chosen character, for the UI around the canvas: `{ id, name, role, hero, accent, flip,
    /// passiveName, passiveText, specialName, specialCost, specialText }`.
    pub fn character(&self) -> Object {
        let c = self.chr();
        let o = Object::new();
        let set = |k: &str, v: JsValue| {
            let _ = Reflect::set(&o, &k.into(), &v);
        };
        set("id", c.id.into());
        set("name", c.name.into());
        set("role", c.role.into());
        set("hero", c.hero.into());
        set("accent", c.accent.into());
        set("flip", c.flip.into());
        set("passiveName", c.passive_name.into());
        set("passiveText", c.passive_text.into());
        set("specialName", c.special_name.into());
        set("specialCost", c.special_cost.into());
        set("specialText", c.special_text.into());
        o
    }

    /// How to play, worded for touch or keys. The intro screen draws it, except on narrow
    /// screens, where the page shows it as text below the game instead.
    pub fn controls(&self) -> String {
        self.controls_text().into()
    }

    // ----- input (`now` is a performance.now() / rAF timestamp in ms) -----

    pub fn press_jump(&mut self, now: f64) {
        match self.mode {
            Mode::Ready => return self.start(now),
            Mode::Over => {
                if now >= self.over_lock_until {
                    self.start(now);
                }
                return;
            }
            Mode::Paused => return self.resume(now),
            Mode::Running => {}
        }
        self.player.held = true;
        if self.active.kind == Some(SpecialKind::Rocket) || self.player.ducking {
            return;
        }
        if self.player.on_ground {
            self.player.vy = JUMP_V;
            self.player.on_ground = false;
            self.player.hold_t = 0.0;
            self.player.jumps_used = 1;
        } else if self.player.jumps_used < self.chr().jumps {
            // Crosby's starlight double jump
            self.player.vy = AIR_JUMP_V;
            self.player.hold_t = self.chr().hold_time;
            self.player.jumps_used += 1;
            self.burst(PLAYER_X + 34.0, self.player.y - 4.0, 8, &[C::TITLE, C::GOLD, C::TEXT], 140.0);
        }
    }

    /// A tap or click at (x, y) in world units. On the intro screen it picks a friend (tapping
    /// the chosen one, or anywhere else, starts the run); after a crash the "choose a friend"
    /// button goes back to the intro; otherwise it's a jump.
    pub fn tap(&mut self, x: f64, y: f64, now: f64) {
        match self.mode {
            Mode::Ready => match (0..CHARACTERS.len()).find(|&i| self.chip_rect(i).contains(x, y)) {
                Some(i) if i != self.sel => self.choose(i),
                _ => self.start(now),
            },
            Mode::Over if self.friends_button().contains(x, y) => self.choose(self.sel),
            _ => self.press_jump(now),
        }
    }

    /// Step through the friends (← / →) on the intro screen, or from the crash screen.
    pub fn pick_step(&mut self, delta: i32) {
        if matches!(self.mode, Mode::Ready | Mode::Over) {
            let n = CHARACTERS.len() as i32;
            self.choose((self.sel as i32 + delta).rem_euclid(n) as usize);
        }
    }

    pub fn release_jump(&mut self) {
        self.player.held = false;
    }

    /// Crouching only starts during a run, but letting go always counts, so a key released
    /// while paused can't leave the player stuck crouched (and unable to jump) after resuming.
    pub fn set_duck(&mut self, on: bool) {
        if !on || self.mode == Mode::Running {
            self.player.ducking = on;
        }
    }

    pub fn use_special(&mut self, now: f64) {
        if self.mode == Mode::Paused {
            return self.resume(now);
        }
        if self.mode != Mode::Running || self.active.kind.is_some() || self.charge < self.chr().special_cost {
            return;
        }
        self.charge = 0;
        let d = self.chr().special_duration;
        self.active = Active { kind: Some(self.chr().special_kind), t: d, max: d };
        if self.chr().special_kind == SpecialKind::Rocket {
            self.player.on_ground = false;
            self.player.vy = 0.0;
            self.player.ducking = false;
        }
        let accent = self.chr().accent;
        self.burst(PLAYER_X + 34.0, self.player.y - 36.0, 14, &[accent, C::TITLE, C::TEXT], 180.0);
    }

    pub fn pause(&mut self) {
        if self.mode == Mode::Running {
            self.mode = Mode::Paused;
            // Pausing on focus loss or a hidden tab means the key-ups may never arrive, so
            // drop held input; a key still held after resuming auto-repeats and re-applies it.
            self.player.ducking = false;
            self.player.held = false;
            self.draw();
        }
    }

    /// Advance one animation frame and draw it. Returns whether the caller should keep
    /// requesting frames (while running, and briefly after a crash so the burst plays out).
    pub fn frame(&mut self, now: f64) -> bool {
        match self.mode {
            Mode::Running => {
                let dt = self.step_time(now);
                self.update(dt, now);
                self.draw();
                true
            }
            Mode::Over => {
                let dt = self.step_time(now);
                self.clock += dt;
                self.update_particles(dt);
                self.draw();
                now < self.over_tail_until
            }
            Mode::Ready | Mode::Paused => false,
        }
    }

    /// Redraw without advancing (after a sprite or web font finishes loading, say).
    pub fn draw(&mut self) {
        // A failed canvas call only loses part of one frame, so there's nothing to recover.
        let _ = self.render();
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

// ---------- game flow & simulation ----------

impl Game {
    fn chr(&self) -> &'static Character {
        &CHARACTERS[self.sel]
    }

    /// Switch friend and go (back) to the intro screen.
    fn choose(&mut self, i: usize) {
        self.sel = i;
        self.reset();
        self.mode = Mode::Ready;
        self.draw();
    }

    fn step_time(&mut self, now: f64) -> f64 {
        // rAF timestamps can trail performance.now(), so never step backwards
        let dt = ((now - self.last_time) / 1000.0).clamp(0.0, 0.05);
        self.last_time = now;
        dt
    }

    fn seed_scenery(&mut self) {
        let rng = &mut self.rng;
        self.star_layers = [0.03, 0.07, 0.13]
            .iter()
            .enumerate()
            .map(|(i, &par)| StarLayer {
                par,
                stars: (0..46 - i * 12)
                    .map(|_| BgStar {
                        x: rng.range(0.0, 1000.0),
                        y: rng.range(4.0, GROUND - 14.0),
                        r: rng.range(0.5, 1.0) + i as f64 * 0.35,
                        tw: rng.range(1.2, 3.2),
                        ph: rng.range(0.0, TAU),
                    })
                    .collect(),
            })
            .collect();
        self.gold_stars = (0..7)
            .map(|_| GoldStar {
                x: rng.range(0.0, 1000.0),
                y: rng.range(14.0, GROUND - 70.0),
                r: rng.range(3.5, 6.0),
                ph: rng.range(0.0, TAU),
            })
            .collect();
        self.nebulae = vec![
            Nebula { x: 120.0, y: 70.0, r: 150.0, use_a: true },
            Nebula { x: 480.0, y: 130.0, r: 180.0, use_a: false },
            Nebula { x: 820.0, y: 60.0, r: 140.0, use_a: true },
        ];
        self.planets = vec![
            Planet { x: 560.0, y: 66.0, r: 20.0, kind: PlanetKind::Ringed },
            Planet { x: 960.0, y: 120.0, r: 11.0, kind: PlanetKind::Moon },
            Planet { x: 260.0, y: 150.0, r: 8.0, kind: PlanetKind::Blue },
        ];
        self.shooting = None;
        self.next_shoot = self.rng.range(2.0, 4.0);
    }

    fn reset(&mut self) {
        self.speed = START_SPEED;
        self.score = 0.0;
        self.distance = 0.0;
        self.clock = 0.0;
        self.next_spawn = 200.0;
        self.last_milestone = 0;
        self.flash_until = 0.0;
        self.bumped_into = "";
        self.charge = 0;
        self.active = INACTIVE;
        self.grace = 0.0;
        self.zone_index = 0;
        self.zone_banner_t = 0.0;
        self.neb_a = ZONES[0].a;
        self.neb_b = ZONES[0].b;
        self.player = Player::new();
        self.obstacles.clear();
        self.pickups.clear();
        self.particles.clear();
        self.popups.clear();
        self.seed_scenery();
    }

    fn start(&mut self, now: f64) {
        self.reset();
        self.mode = Mode::Running;
        self.last_time = now;
    }

    fn resume(&mut self, now: f64) {
        self.mode = Mode::Running;
        self.last_time = now;
    }

    fn game_over(&mut self, now: f64) {
        self.mode = Mode::Over;
        self.over_lock_until = now + OVER_LOCK_MS;
        self.over_tail_until = now + OVER_TAIL_MS;
        self.last_score = self.score.floor();
        self.last_is_best = self.score > self.hi_score;
        if self.last_is_best {
            self.hi_score = self.score;
        }
        self.active = INACTIVE;
        self.burst(PLAYER_X + 30.0, self.player.y - 40.0, 10, &[C::TITLE, C::TEXT], 120.0);
    }

    fn make_blob(&mut self, w: f64, h: f64, n: usize) -> Vec<(f64, f64)> {
        (0..n)
            .map(|i| {
                let a = (i as f64 / n as f64) * PI * 2.0;
                let r = self.rng.range(0.82, 1.0);
                (a.cos() * (w / 2.0) * r, (a.sin() * (h / 2.0) * r).min(h / 2.0 - 2.0))
            })
            .collect()
    }

    fn spawn_obstacle(&mut self) {
        let x = self.w + SPAWN_AHEAD;
        let r = self.rng.next();

        let ob = if self.score > 150.0 && r < 0.25 {
            let lanes = [GROUND - 24.0, GROUND - 60.0, GROUND - 106.0];
            let cy = lanes[(self.rng.next() * lanes.len() as f64) as usize];
            Obstacle {
                kind: ObKind::Comet { spin: self.rng.range(0.0, 6.0) },
                label: "a shooting comet",
                x,
                y: cy - 11.0,
                w: 44.0,
                h: 22.0,
                hit: Rect { x: 6.0, y: 4.0, w: 24.0, h: 14.0 },
                extra: 70.0,
                smashed: false,
            }
        } else if r < 0.62 {
            let mut count = 1;
            if self.speed > 470.0 && self.rng.chance(0.5) {
                count += 1;
            }
            if self.speed > 600.0 && self.rng.chance(0.35) {
                count += 1;
            }
            let mut parts = Vec::with_capacity(count);
            let mut dx = 0.0;
            let mut max_h: f64 = 0.0;
            for _ in 0..count {
                let big = self.rng.chance(0.4);
                let w = if big { self.rng.range(32.0, 38.0) } else { self.rng.range(24.0, 30.0) };
                let h = if big { self.rng.range(32.0, 40.0) } else { self.rng.range(20.0, 26.0) };
                let pts = self.make_blob(w, h, 8);
                let craters = [
                    (self.rng.range(-0.25, 0.1) * w, self.rng.range(-0.2, 0.1) * h, self.rng.range(2.0, 4.0)),
                    (self.rng.range(0.05, 0.25) * w, self.rng.range(0.0, 0.2) * h, self.rng.range(1.5, 3.0)),
                ];
                parts.push(RockPart { dx, w, h, pts, craters });
                dx += w - 4.0;
                max_h = max_h.max(h);
            }
            let w = dx + 4.0;
            Obstacle {
                kind: ObKind::Rocks(parts),
                label: if count > 1 { "some moon rocks" } else { "a moon rock" },
                x,
                y: GROUND - max_h,
                w,
                h: max_h,
                hit: Rect { x: 4.0, y: 5.0, w: w - 8.0, h: max_h - 5.0 },
                extra: 0.0,
                smashed: false,
            }
        } else {
            let count = if self.rng.chance(0.5) { 2 } else { 3 };
            let hue = if self.rng.chance(0.5) { ("#8ff3ea", "#3fb8c9") } else { ("#ffc2e6", "#e66aae") };
            let mut shards = Vec::with_capacity(count);
            let mut dx = 0.0;
            let mut max_h: f64 = 0.0;
            for i in 0..count {
                let w = self.rng.range(12.0, 16.0);
                let h = if i == 1 { self.rng.range(36.0, 46.0) } else { self.rng.range(20.0, 30.0) };
                let tilt = (i as f64 - (count as f64 - 1.0) / 2.0) * self.rng.range(3.0, 6.0);
                shards.push(Shard { dx: dx + w / 2.0, w, h, tilt });
                dx += w - 3.0;
                max_h = max_h.max(h);
            }
            let w = dx + 3.0;
            Obstacle {
                kind: ObKind::Crystal { shards, hue },
                label: "a space crystal",
                x,
                y: GROUND - max_h,
                w,
                h: max_h,
                hit: Rect { x: 4.0, y: 6.0, w: w - 8.0, h: max_h - 6.0 },
                extra: 0.0,
                smashed: false,
            }
        };

        self.next_spawn = ob.w + self.speed * self.rng.range(0.78, 1.55) + 60.0;

        // Reward a well-timed jump: sometimes float a star over ground obstacles.
        if !matches!(ob.kind, ObKind::Comet { .. }) && self.rng.chance(0.32) {
            let y = ob.y - self.rng.range(42.0, 62.0);
            let phase = self.rng.range(0.0, 6.0);
            self.pickups.push(Pickup { x: ob.x + ob.w / 2.0, y, phase });
        }
        // ...and sometimes a little row of stars in the gap before the next one.
        if self.rng.chance(0.16) {
            let mid = ob.x + ob.w + (self.next_spawn - ob.w) / 2.0;
            for i in -1..=1 {
                let i = i as f64;
                self.pickups.push(Pickup { x: mid + i * 26.0, y: GROUND - 34.0, phase: i });
            }
        }
        self.obstacles.push(ob);
    }

    /// The character's drawn size this frame: (base width, width, height, x scale, y scale).
    fn player_metrics(&self) -> (f64, f64, f64, f64, f64) {
        let c = self.chr();
        let ducking = self.player.ducking && self.player.on_ground;
        let (sx, sy) = if ducking { (DUCK_SX, DUCK_SY) } else { (1.0, 1.0) };
        let base_w = c.height * c.aspect;
        (base_w, base_w * sx, c.height * sy, sx, sy)
    }

    fn player_box(&self) -> Rect {
        let (base_w, w, h, _, _) = self.player_metrics();
        let c = self.chr();
        let [hl, hr, ht, hb] = c.hit;
        let left = PLAYER_X + base_w / 2.0 - w / 2.0;
        let top = self.player.y - h;
        let (l, r) = if c.flip { (hr, hl) } else { (hl, hr) };
        Rect { x: left + w * l, y: top + h * ht, w: w * (1.0 - l - r), h: h * (1.0 - ht - hb) }
    }

    fn burst(&mut self, x: f64, y: f64, n: usize, colors: &[&'static str], power: f64) {
        for i in 0..n {
            let a = (i as f64 / n as f64) * PI * 2.0 + self.rng.range(-0.2, 0.2);
            let vx = a.cos() * power * self.rng.range(0.6, 1.0);
            let vy = a.sin() * power * self.rng.range(0.6, 1.0) - 40.0;
            self.particles.push(Particle {
                x,
                y,
                vx,
                vy,
                life: 0.55,
                max: 0.55,
                color: colors[i % colors.len()],
                star: true,
            });
        }
    }

    fn update_player(&mut self, dt: f64) {
        if self.active.kind == Some(SpecialKind::Rocket) {
            let p = &mut self.player;
            p.y += (ROCKET_Y - p.y) * (dt * 5.0).min(1.0);
            p.vy = 0.0;
            p.on_ground = false;
            if self.rng.chance(0.8) {
                let particle = Particle {
                    x: PLAYER_X + 14.0,
                    y: self.player.y - 18.0,
                    vx: self.rng.range(-160.0, -90.0),
                    vy: self.rng.range(40.0, 110.0),
                    life: 0.4,
                    max: 0.4,
                    color: if self.rng.chance(0.5) { "#ffb347" } else { C::TITLE },
                    star: false,
                };
                self.particles.push(particle);
            }
            return;
        }

        if self.player.on_ground {
            return;
        }
        let c = self.chr();
        let p = &mut self.player;
        let mut g = GRAVITY;
        if p.held && p.vy < 0.0 && p.hold_t < c.hold_time {
            g *= c.hold_gravity;
            p.hold_t += dt;
        }
        if p.ducking {
            g *= FAST_FALL;
        }
        p.vy += g * dt;
        p.y += p.vy * dt;

        // keep the character on screen
        let ceiling = c.height + 6.0;
        if p.y < ceiling {
            p.y = ceiling;
            p.vy = p.vy.max(0.0);
        }

        if p.y >= GROUND {
            p.y = GROUND;
            p.vy = 0.0;
            p.on_ground = true;
            p.jumps_used = 0;
            for _ in 0..4 {
                let particle = Particle {
                    x: PLAYER_X + self.rng.range(10.0, 40.0),
                    y: GROUND - 2.0,
                    vx: self.rng.range(-90.0, -20.0),
                    vy: self.rng.range(-60.0, -20.0),
                    life: 0.35,
                    max: 0.35,
                    color: C::GROUND_EDGE,
                    star: false,
                };
                self.particles.push(particle);
            }
        }
    }

    fn update_scenery(&mut self, dt: f64) {
        for i in 0..self.planets.len() {
            if self.planets[i].x - self.distance * 0.05 < -60.0 {
                self.planets[i].x += 1100.0 + self.rng.range(0.0, 300.0);
            }
        }
        if let Some(s) = &mut self.shooting {
            s.t += dt;
            if s.t > s.life {
                self.shooting = None;
            }
        } else {
            self.next_shoot -= dt;
            if self.next_shoot <= 0.0 {
                let rng = &mut self.rng;
                self.shooting = Some(Shooting {
                    x: rng.range(self.w * 0.3, self.w + 40.0),
                    y: rng.range(10.0, 80.0),
                    vx: rng.range(-420.0, -320.0),
                    vy: rng.range(120.0, 180.0),
                    t: 0.0,
                    life: 0.9,
                });
                self.next_shoot = rng.range(3.0, 7.0);
            }
        }
        let z = &ZONES[self.zone_index];
        let k = (dt * 0.8).min(1.0);
        self.neb_a = approach(self.neb_a, z.a, k);
        self.neb_b = approach(self.neb_b, z.b, k);
        self.zone_banner_t = (self.zone_banner_t - dt).max(0.0);
    }

    fn update(&mut self, dt: f64, now: f64) {
        self.clock += dt;

        if let Some(kind) = self.active.kind {
            self.active.t -= dt;
            if self.active.t <= 0.0 {
                if kind == SpecialKind::Rocket {
                    self.player.held = false;
                }
                self.active = INACTIVE;
                self.grace = GRACE;
            }
        }
        self.grace = (self.grace - dt).max(0.0);

        let kind = self.active.kind;
        let time_scale = if kind == Some(SpecialKind::Calm) { 0.55 } else { 1.0 };
        let wdt = dt * time_scale;
        self.speed = (self.speed + ACCEL * wdt).min(MAX_SPEED);
        let mv = self.speed * wdt;
        self.distance += mv;
        let mult = if kind == Some(SpecialKind::Brave) { 2.0 } else { 1.0 };
        self.score += mv * 0.025 * mult;

        let milestone = (self.score / 100.0).floor() as i64;
        if milestone > self.last_milestone {
            self.last_milestone = milestone;
            self.flash_until = self.clock + 0.9;
        }

        let z = (self.score / ZONE_EVERY).floor().max(0.0) as usize % ZONES.len();
        if z != self.zone_index {
            self.zone_index = z;
            self.zone_banner_t = 2.6;
        }

        self.update_player(dt);
        self.update_scenery(dt);

        // obstacles
        self.next_spawn -= mv;
        if self.next_spawn <= 0.0 {
            self.spawn_obstacle();
        }

        let pb = self.player_box();
        let brave = kind == Some(SpecialKind::Brave);
        let invulnerable = brave || kind == Some(SpecialKind::Rocket) || self.grace > 0.0;
        let mut smashed = Vec::new();
        let mut crashed_into = None;
        for ob in &mut self.obstacles {
            ob.x -= mv + ob.extra * wdt;
            if let ObKind::Comet { spin } = &mut ob.kind {
                *spin += dt * 5.0;
            }
            if ob.smashed {
                continue;
            }
            let hb = Rect { x: ob.x + ob.hit.x, y: ob.y + ob.hit.y, w: ob.hit.w, h: ob.hit.h };
            if !pb.overlaps(&hb) {
                continue;
            }
            if brave {
                ob.smashed = true;
                smashed.push((ob.x + ob.w / 2.0, ob.y, ob.h));
            } else if !invulnerable {
                crashed_into = Some(ob.label);
                break;
            }
        }
        for (cx, y, h) in smashed {
            self.score += 25.0 * mult;
            self.popups.push(Popup { x: cx, y: y - 8.0, life: 0.8, text: format!("+{}", 25.0 * mult) });
            self.burst(cx, y + h / 2.0, 12, &[C::TITLE, C::ROCK_LIGHT, "#ff9ad5"], 200.0);
        }
        if let Some(label) = crashed_into {
            self.bumped_into = label;
            self.game_over(now);
            return;
        }
        self.obstacles.retain(|ob| !ob.smashed && ob.x + ob.w > -20.0);

        // stars
        let magnet = match kind {
            Some(SpecialKind::Brave) => 220.0,
            Some(SpecialKind::Rocket) => 150.0,
            _ => self.chr().magnet,
        };
        let pcx = pb.x + pb.w / 2.0;
        let pcy = pb.y + pb.h / 2.0;
        let mut taken = Vec::new();
        self.pickups.retain_mut(|p| {
            p.x -= mv;
            if magnet > 0.0 {
                let dx = pcx - p.x;
                let dy = pcy - p.y;
                let d = dx.hypot(dy);
                if d < magnet && d > 1.0 {
                    let pull = (1.0 - d / magnet) * 900.0 * dt + 120.0 * dt;
                    p.x += (dx / d) * pull;
                    p.y += (dy / d) * pull;
                }
            }
            if pb.overlaps(&Rect { x: p.x - 10.0, y: p.y - 10.0, w: 20.0, h: 20.0 }) {
                taken.push((p.x, p.y));
                return false;
            }
            p.x > -20.0
        });
        for (x, y) in taken {
            self.score += 20.0 * mult;
            if kind.is_none() && self.charge < self.chr().special_cost {
                self.charge += 1;
            }
            self.popups.push(Popup { x, y: y - 12.0, life: 0.8, text: format!("+{}", 20.0 * mult) });
            self.burst(x, y, 8, &[C::TITLE], 110.0);
        }

        for p in &mut self.popups {
            p.y -= 30.0 * dt;
            p.life -= dt;
        }
        self.popups.retain(|p| p.life > 0.0);

        self.update_particles(dt);
    }

    fn update_particles(&mut self, dt: f64) {
        for p in &mut self.particles {
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.vy += 400.0 * dt;
            p.life -= dt;
        }
        self.particles.retain(|p| p.life > 0.0);
    }
}
