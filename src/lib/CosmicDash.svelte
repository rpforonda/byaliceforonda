<script>
  import { resolve } from "$app/paths";
  import { onMount } from "svelte";

  // Endless runner in the spirit of Chrome's offline dino game, themed after
  // Crosby's Cosmic Adventure. Pick a character, jump the moon rocks and
  // starberry bushes, duck (or jump) the comets, and grab stars for bonus points.

  const characters = [
    {
      id: "crosby",
      name: "Crosby",
      src: resolve("/Crosby.webp"),
      aspect: 360 / 325,
      height: 70,
      flip: false,
      sink: 3,
      hit: { l: 0.32, r: 0.12, t: 0.18, b: 0.06 },
      accent: "#8fc27b",
    },
    {
      id: "carla",
      name: "Carla",
      src: resolve("/Carla.webp"),
      aspect: 232 / 329,
      height: 68,
      flip: true,
      sink: 2,
      hit: { l: 0.16, r: 0.16, t: 0.1, b: 0.04 },
      accent: "#ef6f3c",
    },
    {
      id: "tank",
      name: "Tank",
      src: resolve("/Tank.webp"),
      aspect: 458 / 544,
      height: 76,
      flip: false,
      sink: 5,
      hit: { l: 0.12, r: 0.24, t: 0.08, b: 0.1 },
      accent: "#afab23",
    },
  ];

  // Palette pulled from the site and the Crosby's Cosmic Adventure cover.
  const C = {
    sky: "#d7e6ff",
    skyNight: "#3a3577",
    skyNightLow: "#6a4f9e",
    doodle: "#34408a",
    doodleNight: "#ffe9a8",
    title: "#f3d36f",
    titleShadow: "#7c2b28",
    ink: "#352f47",
    ground: "#8fc27b",
    groundNight: "#4e6b52",
    groundEdge: "#6b8a32",
    groundEdgeNight: "#33503c",
    rock: "#c9c2df",
    rockShade: "#a79fc4",
    bush: "#8fc27b",
    bushDark: "#25533f",
    berry: "#d2234c",
    star: "#ffd28d",
    cloud: "#fff6e3",
    moon: "#fff3cf",
  };

  const H = 240;
  const GROUND = 204;
  const PLAYER_X = 48;
  const GRAVITY = 2600;
  const JUMP_V = -700;
  const HOLD_TIME = 0.17;
  const HOLD_GRAVITY = 0.55;
  const FAST_FALL = 2.4;
  const DUCK_SX = 1.18;
  const DUCK_SY = 0.6;
  const START_SPEED = 340;
  const MAX_SPEED = 760;
  const ACCEL = 9;
  const NIGHT_EVERY = 500;
  const HI_KEY = "catchupwithalice-cosmic-dash-hi";

  let wrapper = $state();
  let canvas = $state();
  let selectedId = $state("crosby");
  let uiState = $state("ready");
  let announcement = $state("");

  const images = {};
  let ctx;
  let W = 720;
  let rafId = 0;
  let lastTime = 0;

  // World state (plain variables: mutated every frame, no reactivity needed)
  let mode = "ready";
  let speed = START_SPEED;
  let score = 0;
  let hiScore = 0;
  let distance = 0;
  let clock = 0;
  let night = 0;
  let nextSpawn = 0;
  let overLockUntil = 0;
  let flashUntil = 0;
  let lastMilestone = 0;
  let bumpedInto = "";
  const player = { y: GROUND, vy: 0, onGround: true, holdT: 0, held: false, ducking: false };
  let obstacles = [];
  let pickups = [];
  let particles = [];
  let popups = [];
  let doodles = [];
  let clouds = [];

  const char = () => characters.find((c) => c.id === selectedId) ?? characters[0];
  const rand = (a, b) => a + Math.random() * (b - a);
  const pad = (n) => String(Math.floor(n)).padStart(5, "0");

  function lerpColor(a, b, t) {
    const pa = parseInt(a.slice(1), 16);
    const pb = parseInt(b.slice(1), 16);
    const ch = (s) => {
      const x = (pa >> s) & 255;
      const y = (pb >> s) & 255;
      return Math.round(x + (y - x) * t);
    };
    return `rgb(${ch(16)},${ch(8)},${ch(0)})`;
  }

  function loadHiScore() {
    try {
      hiScore = Number(localStorage.getItem(HI_KEY)) || 0;
    } catch {
      hiScore = 0;
    }
  }

  function saveHiScore() {
    try {
      localStorage.setItem(HI_KEY, String(Math.floor(hiScore)));
    } catch {
      // storage unavailable (private mode etc.) — the score just won't persist
    }
  }

  // ---------- world setup ----------

  function seedScenery() {
    doodles = [];
    const kinds = ["star", "flower", "star", "dot", "swirl"];
    for (let i = 0; i < 9; i++) {
      doodles.push({
        x: rand(0, 900),
        y: rand(18, GROUND - 90),
        kind: kinds[i % kinds.length],
        size: rand(7, 13),
        rot: rand(-0.4, 0.4),
      });
    }
    clouds = [];
    for (let i = 0; i < 3; i++) {
      clouds.push({ x: rand(0, 900), y: rand(30, 110), s: rand(0.7, 1.15) });
    }
  }

  function reset() {
    speed = START_SPEED;
    score = 0;
    distance = 0;
    clock = 0;
    night = 0;
    nextSpawn = 380;
    lastMilestone = 0;
    flashUntil = 0;
    bumpedInto = "";
    Object.assign(player, { y: GROUND, vy: 0, onGround: true, holdT: 0, held: false, ducking: false });
    obstacles = [];
    pickups = [];
    particles = [];
    popups = [];
    seedScenery();
  }

  function makeBlob(w, h, n = 8) {
    const pts = [];
    for (let i = 0; i < n; i++) {
      const a = (i / n) * Math.PI * 2;
      const r = rand(0.82, 1);
      pts.push([Math.cos(a) * (w / 2) * r, Math.min(Math.sin(a) * (h / 2) * r, h / 2 - 2)]);
    }
    return pts;
  }

  function spawnObstacle() {
    const x = W + 30;
    const r = Math.random();
    let ob;

    if (score > 220 && r < 0.24) {
      const lanes = [GROUND - 24, GROUND - 60, GROUND - 104];
      const cy = lanes[Math.floor(Math.random() * lanes.length)];
      ob = {
        kind: "comet",
        label: "a comet",
        x,
        y: cy - 11,
        w: 44,
        h: 22,
        hit: { x: 6, y: 4, w: 24, h: 14 },
        extra: 70,
        spin: rand(0, 6),
      };
    } else if (r < 0.62) {
      let count = 1;
      if (speed > 470 && Math.random() < 0.5) count++;
      if (speed > 600 && Math.random() < 0.35) count++;
      const parts = [];
      let dx = 0;
      let maxH = 0;
      for (let i = 0; i < count; i++) {
        const big = Math.random() < 0.4;
        const w = big ? rand(32, 38) : rand(24, 30);
        const h = big ? rand(32, 40) : rand(20, 26);
        parts.push({
          dx,
          w,
          h,
          pts: makeBlob(w, h),
          craters: [
            [rand(-0.25, 0.1) * w, rand(-0.2, 0.1) * h, rand(2, 4)],
            [rand(0.05, 0.25) * w, rand(0, 0.2) * h, rand(1.5, 3)],
          ],
        });
        dx += w - 4;
        maxH = Math.max(maxH, h);
      }
      const w = dx + 4;
      ob = {
        kind: "rocks",
        label: count > 1 ? "some moon rocks" : "a moon rock",
        x,
        y: GROUND - maxH,
        w,
        h: maxH,
        hit: { x: 4, y: 5, w: w - 8, h: maxH - 5 },
        parts,
      };
    } else {
      const tall = Math.random() < 0.45;
      const w = tall ? 34 : 46;
      const h = tall ? 46 : 32;
      const puffs = tall
        ? [
            [0, -h + 14, 13],
            [-8, -h + 30, 12],
            [8, -h + 28, 12],
            [-6, -10, 12],
            [7, -10, 11],
          ]
        : [
            [-12, -12, 12],
            [12, -12, 12],
            [0, -20, 13],
            [-2, -9, 12],
          ];
      ob = {
        kind: "bush",
        label: "a starberry bush",
        x,
        y: GROUND - h,
        w,
        h,
        hit: { x: 5, y: 5, w: w - 10, h: h - 5 },
        puffs,
        berries: puffs.slice(0, 3).map(([px, py]) => [px + rand(-5, 5), py + rand(-5, 3)]),
      };
    }

    obstacles.push(ob);

    // Reward a well-timed jump: sometimes float a star over ground obstacles.
    if (ob.kind !== "comet" && Math.random() < 0.4) {
      pickups.push({ x: ob.x + ob.w / 2, y: ob.y - rand(42, 62), phase: rand(0, 6) });
    }

    nextSpawn = ob.w + speed * rand(0.78, 1.55) + 60;
  }

  // ---------- game flow ----------

  function start() {
    reset();
    mode = "running";
    uiState = "running";
    announcement = "";
    lastTime = performance.now();
    cancelAnimationFrame(rafId);
    rafId = requestAnimationFrame(frame);
  }

  function gameOver() {
    mode = "over";
    uiState = "over";
    overLockUntil = performance.now() + 450;
    if (score > hiScore) {
      hiScore = score;
      saveHiScore();
    }
    announcement = `${char().name} bumped into ${bumpedInto}. Score ${Math.floor(score)}.`;
    for (let i = 0; i < 10; i++) {
      particles.push({
        x: PLAYER_X + 30,
        y: player.y - 40,
        vx: rand(-120, 120),
        vy: rand(-220, -40),
        life: 0.7,
        max: 0.7,
        color: i % 2 ? C.title : C.cloud,
        star: true,
      });
    }
    // let the burst play out briefly
    const burstEnd = performance.now() + 700;
    const tail = (now) => {
      const dt = Math.min(0.05, (now - lastTime) / 1000);
      lastTime = now;
      clock += dt;
      updateParticles(dt);
      draw();
      if (now < burstEnd && mode === "over") rafId = requestAnimationFrame(tail);
    };
    rafId = requestAnimationFrame(tail);
  }

  function pause() {
    if (mode !== "running") return;
    mode = "paused";
    uiState = "paused";
    cancelAnimationFrame(rafId);
    draw();
  }

  function resume() {
    mode = "running";
    uiState = "running";
    lastTime = performance.now();
    rafId = requestAnimationFrame(frame);
  }

  function pressJump() {
    if (mode === "ready") return start();
    if (mode === "over") {
      if (performance.now() >= overLockUntil) start();
      return;
    }
    if (mode === "paused") return resume();
    player.held = true;
    if (player.onGround && !player.ducking) {
      player.vy = JUMP_V;
      player.onGround = false;
      player.holdT = 0;
    }
  }

  function releaseJump() {
    player.held = false;
  }

  function setDuck(on) {
    if (mode !== "running") return;
    player.ducking = on;
  }

  // ---------- update ----------

  function frame(now) {
    if (mode !== "running") return;
    const dt = Math.min(0.05, (now - lastTime) / 1000);
    lastTime = now;
    update(dt);
    draw();
    if (mode === "running") rafId = requestAnimationFrame(frame);
  }

  function boxesOverlap(a, b) {
    return a.x < b.x + b.w && a.x + a.w > b.x && a.y < b.y + b.h && a.y + a.h > b.y;
  }

  function playerMetrics() {
    const c = char();
    const ducking = player.ducking && player.onGround;
    const sx = ducking ? DUCK_SX : 1;
    const sy = ducking ? DUCK_SY : 1;
    const baseW = c.height * c.aspect;
    return { c, baseW, w: baseW * sx, h: c.height * sy, sx, sy };
  }

  function playerBox() {
    const { c, baseW, w, h } = playerMetrics();
    const left = PLAYER_X + baseW / 2 - w / 2;
    const top = player.y - h;
    const l = c.flip ? c.hit.r : c.hit.l;
    const r = c.flip ? c.hit.l : c.hit.r;
    return {
      x: left + w * l,
      y: top + h * c.hit.t,
      w: w * (1 - l - r),
      h: h * (1 - c.hit.t - c.hit.b),
    };
  }

  function update(dt) {
    clock += dt;
    speed = Math.min(MAX_SPEED, speed + ACCEL * dt);
    const move = speed * dt;
    distance += move;
    score += move * 0.025;

    const milestone = Math.floor(score / 100);
    if (milestone > lastMilestone) {
      lastMilestone = milestone;
      flashUntil = clock + 0.9;
    }

    const targetNight = Math.floor(score / NIGHT_EVERY) % 2;
    night += Math.sign(targetNight - night) * Math.min(Math.abs(targetNight - night), dt * 0.6);

    // player physics
    if (!player.onGround) {
      let g = GRAVITY;
      if (player.held && player.vy < 0 && player.holdT < HOLD_TIME) {
        g *= HOLD_GRAVITY;
        player.holdT += dt;
      }
      if (player.ducking) g *= FAST_FALL;
      player.vy += g * dt;
      player.y += player.vy * dt;
      if (player.y >= GROUND) {
        player.y = GROUND;
        player.vy = 0;
        player.onGround = true;
        for (let i = 0; i < 4; i++) {
          particles.push({
            x: PLAYER_X + rand(10, 40),
            y: GROUND - 2,
            vx: rand(-90, -20),
            vy: rand(-60, -20),
            life: 0.35,
            max: 0.35,
            color: C.groundEdge,
          });
        }
      }
    }

    // scenery
    for (const d of doodles) {
      d.x -= move * 0.12;
      if (d.x < -20) {
        d.x = W + rand(10, 200);
        d.y = rand(18, GROUND - 90);
      }
    }
    for (const cl of clouds) {
      cl.x -= move * 0.3;
      if (cl.x < -80) {
        cl.x = W + rand(40, 300);
        cl.y = rand(30, 110);
      }
    }

    // obstacles
    nextSpawn -= move;
    if (nextSpawn <= 0) spawnObstacle();

    const pb = playerBox();
    for (const ob of obstacles) {
      ob.x -= move + (ob.extra ?? 0) * dt;
      if (ob.kind === "comet") ob.spin += dt * 5;
      const hb = { x: ob.x + ob.hit.x, y: ob.y + ob.hit.y, w: ob.hit.w, h: ob.hit.h };
      if (boxesOverlap(pb, hb)) {
        bumpedInto = ob.label;
        gameOver();
        return;
      }
    }
    obstacles = obstacles.filter((ob) => ob.x + ob.w > -20);

    for (const p of pickups) {
      p.x -= move;
      if (!p.taken && boxesOverlap(pb, { x: p.x - 9, y: p.y - 9, w: 18, h: 18 })) {
        p.taken = true;
        score += 20;
        popups.push({ x: p.x, y: p.y - 12, life: 0.8 });
        for (let i = 0; i < 8; i++) {
          const a = (i / 8) * Math.PI * 2;
          particles.push({
            x: p.x,
            y: p.y,
            vx: Math.cos(a) * 110,
            vy: Math.sin(a) * 110,
            life: 0.45,
            max: 0.45,
            color: C.title,
            star: true,
          });
        }
      }
    }
    pickups = pickups.filter((p) => !p.taken && p.x > -20);

    for (const p of popups) {
      p.y -= 30 * dt;
      p.life -= dt;
    }
    popups = popups.filter((p) => p.life > 0);

    updateParticles(dt);
  }

  function updateParticles(dt) {
    for (const p of particles) {
      p.x += p.vx * dt;
      p.y += p.vy * dt;
      p.vy += 400 * dt;
      p.life -= dt;
    }
    particles = particles.filter((p) => p.life > 0);
  }

  // ---------- drawing ----------

  function starPath(cx, cy, r, rot = 0, inner = 0.48) {
    ctx.beginPath();
    for (let i = 0; i < 10; i++) {
      const a = rot - Math.PI / 2 + (i * Math.PI) / 5;
      const rr = i % 2 ? r * inner : r;
      const px = cx + Math.cos(a) * rr;
      const py = cy + Math.sin(a) * rr;
      i ? ctx.lineTo(px, py) : ctx.moveTo(px, py);
    }
    ctx.closePath();
  }

  function flowerPath(cx, cy, r, rot = 0) {
    const n = 5;
    const step = (Math.PI * 2) / n;
    ctx.beginPath();
    for (let i = 0; i < n; i++) {
      const a = rot + i * step;
      const a0 = a - step / 2;
      const a1 = a + step / 2;
      const ir = r * 0.42;
      const or = r * 1.3;
      if (i === 0) ctx.moveTo(cx + Math.cos(a0) * ir, cy + Math.sin(a0) * ir);
      ctx.bezierCurveTo(
        cx + Math.cos(a - step * 0.42) * or,
        cy + Math.sin(a - step * 0.42) * or,
        cx + Math.cos(a + step * 0.42) * or,
        cy + Math.sin(a + step * 0.42) * or,
        cx + Math.cos(a1) * ir,
        cy + Math.sin(a1) * ir,
      );
    }
    ctx.closePath();
  }

  function smoothBlob(cx, cy, pts) {
    ctx.beginPath();
    const n = pts.length;
    const mid = (i) => {
      const a = pts[i % n];
      const b = pts[(i + 1) % n];
      return [(a[0] + b[0]) / 2, (a[1] + b[1]) / 2];
    };
    const m0 = mid(n - 1);
    ctx.moveTo(cx + m0[0], cy + m0[1]);
    for (let i = 0; i < n; i++) {
      const m = mid(i);
      ctx.quadraticCurveTo(cx + pts[i][0], cy + pts[i][1], cx + m[0], cy + m[1]);
    }
    ctx.closePath();
  }

  function drawBackground() {
    const sky = ctx.createLinearGradient(0, 0, 0, GROUND);
    sky.addColorStop(0, lerpColor(C.sky, C.skyNight, night));
    sky.addColorStop(1, lerpColor("#e9f1ff", C.skyNightLow, night));
    ctx.fillStyle = sky;
    ctx.fillRect(0, 0, W, H);

    // moon fades in at night
    if (night > 0.01) {
      ctx.globalAlpha = night;
      // crescent: clip away an offset circle so the sky shows through
      ctx.save();
      ctx.beginPath();
      ctx.rect(0, 0, W, H);
      ctx.arc(W - 82, 46, 15, 0, Math.PI * 2);
      ctx.clip("evenodd");
      ctx.fillStyle = C.moon;
      ctx.beginPath();
      ctx.arc(W - 90, 52, 18, 0, Math.PI * 2);
      ctx.fill();
      ctx.restore();
      ctx.globalAlpha = 1;
    }

    // cream clouds
    ctx.fillStyle = C.cloud;
    ctx.globalAlpha = 0.85 - night * 0.65;
    for (const cl of clouds) {
      const s = cl.s;
      ctx.beginPath();
      ctx.arc(cl.x, cl.y, 13 * s, 0, Math.PI * 2);
      ctx.arc(cl.x + 16 * s, cl.y - 7 * s, 16 * s, 0, Math.PI * 2);
      ctx.arc(cl.x + 34 * s, cl.y, 12 * s, 0, Math.PI * 2);
      ctx.rect(cl.x, cl.y, 34 * s, 12 * s);
      ctx.fill();
    }
    ctx.globalAlpha = 1;

    // navy doodles from the book cover (they glow gold at night)
    const doodleColor = lerpColor(C.doodle, C.doodleNight, night);
    ctx.strokeStyle = doodleColor;
    ctx.lineWidth = 1.6;
    ctx.lineJoin = "round";
    ctx.lineCap = "round";
    for (const d of doodles) {
      if (d.kind === "star") {
        starPath(d.x, d.y, d.size, d.rot);
        ctx.stroke();
      } else if (d.kind === "flower") {
        flowerPath(d.x, d.y, d.size * 0.8, d.rot);
        ctx.stroke();
      } else if (d.kind === "dot") {
        ctx.beginPath();
        ctx.ellipse(d.x, d.y, 3, 2.2, d.rot, 0, Math.PI * 2);
        ctx.stroke();
      } else {
        ctx.beginPath();
        for (let a = 0; a < Math.PI * 3.2; a += 0.2) {
          const r = 1 + a * 1.1;
          const px = d.x + Math.cos(a + d.rot) * r;
          const py = d.y + Math.sin(a + d.rot) * r;
          a ? ctx.lineTo(px, py) : ctx.moveTo(px, py);
        }
        ctx.stroke();
      }
    }
  }

  function drawGround() {
    const off = distance;
    ctx.fillStyle = lerpColor(C.ground, C.groundNight, night);
    ctx.beginPath();
    ctx.moveTo(0, H);
    for (let x = 0; x <= W; x += 8) {
      ctx.lineTo(x, GROUND + Math.sin((x + off) * 0.025) * 1.6);
    }
    ctx.lineTo(W, H);
    ctx.closePath();
    ctx.fill();

    ctx.strokeStyle = lerpColor(C.groundEdge, C.groundEdgeNight, night);
    ctx.lineWidth = 2;
    ctx.beginPath();
    for (let x = 0; x <= W; x += 8) {
      const y = GROUND + Math.sin((x + off) * 0.025) * 1.6;
      x ? ctx.lineTo(x, y) : ctx.moveTo(x, y);
    }
    ctx.stroke();

    // grass tufts and pebbles scrolling with the ground
    ctx.lineWidth = 1.4;
    const spacing = 46;
    const shift = off % spacing;
    for (let i = -1; i < W / spacing + 1; i++) {
      const x = i * spacing - shift;
      const seed = Math.floor((off + x) / spacing);
      const y = GROUND + 10 + ((seed * 7) % 17);
      if (seed % 3 === 0) {
        ctx.beginPath();
        ctx.moveTo(x - 3, y);
        ctx.lineTo(x, y - 5);
        ctx.lineTo(x + 3, y);
        ctx.stroke();
      } else {
        ctx.beginPath();
        ctx.ellipse(x, y, 2.5, 1.4, 0, 0, Math.PI * 2);
        ctx.stroke();
      }
    }
  }

  function drawObstacle(ob) {
    ctx.lineWidth = 2;
    ctx.lineJoin = "round";
    if (ob.kind === "rocks") {
      for (const p of ob.parts) {
        const cx = ob.x + p.dx + p.w / 2;
        const cy = GROUND - p.h / 2;
        smoothBlob(cx, cy, p.pts);
        ctx.fillStyle = C.rock;
        ctx.fill();
        ctx.strokeStyle = C.ink;
        ctx.stroke();
        ctx.fillStyle = C.rockShade;
        for (const [dx, dy, r] of p.craters) {
          ctx.beginPath();
          ctx.arc(cx + dx, cy + dy, r, 0, Math.PI * 2);
          ctx.fill();
        }
      }
    } else if (ob.kind === "bush") {
      const cx = ob.x + ob.w / 2;
      // stroke every puff first, then fill on top so only the outer outline shows
      ctx.strokeStyle = C.bushDark;
      ctx.lineWidth = 4;
      for (const [dx, dy, r] of ob.puffs) {
        ctx.beginPath();
        ctx.arc(cx + dx, GROUND + dy, r, 0, Math.PI * 2);
        ctx.stroke();
      }
      ctx.fillStyle = lerpColor(C.bush, "#79ad6b", night);
      for (const [dx, dy, r] of ob.puffs) {
        ctx.beginPath();
        ctx.arc(cx + dx, GROUND + dy, r, 0, Math.PI * 2);
        ctx.fill();
      }
      ctx.fillStyle = C.berry;
      for (const [dx, dy] of ob.berries) {
        starPath(cx + dx, GROUND + dy, 3.4, 0, 0.5);
        ctx.fill();
      }
    } else {
      const cx = ob.x + 17;
      const cy = ob.y + ob.h / 2;
      const trail = ctx.createLinearGradient(cx, 0, cx + 46, 0);
      trail.addColorStop(0, "rgba(255,210,141,0.9)");
      trail.addColorStop(1, "rgba(255,210,141,0)");
      ctx.fillStyle = trail;
      ctx.beginPath();
      ctx.moveTo(cx, cy - 8);
      ctx.quadraticCurveTo(cx + 30, cy - 6, cx + 48, cy);
      ctx.quadraticCurveTo(cx + 30, cy + 6, cx, cy + 8);
      ctx.fill();
      starPath(cx, cy, 13, Math.sin(ob.spin) * 0.25);
      ctx.fillStyle = C.title;
      ctx.fill();
      ctx.strokeStyle = C.titleShadow;
      ctx.lineWidth = 2;
      ctx.stroke();
      // a little face, looking ahead
      ctx.fillStyle = C.titleShadow;
      ctx.beginPath();
      ctx.arc(cx - 4, cy - 1, 1.5, 0, Math.PI * 2);
      ctx.arc(cx + 2, cy - 1, 1.5, 0, Math.PI * 2);
      ctx.fill();
    }
  }

  function drawPickups() {
    for (const p of pickups) {
      const y = p.y + Math.sin(clock * 5 + p.phase) * 3;
      starPath(p.x, y, 9, Math.sin(clock * 3 + p.phase) * 0.3);
      ctx.fillStyle = C.star;
      ctx.fill();
      ctx.strokeStyle = C.titleShadow;
      ctx.lineWidth = 1.6;
      ctx.stroke();
    }
  }

  function drawPlayer() {
    const { c, baseW, h, sx, sy } = playerMetrics();
    const img = images[c.id];
    const cx = PLAYER_X + baseW / 2;
    let rot = 0;
    let bob = 0;

    if (mode === "over") {
      rot = -0.22;
    } else if (!player.onGround) {
      rot = player.vy < 0 ? -0.1 : 0.06;
    } else if (mode === "running" && !player.ducking) {
      const step = Math.sin(clock * 16);
      bob = Math.abs(step) * 3;
      rot = step * 0.045;
    }

    // soft shadow shrinks as the character rises
    const lift = Math.max(0, GROUND - player.y);
    const shadowScale = Math.max(0.35, 1 - lift / 160);
    ctx.fillStyle = "rgba(37, 83, 63, 0.25)";
    ctx.beginPath();
    ctx.ellipse(cx + 4, GROUND + 2, baseW * 0.38 * shadowScale * sx, 4 * shadowScale, 0, 0, Math.PI * 2);
    ctx.fill();

    ctx.save();
    ctx.translate(cx, player.y - bob + c.sink * sy);
    ctx.rotate(rot);
    ctx.scale(sx * (c.flip ? -1 : 1), sy);
    if (img?.complete && img.naturalWidth) {
      ctx.drawImage(img, -baseW / 2, -h / sy, baseW, h / sy);
    } else {
      ctx.fillStyle = c.accent;
      ctx.beginPath();
      ctx.ellipse(0, -h / sy / 2, baseW / 2.4, h / sy / 2.2, 0, 0, Math.PI * 2);
      ctx.fill();
    }
    ctx.restore();

    if (mode === "over") {
      // dizzy stars
      for (let i = 0; i < 3; i++) {
        const a = clock * 4 + (i * Math.PI * 2) / 3;
        starPath(cx + Math.cos(a) * 18, player.y - h - 6 + Math.sin(a) * 5, 5);
        ctx.fillStyle = C.title;
        ctx.fill();
        ctx.strokeStyle = C.titleShadow;
        ctx.lineWidth = 1.2;
        ctx.stroke();
      }
    }
  }

  function drawEffects() {
    for (const p of particles) {
      ctx.globalAlpha = Math.max(0, p.life / p.max);
      ctx.fillStyle = p.color;
      if (p.star) {
        starPath(p.x, p.y, 4);
        ctx.fill();
      } else {
        ctx.beginPath();
        ctx.arc(p.x, p.y, 2, 0, Math.PI * 2);
        ctx.fill();
      }
    }
    ctx.globalAlpha = 1;
    ctx.font = "700 13px Nunito, system-ui, sans-serif";
    ctx.textAlign = "center";
    for (const p of popups) {
      ctx.globalAlpha = Math.max(0, p.life / 0.8);
      ctx.fillStyle = night > 0.5 ? C.title : C.titleShadow;
      ctx.fillText("+20", p.x, p.y);
    }
    ctx.globalAlpha = 1;
  }

  function drawScore() {
    const color = lerpColor(C.ink, "#fff6e3", night);
    ctx.font = "700 15px Nunito, system-ui, sans-serif";
    ctx.textAlign = "right";
    ctx.textBaseline = "alphabetic";
    const flashing = clock < flashUntil && Math.floor(clock * 8) % 2 === 0;
    ctx.fillStyle = color;
    if (!flashing) ctx.fillText(pad(score), W - 16, 26);
    ctx.globalAlpha = 0.6;
    ctx.fillText(`HI ${pad(hiScore)}`, W - 72, 26);
    ctx.globalAlpha = 1;
  }

  function titleText(text, x, y, size) {
    ctx.font = `700 ${size}px Nunito, system-ui, sans-serif`;
    ctx.textAlign = "center";
    ctx.fillStyle = C.titleShadow;
    ctx.fillText(text, x + 2, y + 3);
    ctx.fillStyle = C.title;
    ctx.fillText(text, x, y);
  }

  function bodyText(text, x, y, size = 15) {
    ctx.font = `700 ${size}px Nunito, system-ui, sans-serif`;
    ctx.textAlign = "center";
    ctx.fillStyle = lerpColor(C.doodle, "#fff6e3", night);
    ctx.fillText(text, x, y);
  }

  function drawOverlay() {
    const cx = W / 2 + 30;
    if (mode === "running") return;

    // soft panel so the doodles don't cross the text
    const pw = Math.min(W - 150, 440);
    ctx.fillStyle = lerpColor("#eef4ff", C.skyNight, night);
    ctx.globalAlpha = 0.82;
    ctx.beginPath();
    ctx.roundRect(cx - pw / 2, 48, pw, mode === "paused" ? 88 : 110, 18);
    ctx.fill();
    ctx.globalAlpha = 1;

    if (mode === "ready") {
      titleText("CROSBY'S COSMIC DASH", cx, 84, W < 600 ? 26 : 34);
      bodyText("press space or tap to start", cx, 116);
      ctx.globalAlpha = 0.75;
      bodyText("↑ / space to jump  ·  ↓ to duck", cx, 140, 13);
      ctx.globalAlpha = 1;
    } else if (mode === "over") {
      titleText("OH NO!", cx, 82, 34);
      bodyText(`${char().name} bumped into ${bumpedInto}`, cx, 112);
      ctx.globalAlpha = 0.75;
      bodyText("tap or press space to try again", cx, 136, 13);
      ctx.globalAlpha = 1;
    } else if (mode === "paused") {
      titleText("PAUSED", cx, 90, 30);
      bodyText("tap or press space to keep going", cx, 120, 14);
    }
  }

  function draw() {
    if (!ctx) return;
    drawBackground();
    drawGround();
    for (const ob of obstacles) drawObstacle(ob);
    drawPickups();
    drawPlayer();
    drawEffects();
    drawScore();
    drawOverlay();
  }

  // ---------- sizing & input ----------

  function resize() {
    if (!wrapper || !canvas) return;
    const cssW = wrapper.clientWidth;
    W = cssW < 560 ? 480 : 720;
    const dpr = window.devicePixelRatio || 1;
    canvas.width = Math.round(cssW * dpr);
    canvas.height = Math.round((cssW * dpr * H) / W);
    canvas.style.aspectRatio = `${W} / ${H}`;
    const scale = canvas.width / W;
    ctx = canvas.getContext("2d");
    ctx.setTransform(scale, 0, 0, scale, 0, 0);
    draw();
  }

  function handleKeyDown(event) {
    if (event.code === "Space" || event.code === "ArrowUp" || event.code === "KeyW" || event.code === "Enter") {
      event.preventDefault();
      if (!event.repeat) pressJump();
    } else if (event.code === "ArrowDown" || event.code === "KeyS") {
      event.preventDefault();
      setDuck(true);
    } else if (event.code === "KeyP" || event.code === "Escape") {
      if (mode === "running") pause();
    }
  }

  function handleKeyUp(event) {
    if (event.code === "Space" || event.code === "ArrowUp" || event.code === "KeyW" || event.code === "Enter") {
      releaseJump();
    } else if (event.code === "ArrowDown" || event.code === "KeyS") {
      setDuck(false);
    }
  }

  let touchStartY = null;

  function handlePointerDown(event) {
    if (event.button > 0) return;
    wrapper?.focus({ preventScroll: true });
    touchStartY = event.clientY;
    pressJump();
  }

  function handlePointerMove(event) {
    // swipe down to duck on touch screens
    if (touchStartY !== null && event.clientY - touchStartY > 24) {
      releaseJump();
      setDuck(true);
    }
  }

  function handlePointerUp() {
    touchStartY = null;
    releaseJump();
    setDuck(false);
  }

  function selectCharacter(id) {
    if (mode === "running") return;
    selectedId = id;
    if (mode === "over" || mode === "paused") {
      reset();
      mode = "ready";
      uiState = "ready";
    }
    draw();
    wrapper?.focus({ preventScroll: true });
  }

  onMount(() => {
    loadHiScore();
    reset();
    for (const c of characters) {
      const img = new Image();
      img.decoding = "async";
      img.onload = () => draw();
      img.src = c.src;
      images[c.id] = img;
    }

    resize();
    const ro = new ResizeObserver(() => resize());
    ro.observe(wrapper);

    const onVisibility = () => {
      if (document.hidden) pause();
    };
    document.addEventListener("visibilitychange", onVisibility);

    // Redraw once web fonts are ready so canvas text uses Nunito.
    document.fonts?.ready.then(() => draw());

    return () => {
      cancelAnimationFrame(rafId);
      ro.disconnect();
      document.removeEventListener("visibilitychange", onVisibility);
    };
  });
