<script>
  import { onMount } from "svelte";
  import crosbySprite from "#lib/assets/game/crosby.webp";
  import carlaSprite from "#lib/assets/game/carla.webp";
  import tankSprite from "#lib/assets/game/tank.webp";

  // Endless runner in the spirit of Chrome's offline dino game, set in the
  // starry world of Crosby's Cosmic Adventure. Each friend has a passive perk
  // and a special move charged by collecting stars — Crosby's are the strongest.

  const characters = [
    {
      id: "crosby",
      name: "Crosby",
      role: "Gentle dreamer",
      hero: true,
      power: 5,
      src: crosbySprite,
      aspect: 288 / 240,
      height: 72,
      flip: false,
      hit: { l: 0.3, r: 0.1, t: 0.1, b: 0.03 },
      accent: "#7dbd55",
      jumps: 2,
      holdTime: 0.17,
      holdGravity: 0.55,
      magnet: 0,
      map: false,
      passive: {
        name: "Starlight Double Jump",
        text: "Leap again in mid-air on a burst of starlight.",
      },
      special: {
        kind: "brave",
        name: "Brave Heart",
        cost: 6,
        duration: 5,
        text: "5 seconds of glowing courage: smash through anything, pull in every star and score double.",
      },
    },
    {
      id: "carla",
      name: "Carla",
      role: "Fearless navigator",
      power: 3,
      src: carlaSprite,
      aspect: 172 / 240,
      height: 70,
      flip: true,
      hit: { l: 0.14, r: 0.14, t: 0.06, b: 0.03 },
      accent: "#d68a4c",
      jumps: 1,
      holdTime: 0.17,
      holdGravity: 0.55,
      magnet: 80,
      map: true,
      passive: {
        name: "Navigator's Map",
        text: "Spots what's coming before it's on screen, and nearby stars drift her way.",
      },
      special: {
        kind: "calm",
        name: "Calm Compass",
        cost: 9,
        duration: 4,
        text: "She sets a steady pace: the whole galaxy slows down for 4 seconds.",
      },
    },
    {
      id: "tank",
      name: "Tank",
      role: "Inventor and tinkerer",
      power: 4,
      src: tankSprite,
      aspect: 211 / 240,
      height: 74,
      flip: false,
      hit: { l: 0.1, r: 0.25, t: 0.05, b: 0.03 },
      accent: "#e8a24a",
      jumps: 1,
      holdTime: 0.2,
      holdGravity: 0.45,
      magnet: 0,
      map: false,
      passive: {
        name: "Spring Gadget",
        text: "Hold jump to bounce extra high.",
      },
      special: {
        kind: "rocket",
        name: "Rocket Pack",
        cost: 9,
        duration: 3,
        text: "Blast off and cruise safely over everything for 3 seconds.",
      },
    },
  ];

  // Palette drawn from the book cover and the nighttime spreads.
  const C = {
    skyTop: "#120f33",
    skyMid: "#2b2466",
    skyLow: "#4f3a8c",
    title: "#f3d36f",
    titleShadow: "#7c2b28",
    ink: "#241d4a",
    text: "#fff6e3",
    gold: "#ffd28d",
    ground1: "#a99ad8",
    ground2: "#7d6cb8",
    ground3: "#4a3d86",
    groundEdge: "#e9e1ff",
    crater: "#6a5aa6",
    craterRim: "#c4b8ee",
    rock: "#8e83bd",
    rockShade: "#6f63a3",
    rockLight: "#c9bff0",
    navigator: "#5fe0d0",
  };

  // Each zone tints the nebulae; a new zone every ZONE_EVERY points.
  const zones = [
    { name: "Purple Nebula", a: "#ff8fc8", b: "#8f6bff" },
    { name: "Teal Galaxy", a: "#5fe0d0", b: "#3a7bd5" },
    { name: "Rosy Starfield", a: "#ffb4a0", b: "#ff6fa0" },
    { name: "Golden Comet Belt", a: "#ffd28d", b: "#f09a58" },
  ];
  const ZONE_EVERY = 500;

  const H = 280;
  const GROUND = 236;
  const PLAYER_X = 48;
  const GRAVITY = 2600;
  const JUMP_V = -700;
  const AIR_JUMP_V = -640;
  const FAST_FALL = 2.4;
  const DUCK_SX = 1.18;
  const DUCK_SY = 0.6;
  const START_SPEED = 340;
  const MAX_SPEED = 760;
  const ACCEL = 9;
  const SPAWN_AHEAD = 260;
  const GRACE = 0.9;
  const ROCKET_Y = GROUND - 128;
  const HI_KEY = "catchupwithalice-cosmic-dash-hi";

  let wrapper = $state();
  let canvas = $state();
  let selectedId = $state("crosby");
  let uiState = $state("ready");
  let uiCharge = $state(0);
  let uiActive = $state(false);
  let announcement = $state("");

  const selected = $derived(characters.find((c) => c.id === selectedId) ?? characters[0]);

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
  let nextSpawn = 0;
  let overLockUntil = 0;
  let flashUntil = 0;
  let lastMilestone = 0;
  let bumpedInto = "";
  let charge = 0;
  let active = { kind: null, t: 0, max: 0 };
  let grace = 0;
  let zoneIndex = 0;
  let zoneBannerT = 0;
  let nebA = [255, 143, 200];
  let nebB = [143, 107, 255];
  const player = {
    y: GROUND,
    vy: 0,
    onGround: true,
    holdT: 0,
    held: false,
    ducking: false,
    jumpsUsed: 0,
  };
  let obstacles = [];
  let pickups = [];
  let particles = [];
  let popups = [];
  let starLayers = [];
  let goldStars = [];
  let nebulae = [];
  let planets = [];
  let shooting = null;
  let nextShoot = 3;

  const char = () => selected;
  const rand = (a, b) => a + Math.random() * (b - a);
  const pad = (n) => String(Math.floor(n)).padStart(5, "0");

  const hexToRgb = (hex) => {
    const n = parseInt(hex.slice(1), 16);
    return [(n >> 16) & 255, (n >> 8) & 255, n & 255];
  };
  const rgba = ([r, g, b], a = 1) => `rgba(${r | 0},${g | 0},${b | 0},${a})`;
  const approach = (cur, target, k) => cur.map((v, i) => v + (target[i] - v) * k);

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

  function syncUi() {
    uiCharge = charge;
    uiActive = !!active.kind;
  }

  // ---------- world setup ----------

  function seedScenery() {
    starLayers = [0.03, 0.07, 0.13].map((par, i) => ({
      par,
      stars: Array.from({ length: 46 - i * 12 }, () => ({
        x: rand(0, 1000),
        y: rand(4, GROUND - 14),
        r: rand(0.5, 1) + i * 0.35,
        tw: rand(1.2, 3.2),
        ph: rand(0, 6.28),
      })),
    }));
    goldStars = Array.from({ length: 7 }, () => ({
      x: rand(0, 1000),
      y: rand(14, GROUND - 70),
      r: rand(3.5, 6),
      ph: rand(0, 6.28),
    }));
    nebulae = [
      { x: 120, y: 70, r: 150, c: "a" },
      { x: 480, y: 130, r: 180, c: "b" },
      { x: 820, y: 60, r: 140, c: "a" },
    ];
    planets = [
      { x: 560, y: 66, r: 20, kind: "ringed" },
      { x: 960, y: 120, r: 11, kind: "moon" },
      { x: 260, y: 150, r: 8, kind: "blue" },
    ];
    shooting = null;
    nextShoot = rand(2, 4);
  }

  function reset() {
    speed = START_SPEED;
    score = 0;
    distance = 0;
    clock = 0;
    nextSpawn = 200;
    lastMilestone = 0;
    flashUntil = 0;
    bumpedInto = "";
    charge = 0;
    active = { kind: null, t: 0, max: 0 };
    grace = 0;
    zoneIndex = 0;
    zoneBannerT = 0;
    nebA = hexToRgb(zones[0].a);
    nebB = hexToRgb(zones[0].b);
    Object.assign(player, {
      y: GROUND,
      vy: 0,
      onGround: true,
      holdT: 0,
      held: false,
      ducking: false,
      jumpsUsed: 0,
    });
    obstacles = [];
    pickups = [];
    particles = [];
    popups = [];
    seedScenery();
    syncUi();
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
    const x = W + SPAWN_AHEAD;
    const r = Math.random();
    let ob;

    if (score > 150 && r < 0.25) {
      const lanes = [GROUND - 24, GROUND - 60, GROUND - 106];
      const cy = lanes[Math.floor(Math.random() * lanes.length)];
      ob = {
        kind: "comet",
        label: "a shooting comet",
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
      const count = Math.random() < 0.5 ? 2 : 3;
      const hue = Math.random() < 0.5 ? ["#8ff3ea", "#3fb8c9"] : ["#ffc2e6", "#e66aae"];
      const shards = [];
      let dx = 0;
      let maxH = 0;
      for (let i = 0; i < count; i++) {
        const w = rand(12, 16);
        const h = i === 1 ? rand(36, 46) : rand(20, 30);
        shards.push({ dx: dx + w / 2, w, h, tilt: (i - (count - 1) / 2) * rand(3, 6) });
        dx += w - 3;
        maxH = Math.max(maxH, h);
      }
      const w = dx + 3;
      ob = {
        kind: "crystal",
        label: "a space crystal",
        x,
        y: GROUND - maxH,
        w,
        h: maxH,
        hit: { x: 4, y: 6, w: w - 8, h: maxH - 6 },
        shards,
        hue,
      };
    }

    obstacles.push(ob);
    nextSpawn = ob.w + speed * rand(0.78, 1.55) + 60;

    // Reward a well-timed jump: sometimes float a star over ground obstacles.
    if (ob.kind !== "comet" && Math.random() < 0.32) {
      pickups.push({ x: ob.x + ob.w / 2, y: ob.y - rand(42, 62), phase: rand(0, 6) });
    }
    // ...and sometimes a little row of stars in the gap before the next one.
    if (Math.random() < 0.16) {
      const mid = ob.x + ob.w + (nextSpawn - ob.w) / 2;
      for (let i = -1; i <= 1; i++) {
        pickups.push({ x: mid + i * 26, y: GROUND - 34, phase: i });
      }
    }
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
    active = { kind: null, t: 0, max: 0 };
    syncUi();
    announcement = `${char().name} bumped into ${bumpedInto}. Score ${Math.floor(score)}.`;
    burst(PLAYER_X + 30, player.y - 40, 10, [C.title, C.text], 120);
    // let the burst play out briefly
    const burstEnd = performance.now() + 700;
    const tail = (now) => {
      // rAF timestamps can trail performance.now(), so never step backwards
      const dt = Math.max(0, Math.min(0.05, (now - lastTime) / 1000));
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
    if (active.kind === "rocket" || player.ducking) return;
    const c = char();
    if (player.onGround) {
      player.vy = JUMP_V;
      player.onGround = false;
      player.holdT = 0;
      player.jumpsUsed = 1;
    } else if (player.jumpsUsed < c.jumps) {
      // Crosby's starlight double jump
      player.vy = AIR_JUMP_V;
      player.holdT = c.holdTime;
      player.jumpsUsed++;
      burst(PLAYER_X + 34, player.y - 4, 8, [C.title, C.gold, C.text], 140);
    }
  }

  function releaseJump() {
    player.held = false;
  }

  function setDuck(on) {
    if (mode !== "running") return;
    player.ducking = on;
  }

  function useSpecial() {
    if (mode === "paused") return resume();
    if (mode !== "running" || active.kind) return;
    const c = char();
    if (charge < c.special.cost) return;
    charge = 0;
    active = { kind: c.special.kind, t: c.special.duration, max: c.special.duration };
    if (active.kind === "rocket") {
      player.onGround = false;
      player.vy = 0;
      player.ducking = false;
    }
    burst(PLAYER_X + 34, player.y - 36, 14, [c.accent, C.title, C.text], 180);
    announcement = `${c.special.name}!`;
    syncUi();
  }

  // ---------- update ----------

  function frame(now) {
    if (mode !== "running") return;
    // rAF timestamps can trail performance.now(), so never step backwards
    const dt = Math.max(0, Math.min(0.05, (now - lastTime) / 1000));
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

  function burst(x, y, n, colors, power) {
    for (let i = 0; i < n; i++) {
      const a = (i / n) * Math.PI * 2 + rand(-0.2, 0.2);
      particles.push({
        x,
        y,
        vx: Math.cos(a) * power * rand(0.6, 1),
        vy: Math.sin(a) * power * rand(0.6, 1) - 40,
        life: 0.55,
        max: 0.55,
        color: colors[i % colors.length],
        star: true,
      });
    }
  }

  function updatePlayer(dt) {
    const c = char();

    if (active.kind === "rocket") {
      player.y += (ROCKET_Y - player.y) * Math.min(1, dt * 5);
      player.vy = 0;
      player.onGround = false;
      if (Math.random() < 0.8) {
        particles.push({
          x: PLAYER_X + 14,
          y: player.y - 18,
          vx: rand(-160, -90),
          vy: rand(40, 110),
          life: 0.4,
          max: 0.4,
          color: Math.random() < 0.5 ? "#ffb347" : C.title,
        });
      }
      return;
    }

    if (!player.onGround) {
      let g = GRAVITY;
      if (player.held && player.vy < 0 && player.holdT < c.holdTime) {
        g *= c.holdGravity;
        player.holdT += dt;
      }
      if (player.ducking) g *= FAST_FALL;
      player.vy += g * dt;
      player.y += player.vy * dt;

      // keep the character on screen
      const ceiling = c.height + 6;
      if (player.y < ceiling) {
        player.y = ceiling;
        player.vy = Math.max(player.vy, 0);
      }

      if (player.y >= GROUND) {
        player.y = GROUND;
        player.vy = 0;
        player.onGround = true;
        player.jumpsUsed = 0;
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
  }

  function updateScenery(move, dt) {
    for (const p of planets) {
      if (p.x - distance * 0.05 < -60) p.x += 1100 + rand(0, 300);
    }
    if (shooting) {
      shooting.t += dt;
      if (shooting.t > shooting.life) shooting = null;
    } else {
      nextShoot -= dt;
      if (nextShoot <= 0) {
        shooting = {
          x: rand(W * 0.3, W + 40),
          y: rand(10, 80),
          vx: rand(-420, -320),
          vy: rand(120, 180),
          t: 0,
          life: 0.9,
        };
        nextShoot = rand(3, 7);
      }
    }
    const z = zones[zoneIndex];
    nebA = approach(nebA, hexToRgb(z.a), Math.min(1, dt * 0.8));
    nebB = approach(nebB, hexToRgb(z.b), Math.min(1, dt * 0.8));
    zoneBannerT = Math.max(0, zoneBannerT - dt);
  }

  function update(dt) {
    const c = char();
    clock += dt;

    if (active.kind) {
      active.t -= dt;
      if (active.t <= 0) {
        if (active.kind === "rocket") player.held = false;
        active = { kind: null, t: 0, max: 0 };
        grace = GRACE;
        syncUi();
      }
    }
    grace = Math.max(0, grace - dt);

    const timeScale = active.kind === "calm" ? 0.55 : 1;
    const wdt = dt * timeScale;
    speed = Math.min(MAX_SPEED, speed + ACCEL * wdt);
    const move = speed * wdt;
    distance += move;
    const mult = active.kind === "brave" ? 2 : 1;
    score += move * 0.025 * mult;

    const milestone = Math.floor(score / 100);
    if (milestone > lastMilestone) {
      lastMilestone = milestone;
      flashUntil = clock + 0.9;
    }

    const z = Math.max(0, Math.floor(score / ZONE_EVERY)) % zones.length;
    if (z !== zoneIndex) {
      zoneIndex = z;
      zoneBannerT = 2.6;
    }

    updatePlayer(dt);
    updateScenery(move, dt);

    // obstacles
    nextSpawn -= move;
    if (nextSpawn <= 0) spawnObstacle();

    const pb = playerBox();
    const invulnerable = active.kind === "brave" || active.kind === "rocket" || grace > 0;
    for (const ob of obstacles) {
      ob.x -= move + (ob.extra ?? 0) * wdt;
      if (ob.kind === "comet") ob.spin += dt * 5;
      if (ob.smashed) continue;
      const hb = { x: ob.x + ob.hit.x, y: ob.y + ob.hit.y, w: ob.hit.w, h: ob.hit.h };
      if (!boxesOverlap(pb, hb)) continue;
      if (active.kind === "brave") {
        ob.smashed = true;
        score += 25 * mult;
        popups.push({ x: ob.x + ob.w / 2, y: ob.y - 8, life: 0.8, text: `+${25 * mult}` });
        burst(ob.x + ob.w / 2, ob.y + ob.h / 2, 12, [C.title, C.rockLight, "#ff9ad5"], 200);
      } else if (!invulnerable) {
        bumpedInto = ob.label;
        gameOver();
        return;
      }
    }
    obstacles = obstacles.filter((ob) => !ob.smashed && ob.x + ob.w > -20);

    // stars
    let magnet = c.magnet;
    if (active.kind === "brave") magnet = 220;
    if (active.kind === "rocket") magnet = 150;
    const pcx = pb.x + pb.w / 2;
    const pcy = pb.y + pb.h / 2;
    for (const p of pickups) {
      p.x -= move;
      if (magnet) {
        const dx = pcx - p.x;
        const dy = pcy - p.y;
        const d = Math.hypot(dx, dy);
        if (d < magnet && d > 1) {
          const pull = (1 - d / magnet) * 900 * dt + 120 * dt;
          p.x += (dx / d) * pull;
          p.y += (dy / d) * pull;
        }
      }
      if (!p.taken && boxesOverlap(pb, { x: p.x - 10, y: p.y - 10, w: 20, h: 20 })) {
        p.taken = true;
        score += 20 * mult;
        if (!active.kind && charge < c.special.cost) {
          charge++;
          syncUi();
        }
        popups.push({ x: p.x, y: p.y - 12, life: 0.8, text: `+${20 * mult}` });
        burst(p.x, p.y, 8, [C.title], 110);
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

  // parallax position that wraps around a strip wider than the canvas
  function wrapX(x, par, span = Math.max(W + 80, 1000)) {
    return ((((x - distance * par) % span) + span) % span) - 40;
  }

  function drawPlanet(p) {
    const x = p.x - distance * 0.05;
    if (x < -60 || x > W + 60) return;
    const { r } = p;
    if (p.kind === "ringed") {
      ctx.strokeStyle = "rgba(255,210,141,0.75)";
      ctx.lineWidth = 3;
      ctx.beginPath();
      ctx.ellipse(x, p.y, r * 1.9, r * 0.5, -0.3, Math.PI, Math.PI * 2);
      ctx.stroke();
      const g = ctx.createRadialGradient(x - r * 0.4, p.y - r * 0.4, r * 0.2, x, p.y, r);
      g.addColorStop(0, "#ffc4dd");
      g.addColorStop(1, "#b06ab3");
      ctx.fillStyle = g;
      ctx.beginPath();
      ctx.arc(x, p.y, r, 0, Math.PI * 2);
      ctx.fill();
      ctx.beginPath();
      ctx.ellipse(x, p.y, r * 1.9, r * 0.5, -0.3, 0, Math.PI);
      ctx.stroke();
    } else if (p.kind === "moon") {
      ctx.fillStyle = "#d9d2f0";
      ctx.beginPath();
      ctx.arc(x, p.y, r, 0, Math.PI * 2);
      ctx.fill();
      ctx.fillStyle = "#b5abd8";
      ctx.beginPath();
      ctx.arc(x - r * 0.3, p.y - r * 0.2, r * 0.28, 0, Math.PI * 2);
      ctx.arc(x + r * 0.35, p.y + r * 0.3, r * 0.2, 0, Math.PI * 2);
      ctx.fill();
    } else {
      const g = ctx.createRadialGradient(x - r * 0.4, p.y - r * 0.4, r * 0.1, x, p.y, r);
      g.addColorStop(0, "#c7e9ff");
      g.addColorStop(1, "#3a7bd5");
      ctx.fillStyle = g;
      ctx.beginPath();
      ctx.arc(x, p.y, r, 0, Math.PI * 2);
      ctx.fill();
    }
  }

  function drawBackground() {
    const sky = ctx.createLinearGradient(0, 0, 0, GROUND);
    sky.addColorStop(0, C.skyTop);
    sky.addColorStop(0.55, C.skyMid);
    sky.addColorStop(1, C.skyLow);
    ctx.fillStyle = sky;
    ctx.fillRect(0, 0, W, H);

    // nebulae glow in the current zone's colors
    for (const n of nebulae) {
      const x = wrapX(n.x, 0.02);
      const col = n.c === "a" ? nebA : nebB;
      const g = ctx.createRadialGradient(x, n.y, 0, x, n.y, n.r);
      g.addColorStop(0, rgba(col, 0.32));
      g.addColorStop(1, rgba(col, 0));
      ctx.fillStyle = g;
      ctx.fillRect(x - n.r, n.y - n.r, n.r * 2, n.r * 2);
    }
    const horizon = ctx.createLinearGradient(0, GROUND - 70, 0, GROUND);
    horizon.addColorStop(0, rgba(nebB, 0));
    horizon.addColorStop(1, rgba(nebB, 0.28));
    ctx.fillStyle = horizon;
    ctx.fillRect(0, GROUND - 70, W, 70);

    // twinkling star field, three parallax layers
    for (const layer of starLayers) {
      for (const s of layer.stars) {
        const x = wrapX(s.x, layer.par);
        if (x < -4 || x > W + 4) continue;
        ctx.globalAlpha = 0.45 + 0.55 * Math.abs(Math.sin(clock * s.tw + s.ph));
        ctx.fillStyle = "#ffffff";
        ctx.beginPath();
        ctx.arc(x, s.y, s.r, 0, Math.PI * 2);
        ctx.fill();
      }
    }
    ctx.globalAlpha = 1;

    for (const p of planets) drawPlanet(p);

    // golden storybook stars
    for (const s of goldStars) {
      const x = wrapX(s.x, 0.09);
      if (x < -10 || x > W + 10) continue;
      const pulse = 0.8 + 0.2 * Math.sin(clock * 2 + s.ph);
      ctx.shadowColor = C.gold;
      ctx.shadowBlur = 8;
      starPath(x, s.y, s.r * pulse, s.ph);
      ctx.fillStyle = C.title;
      ctx.fill();
      ctx.shadowBlur = 0;
    }

    if (shooting) {
      const k = shooting.t;
      const x = shooting.x + shooting.vx * k;
      const y = shooting.y + shooting.vy * k;
      const a = 1 - k / shooting.life;
      const g = ctx.createLinearGradient(x, y, x - shooting.vx * 0.18, y - shooting.vy * 0.18);
      g.addColorStop(0, `rgba(255,255,255,${a})`);
      g.addColorStop(1, "rgba(255,255,255,0)");
      ctx.strokeStyle = g;
      ctx.lineWidth = 2;
      ctx.beginPath();
      ctx.moveTo(x, y);
      ctx.lineTo(x - shooting.vx * 0.18, y - shooting.vy * 0.18);
      ctx.stroke();
    }
  }

  function groundY(x) {
    const t = x + distance;
    return GROUND + Math.sin(t * 0.02) * 1.5 + Math.sin(t * 0.007) * 1.5;
  }

  function drawGround() {
    const g = ctx.createLinearGradient(0, GROUND, 0, H);
    g.addColorStop(0, C.ground1);
    g.addColorStop(0.35, C.ground2);
    g.addColorStop(1, C.ground3);
    ctx.fillStyle = g;
    ctx.beginPath();
    ctx.moveTo(0, H);
    for (let x = 0; x <= W; x += 8) ctx.lineTo(x, groundY(x));
    ctx.lineTo(W, H);
    ctx.closePath();
    ctx.fill();

    ctx.strokeStyle = C.groundEdge;
    ctx.lineWidth = 1.5;
    ctx.beginPath();
    for (let x = 0; x <= W; x += 8) {
      x ? ctx.lineTo(x, groundY(x)) : ctx.moveTo(x, groundY(x));
    }
    ctx.stroke();

    // craters and moon dust scrolling with the ground
    const spacing = 64;
    const shift = distance % spacing;
    for (let i = -1; i < W / spacing + 2; i++) {
      const x = i * spacing - shift;
      const seed = Math.floor((distance + x) / spacing);
      const y = GROUND + 12 + ((seed * 13) % 20);
      const rx = 5 + (seed % 3) * 4;
      if (seed % 2 === 0) {
        ctx.fillStyle = C.crater;
        ctx.beginPath();
        ctx.ellipse(x, y, rx, rx * 0.35, 0, 0, Math.PI * 2);
        ctx.fill();
        ctx.strokeStyle = C.craterRim;
        ctx.lineWidth = 1;
        ctx.beginPath();
        ctx.ellipse(x, y + 0.6, rx, rx * 0.35, 0, 0.1, Math.PI - 0.1);
        ctx.stroke();
      } else {
        ctx.fillStyle = C.craterRim;
        ctx.globalAlpha = 0.7;
        ctx.beginPath();
        ctx.arc(x, y, 1.2, 0, Math.PI * 2);
        ctx.arc(x + 9, y + 5, 0.9, 0, Math.PI * 2);
        ctx.fill();
        ctx.globalAlpha = 1;
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
        ctx.strokeStyle = C.rockLight;
        ctx.lineWidth = 1.5;
        ctx.beginPath();
        ctx.arc(cx, cy, Math.min(p.w, p.h) * 0.32, Math.PI * 1.1, Math.PI * 1.45);
        ctx.stroke();
        ctx.lineWidth = 2;
      }
    } else if (ob.kind === "crystal") {
      const [light, dark] = ob.hue;
      ctx.shadowColor = light;
      ctx.shadowBlur = 12;
      for (const s of ob.shards) {
        const bx = ob.x + s.dx;
        const t = s.tilt;
        ctx.beginPath();
        ctx.moveTo(bx - s.w / 2, GROUND);
        ctx.lineTo(bx - s.w / 2 + t * 0.7, GROUND - s.h * 0.72);
        ctx.lineTo(bx + t, GROUND - s.h);
        ctx.lineTo(bx + s.w / 2 + t * 0.7, GROUND - s.h * 0.72);
        ctx.lineTo(bx + s.w / 2, GROUND);
        ctx.closePath();
        const g = ctx.createLinearGradient(0, GROUND - s.h, 0, GROUND);
        g.addColorStop(0, light);
        g.addColorStop(1, dark);
        ctx.fillStyle = g;
        ctx.fill();
        ctx.strokeStyle = C.ink;
        ctx.stroke();
      }
      ctx.shadowBlur = 0;
      ctx.strokeStyle = "rgba(255,255,255,0.7)";
      ctx.lineWidth = 1.2;
      for (const s of ob.shards) {
        const bx = ob.x + s.dx;
        ctx.beginPath();
        ctx.moveTo(bx + s.tilt, GROUND - s.h + 4);
        ctx.lineTo(bx + s.tilt * 0.4, GROUND - 4);
        ctx.stroke();
      }
    } else {
      const cx = ob.x + 17;
      const cy = ob.y + ob.h / 2;
      const trail = ctx.createLinearGradient(cx, 0, cx + 56, 0);
      trail.addColorStop(0, "rgba(255,210,141,0.95)");
      trail.addColorStop(0.5, "rgba(255,143,200,0.5)");
      trail.addColorStop(1, "rgba(143,107,255,0)");
      ctx.fillStyle = trail;
      ctx.beginPath();
      ctx.moveTo(cx, cy - 9);
      ctx.quadraticCurveTo(cx + 34, cy - 7, cx + 58, cy);
      ctx.quadraticCurveTo(cx + 34, cy + 7, cx, cy + 9);
      ctx.fill();
      ctx.shadowColor = C.gold;
      ctx.shadowBlur = 14;
      starPath(cx, cy, 13, Math.sin(ob.spin) * 0.25);
      ctx.fillStyle = C.title;
      ctx.fill();
      ctx.shadowBlur = 0;
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

  // Carla's Navigator's Map: markers for things still off screen
  function drawNavigatorMarkers() {
    if (!char().map || mode !== "running") return;
    for (const ob of obstacles) {
      if (ob.x <= W - 4) continue;
      const closeness = 1 - Math.min(1, (ob.x - W) / SPAWN_AHEAD);
      const y = ob.y + ob.h / 2;
      ctx.globalAlpha = 0.35 + 0.65 * closeness;
      ctx.fillStyle = C.navigator;
      ctx.beginPath();
      ctx.moveTo(W - 22, y);
      ctx.lineTo(W - 8, y - 8);
      ctx.lineTo(W - 8, y + 8);
      ctx.closePath();
      ctx.fill();
      ctx.fillStyle = C.ink;
      ctx.font = "700 10px Nunito, system-ui, sans-serif";
      ctx.textAlign = "center";
      ctx.fillText("!", W - 12, y + 3.5);
    }
    ctx.globalAlpha = 1;
  }

  function drawPickups() {
    ctx.shadowColor = C.gold;
    ctx.shadowBlur = 10;
    for (const p of pickups) {
      const y = p.y + Math.sin(clock * 5 + p.phase) * 3;
      starPath(p.x, y, 9, Math.sin(clock * 3 + p.phase) * 0.3);
      ctx.fillStyle = C.gold;
      ctx.fill();
      ctx.strokeStyle = C.titleShadow;
      ctx.lineWidth = 1.6;
      ctx.stroke();
    }
    ctx.shadowBlur = 0;
  }

  function drawPlayer() {
    const { c, baseW, h, sx, sy } = playerMetrics();
    const img = images[c.id];
    const cx = PLAYER_X + baseW / 2;
    let rot = 0;
    let bob = 0;

    if (mode === "over") {
      rot = -0.22;
    } else if (active.kind === "rocket") {
      rot = -0.12 + Math.sin(clock * 20) * 0.03;
    } else if (!player.onGround) {
      rot = player.vy < 0 ? -0.1 : 0.06;
    } else if (mode === "running" && !player.ducking) {
      const step = Math.sin(clock * 16);
      bob = Math.abs(step) * 3;
      rot = step * 0.045;
    }

    // soft shadow shrinks as the character rises
    const lift = Math.max(0, GROUND - player.y);
    const shadowScale = Math.max(0.3, 1 - lift / 170);
    ctx.fillStyle = "rgba(36, 29, 74, 0.35)";
    ctx.beginPath();
    ctx.ellipse(cx + 4, GROUND + 2, baseW * 0.36 * shadowScale * sx, 4 * shadowScale, 0, 0, Math.PI * 2);
    ctx.fill();

    const midY = player.y - h / 2;

    if (active.kind === "brave") {
      const pr = 46 + Math.sin(clock * 10) * 4;
      const g = ctx.createRadialGradient(cx, midY, 4, cx, midY, pr);
      g.addColorStop(0, "rgba(255,226,140,0.75)");
      g.addColorStop(0.6, "rgba(255,190,90,0.3)");
      g.addColorStop(1, "rgba(255,190,90,0)");
      ctx.fillStyle = g;
      ctx.beginPath();
      ctx.arc(cx, midY, pr, 0, Math.PI * 2);
      ctx.fill();
      if (Math.random() < 0.5) {
        particles.push({
          x: cx + rand(-20, 10),
          y: midY + rand(-25, 25),
          vx: rand(-200, -120),
          vy: rand(-30, 30),
          life: 0.4,
          max: 0.4,
          color: C.title,
          star: true,
        });
      }
    } else if (active.kind === "calm") {
      ctx.strokeStyle = C.navigator;
      ctx.lineWidth = 2;
      for (let i = 0; i < 2; i++) {
        const t = (clock * 0.9 + i * 0.5) % 1;
        ctx.globalAlpha = 1 - t;
        ctx.beginPath();
        ctx.ellipse(cx, midY, 20 + t * 50, 14 + t * 34, 0, 0, Math.PI * 2);
        ctx.stroke();
      }
      ctx.globalAlpha = 1;
    } else if (active.kind === "rocket") {
      // jet flame under the rocket pack
      const fx = cx - baseW * 0.28;
      const fy = player.y - h * 0.3;
      const fl = 16 + Math.random() * 10;
      ctx.fillStyle = "#ffb347";
      ctx.beginPath();
      ctx.moveTo(fx - 6, fy);
      ctx.lineTo(fx + 6, fy);
      ctx.lineTo(fx - 4, fy + fl);
      ctx.closePath();
      ctx.fill();
      ctx.fillStyle = C.title;
      ctx.beginPath();
      ctx.moveTo(fx - 3, fy);
      ctx.lineTo(fx + 3, fy);
      ctx.lineTo(fx - 2, fy + fl * 0.6);
      ctx.closePath();
      ctx.fill();
      ctx.fillStyle = "#8fa3c8";
      ctx.strokeStyle = C.ink;
      ctx.lineWidth = 1.5;
      ctx.beginPath();
      ctx.roundRect(fx - 8, fy - 22, 14, 24, 5);
      ctx.fill();
      ctx.stroke();
    }

    // flicker while the post-special grace period runs out
    if (grace > 0 && Math.floor(clock * 14) % 2 === 0) ctx.globalAlpha = 0.45;

    ctx.save();
    ctx.translate(cx, player.y - bob + 1);
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
    ctx.globalAlpha = 1;

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
      ctx.fillStyle = C.title;
      ctx.fillText(p.text, p.x, p.y);
    }
    ctx.globalAlpha = 1;
  }

  function drawHud() {
    const c = char();
    ctx.textBaseline = "alphabetic";

    // score
    ctx.font = "700 15px Nunito, system-ui, sans-serif";
    ctx.textAlign = "right";
    const flashing = clock < flashUntil && Math.floor(clock * 8) % 2 === 0;
    ctx.fillStyle = C.text;
    if (!flashing) ctx.fillText(pad(score), W - 16, 24);
    ctx.globalAlpha = 0.6;
    ctx.fillText(`HI ${pad(hiScore)}`, W - 72, 24);
    ctx.globalAlpha = 1;

    // special meter
    if (mode === "ready") return;
    const { cost, name } = c.special;
    const ready = charge >= cost;
    ctx.textAlign = "right";
    ctx.font = "700 12px Nunito, system-ui, sans-serif";
    ctx.fillStyle = active.kind || ready ? C.title : C.text;
    let label = name;
    if (active.kind) label = `${name}!`;
    else if (ready) label = `${name} ready — press X`;
    ctx.fillText(label, W - 16, 46);

    const pip = cost > 7 ? 10 : 13;
    for (let i = 0; i < cost; i++) {
      const x = W - 22 - (cost - 1 - i) * pip;
      let filled = i < charge;
      if (active.kind) filled = i < Math.ceil((active.t / active.max) * cost);
      starPath(x, 60, pip * 0.42);
      if (filled) {
        ctx.fillStyle = ready && Math.floor(clock * 4) % 2 ? "#fff3c4" : C.title;
        ctx.fill();
      } else {
        ctx.strokeStyle = "rgba(255,246,227,0.45)";
        ctx.lineWidth = 1;
        ctx.stroke();
      }
    }

    if (zoneBannerT > 0) {
      ctx.globalAlpha = Math.min(1, zoneBannerT);
      ctx.textAlign = "center";
      ctx.font = "700 16px Nunito, system-ui, sans-serif";
      ctx.fillStyle = C.text;
      ctx.fillText(`✦ entering the ${zones[zoneIndex].name} ✦`, W / 2, 92);
      ctx.globalAlpha = 1;
    }
  }

  function titleText(text, x, y, size) {
    ctx.font = `700 ${size}px Nunito, system-ui, sans-serif`;
    ctx.textAlign = "center";
    ctx.fillStyle = C.titleShadow;
    ctx.fillText(text, x + 2, y + 3);
    ctx.fillStyle = C.title;
    ctx.fillText(text, x, y);
  }

  function bodyText(text, x, y, size = 15, alpha = 1) {
    ctx.font = `700 ${size}px Nunito, system-ui, sans-serif`;
    ctx.textAlign = "center";
    ctx.globalAlpha = alpha;
    ctx.fillStyle = C.text;
    ctx.fillText(text, x, y);
    ctx.globalAlpha = 1;
  }

  function drawOverlay() {
    if (mode === "running") return;
    const cx = W / 2 + 30;
    const pw = Math.min(W - 150, 440);
    ctx.fillStyle = "rgba(18,15,51,0.6)";
    ctx.beginPath();
    ctx.roundRect(cx - pw / 2, 52, pw, mode === "paused" ? 88 : 122, 18);
    ctx.fill();

    if (mode === "ready") {
      titleText("CROSBY'S COSMIC DASH", cx, 90, W < 600 ? 26 : 34);
      bodyText("press space or tap to start", cx, 120);
      bodyText("↑ jump · ↓ duck · X special move", cx, 144, 13, 0.75);
    } else if (mode === "over") {
      titleText("OH NO!", cx, 90, 34);
      bodyText(`${char().name} bumped into ${bumpedInto}`, cx, 120);
      bodyText("tap or press space to try again", cx, 146, 13, 0.75);
    } else if (mode === "paused") {
      titleText("PAUSED", cx, 94, 30);
      bodyText("tap or press space to keep going", cx, 124, 14);
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
    if (active.kind === "calm") {
      ctx.fillStyle = "rgba(95,224,208,0.08)";
      ctx.fillRect(0, 0, W, H);
    }
    drawNavigatorMarkers();
    drawHud();
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

  const JUMP_KEYS = ["Space", "ArrowUp", "KeyW", "Enter"];
  const DUCK_KEYS = ["ArrowDown", "KeyS"];
  const SPECIAL_KEYS = ["KeyX", "KeyE", "ShiftLeft", "ShiftRight"];

  function handleKeyDown(event) {
    if (JUMP_KEYS.includes(event.code)) {
      event.preventDefault();
      if (!event.repeat) pressJump();
    } else if (DUCK_KEYS.includes(event.code)) {
      event.preventDefault();
      setDuck(true);
    } else if (SPECIAL_KEYS.includes(event.code)) {
      event.preventDefault();
      if (!event.repeat) useSpecial();
    } else if (event.code === "KeyP" || event.code === "Escape") {
      if (mode === "running") pause();
    }
  }

  function handleKeyUp(event) {
    if (JUMP_KEYS.includes(event.code)) releaseJump();
    else if (DUCK_KEYS.includes(event.code)) setDuck(false);
  }

  function handleFocusOut(event) {
    // keep playing when focus moves to the special button inside the game
    if (!wrapper?.contains(event.relatedTarget)) pause();
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

  function handleSpecialPointerDown(event) {
    // don't steal keyboard focus from the game
    event.preventDefault();
    event.stopPropagation();
    useSpecial();
  }

  function selectCharacter(id) {
    if (mode === "running") return;
    selectedId = id;
    reset();
    mode = "ready";
    uiState = "ready";
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
        {#if c.hero}<span aria-hidden="true" class="text-[#d2234c]">★</span>{/if}
      </button>
    {/each}
  </div>

  <div
    class="skill-card w-full max-w-xl rounded-[1.25rem] border-2 bg-white/80 px-4 py-3 text-left text-sm text-[#352f47] shadow-[0_6px_14px_rgba(53,47,71,0.12)]"
    style="border-color: {selected.accent};"
  >
    <div class="flex flex-wrap items-baseline justify-between gap-x-3 gap-y-1">
      <p class="text-base font-bold">
        {selected.name}
        <span class="font-normal text-[#4c465f]">· {selected.role}</span>
        {#if selected.hero}
          <span class="ml-1 rounded-full bg-[#d2234c] px-2 py-0.5 align-middle text-xs font-bold text-white">main hero</span>
        {/if}
      </p>
      <p class="text-xs font-bold tracking-wide text-[#4c465f]" aria-label="Power {selected.power} out of 5">
        power <span class="text-[#e0a526]">{"★".repeat(selected.power)}</span><span class="text-[#c9c4d6]">{"★".repeat(5 - selected.power)}</span>
      </p>
    </div>
    <p class="mt-2">
      <span class="font-bold">Passive · {selected.passive.name}</span> — {selected.passive.text}
    </p>
    <p class="mt-1">
      <span class="font-bold">Special · {selected.special.name}</span>
      <span class="text-[#4c465f]">({selected.special.cost} stars)</span> — {selected.special.text}
    </p>
  </div>

  <!-- svelte-ignore a11y_no_noninteractive_tabindex, a11y_no_noninteractive_element_interactions -->
  <div
    bind:this={wrapper}
    class="game-frame w-full overflow-hidden rounded-[1.25rem] border-4 border-white bg-[#120f33] shadow-[0_10px_24px_rgba(53,47,71,0.22)]"
    tabindex="0"
    role="application"
    aria-label="Crosby's Cosmic Dash game. Press space or up arrow to jump, down arrow to duck, X for your special move, P to pause."
    aria-describedby="cosmic-dash-help"
    onkeydown={handleKeyDown}
    onkeyup={handleKeyUp}
    onfocusout={handleFocusOut}
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
    <div class="flex items-center justify-between gap-3 bg-[#241d4a] px-3 py-2">
      <span class="text-xs font-bold text-[#fff6e3]/80">
        {#if uiActive}
          {selected.special.name}!
        {:else}
          stars {Math.min(uiCharge, selected.special.cost)} / {selected.special.cost}
        {/if}
      </span>
      <button
        type="button"
        tabindex="-1"
        class="special-btn rounded-full px-4 py-1.5 text-sm font-bold lowercase text-[#352f47] disabled:opacity-40"
        style="background-color: {selected.accent};"
        disabled={uiState !== "running" || uiActive || uiCharge < selected.special.cost}
        onpointerdown={handleSpecialPointerDown}
      >
        ★ {selected.special.name}
      </button>
    </div>
  </div>

  <p id="cosmic-dash-help" class="text-sm text-[#4c465f]">
    Space / ↑ to jump (hold for higher) · ↓ to duck · X for your special move · on phones, tap to jump, swipe down to duck,
    and use the ★ button
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

  .special-btn:not(:disabled) {
    animation: special-ready 1s ease-in-out infinite alternate;
  }

  @keyframes special-ready {
    from {
      box-shadow: 0 0 0 0 rgba(243, 211, 111, 0.7);
    }
    to {
      box-shadow: 0 0 0 6px rgba(243, 211, 111, 0);
    }
  }
</style>