</script>

<div class="flex w-full flex-col items-center gap-4">
  <div class="flex flex-wrap items-center justify-center gap-3" role="group" aria-label="Choose your character">
    {#each characters as c (c.id)}
      <button
        type="button"
        class="character-pick flex items-center gap-2 rounded-full border-2 bg-white py-1 pl-1 pr-4 text-sm font-bold lowercase text-[#352f47] shadow-[0_6px_14px_rgba(53,47,71,0.16)] disabled:cursor-not-allowed disabled:opacity-60"
        style="border-color: {selectedId === c.id ? c.accent : 'transparent'};"
        aria-pressed={selectedId === c.id}
        disabled={uiState === "running"}
        onclick={() => selectCharacter(c.id)}
      >
        <span
          class="grid h-10 w-10 place-items-center overflow-hidden rounded-full"
          style="background-color: {c.accent}33;"
        >
          <img src={c.src} alt="" class="h-9 w-auto" style={c.flip ? "transform: scaleX(-1)" : ""} />
        </span>
        {c.name}
      </button>
    {/each}
  </div>

  <!-- svelte-ignore a11y_no_noninteractive_tabindex, a11y_no_noninteractive_element_interactions -->
  <div
    bind:this={wrapper}
    class="game-frame w-full overflow-hidden rounded-[1.25rem] border-4 border-white bg-[#d7e6ff] shadow-[0_10px_24px_rgba(53,47,71,0.22)]"
    tabindex="0"
    role="application"
    aria-label="Crosby's Cosmic Dash game. Press space or up arrow to jump, down arrow to duck, P to pause."
    aria-describedby="cosmic-dash-help"
    onkeydown={handleKeyDown}
    onkeyup={handleKeyUp}
    onblur={pause}
  >
    <canvas
      bind:this={canvas}
      class="block w-full select-none"
      style="touch-action: {uiState === 'running' ? 'none' : 'manipulation'};"
      onpointerdown={handlePointerDown}
      onpointermove={handlePointerMove}
      onpointerup={handlePointerUp}
      onpointercancel={handlePointerUp}
      oncontextmenu={(e) => e.preventDefault()}
    ></canvas>
  </div>

  <p id="cosmic-dash-help" class="text-sm text-[#4c465f]">
    Space / ↑ to jump (hold for higher) · ↓ to duck · on phones, tap to jump and swipe down to duck
  </p>
  <p class="sr-only" aria-live="polite">{announcement}</p>
</div>

<style>
  .game-frame:focus-visible {
    outline: 3px solid #d2234c;
    outline-offset: 3px;
  }

  .character-pick:not(:disabled):hover {
    transform: translateY(-2px);
  }

  /* the site-wide hover zoom on button images would jostle the portraits */
  .character-pick :global(img:hover) {
    transform: none;
  }
</style>
