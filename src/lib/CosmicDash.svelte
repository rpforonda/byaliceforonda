<script>
  import { onMount } from "svelte";
  import { SITE_URL } from "#lib/seo.js";
  import crosbySprite from "#lib/assets/game/crosby.webp";
  import carlaSprite from "#lib/assets/game/carla.webp";
  import tankSprite from "#lib/assets/game/tank.webp";

  // Crosby's Cosmic Dash: an endless runner in the starry world of Crosby's Cosmic Adventure.
  // The game itself is written in Rust and compiled to WebAssembly (`cosmic-dash/` at the repo
  // root): the simulation, the characters, and every screen including the intro/character
  // select are drawn on the canvas. This component is the shell around it: sprites, input,
  // the animation loop, the high score, screen-reader announcements, and score sharing.

  const sprites = { crosby: crosbySprite, carla: carlaSprite, tank: tankSprite };

  // Colors for the share card (the in-game palette lives in the Rust engine).
  const C = {
    skyTop: "#120f33",
    skyMid: "#2b2466",
    skyLow: "#4f3a8c",
    title: "#f3d36f",
    titleShadow: "#7c2b28",
    text: "#fff6e3",
    gold: "#ffd28d",
  };

  // The engine lays the world out 480 or 720 units wide and always 280 tall.
  const H = 280;
  let W = 720;
  const HI_KEY = "catchupwithalice-cosmic-dash-hi";

  let wrapper = $state();
  let canvas = $state();
  let strip = $state(); // the bar under the canvas (star count, special button)
  let fullscreenOK = $state(false); // set on mount; iPhone Safari can't make an element full screen
  let isFullscreen = $state(false);
  let friend = $state(null); // the chosen character, as described by the engine
  let uiState = $state("ready");
  let uiCharge = $state(0);
  let uiActive = $state(false);
  let announcement = $state("");
  let controls = $state(""); // how to play, from the engine (shown below the game on narrow screens)
  let loadError = $state(""); // "nowasm" or "failed" if the engine couldn't load
  // Results shown on the game-over screen.
  let lastScore = $state(0);
  let lastIsBest = $state(false);
  let shareMsg = $state("");
  let shareTimer;

  const images = {};
  let game = null; // the Rust engine; loaded in onMount, so it never runs during prerendering
  let selected = -1;
  let rafId = 0;
  let looping = false;

  function loadHiScore() {
    try {
      return Number(localStorage.getItem(HI_KEY)) || 0;
    } catch {
      return 0;
    }
  }

  function saveHiScore(score) {
    try {
      localStorage.setItem(HI_KEY, String(Math.floor(score)));
    } catch {
      // storage unavailable (private mode etc.) — the score just won't persist
    }
  }

  // Copy what the UI shows out of the engine, and react to what changed since last time.
  function sync() {
    if (!game) return;
    if (game.selected() !== selected) {
      const first = selected === -1;
      selected = game.selected();
      friend = game.character();
      // the intro is drawn on the canvas, so read the choice out for screen readers
      if (!first) {
        const f = friend;
        announcement = `${f.name}, ${f.role}. Passive, ${f.passiveName}: ${f.passiveText} Special, ${f.specialName}, ${f.specialCost} stars: ${f.specialText}`;
      }
    }
    const mode = game.mode();
    const active = game.special_active();
    if (mode !== uiState) {
      if (mode === "running" && uiState !== "paused") announcement = "";
      if (mode === "over") {
        lastScore = game.last_score();
        lastIsBest = game.last_is_best();
        shareMsg = "";
        if (lastIsBest) saveHiScore(game.hi_score());
        announcement = `${friend.name} bumped into ${game.bumped_into()}. Score ${lastScore}.`;
      }
      uiState = mode;
    }
    if (active && !uiActive) announcement = `${friend.specialName}!`;
    uiActive = active;
    uiCharge = game.charge();
  }

  // The engine words its hints for touch or keys; keep the page's copy of the controls in step.
  function setTouch(touch) {
    if (!game) return;
    game.set_touch(touch);
    controls = game.controls();
  }

  function loop(now) {
    looping = game.frame(now);
    sync();
    if (looping) rafId = requestAnimationFrame(loop);
  }

  // Call after any input: starts the frame loop if that input started or resumed a run.
  function afterInput() {
    sync();
    if (!looping && uiState === "running") {
      looping = true;
      rafId = requestAnimationFrame(loop);
    }
  }

  // Share the score. Privacy-safe by design: the text and the score-card image are
  // built in the browser from the run's own numbers (no name, device, or identifier),
  // nothing is sent to any server, and it only runs when the player taps the button.
  // We hand them to the OS share sheet (Web Share API). The image is what actually
  // posts to Instagram/Facebook, which don't accept pre-filled text.
  function shareText() {
    const pts = Math.floor(lastScore).toLocaleString();
    const best = lastIsBest ? " (new best!)" : "";
    return `🚀 I scored ${pts} as ${friend.name} in Crosby's Cosmic Dash!${best}\nPlay at ${SITE_URL}/freebies/cosmic-dash`;
  }

  // Draw a 1080×1080 share card on an off-screen canvas and return it as a PNG File.
  // All inputs are local (game numbers + same-origin sprites), so the canvas is never
  // tainted and nothing leaves the device.
  function buildScoreCard() {
    return new Promise((resolve, reject) => {
      const S = 1080;
      const cv = document.createElement("canvas");
      cv.width = S;
      cv.height = S;
      const g = cv.getContext("2d");
      if (!g) return reject(new Error("no 2d context"));

      // deep-space background
      const grad = g.createLinearGradient(0, 0, 0, S);
      grad.addColorStop(0, C.skyTop);
      grad.addColorStop(0.55, C.skyMid);
      grad.addColorStop(1, C.skyLow);
      g.fillStyle = grad;
      g.fillRect(0, 0, S, S);

      // scattered stars
      for (let i = 0; i < 80; i++) {
        g.globalAlpha = 0.35 + Math.random() * 0.6;
        g.fillStyle = "#fff6e3";
        g.beginPath();
        g.arc(Math.random() * S, Math.random() * S * 0.85, Math.random() * 2.4 + 0.6, 0, Math.PI * 2);
        g.fill();
      }
      g.globalAlpha = 1;
      g.textAlign = "center";

      // title
      g.font = "700 54px Nunito, sans-serif";
      g.fillStyle = C.text;
      g.fillText("CROSBY'S COSMIC DASH", S / 2, 135);

      const finish = () => {
        const pts = Math.floor(lastScore).toLocaleString();
        // big score with maroon drop shadow in the cover palette
        g.font = "700 190px Nunito, sans-serif";
        g.fillStyle = C.titleShadow;
        g.fillText(pts, S / 2 + 7, 727);
        g.fillStyle = C.title;
        g.fillText(pts, S / 2, 720);

        g.font = "700 46px Nunito, sans-serif";
        g.fillStyle = C.text;
        g.fillText(`points as ${friend.name}`, S / 2, 795);

        if (lastIsBest) {
          g.font = "700 40px Nunito, sans-serif";
          g.fillStyle = C.gold;
          g.fillText("★ NEW BEST! ★", S / 2, 862);
        }

        g.font = "700 42px Nunito, sans-serif";
        g.fillStyle = "rgba(255,246,227,0.85)";
        g.fillText("catchupwithalice.com", S / 2, S - 72);

        cv.toBlob((blob) => {
          if (!blob) return reject(new Error("toBlob failed"));
          resolve(new File([blob], "cosmic-dash-score.png", { type: "image/png" }));
        }, "image/png");
      };

      // character sprite (reuse the already-loaded, same-origin game image)
      const img = images[friend.id];
      if (img?.complete && img.naturalWidth) {
        const h = 330;
        const w = (img.naturalWidth / img.naturalHeight) * h;
        const top = 175;
        g.save();
        if (friend.flip) {
          g.translate(S / 2 + w / 2, top);
          g.scale(-1, 1);
          g.drawImage(img, 0, 0, w, h);
        } else {
          g.drawImage(img, S / 2 - w / 2, top, w, h);
        }
        g.restore();
      }
      finish();
    });
  }

  function downloadFile(file) {
    const url = URL.createObjectURL(file);
    const a = document.createElement("a");
    a.href = url;
    a.download = file.name;
    document.body.appendChild(a);
    a.click();
    a.remove();
    setTimeout(() => URL.revokeObjectURL(url), 1000);
  }

  async function shareScore() {
    const text = shareText();
    shareMsg = "";
    clearTimeout(shareTimer);
    const flash = (msg) => {
      shareMsg = msg;
      shareTimer = setTimeout(() => (shareMsg = ""), 2600);
    };

    let file = null;
    try {
      await document.fonts?.ready; // so the card text uses Nunito, not a fallback
      file = await buildScoreCard();
    } catch {
      file = null; // fall back to text-only sharing
    }

    try {
      if (file && navigator.canShare?.({ files: [file] })) {
        await navigator.share({ files: [file], text });
      } else if (navigator.share) {
        await navigator.share({ text });
      } else if (file) {
        // desktop browsers without Web Share: save the card to post manually
        downloadFile(file);
        flash("Saved your score card — post it on Instagram or Facebook!");
      } else if (navigator.clipboard?.writeText) {
        await navigator.clipboard.writeText(text);
        flash("Copied — paste it anywhere!");
      } else {
        flash("Copy: " + text);
      }
    } catch (err) {
      // AbortError = the player dismissed the share sheet; nothing to report.
      if (err && err.name === "AbortError") return;
      try {
        if (file) {
          downloadFile(file);
          flash("Saved your score card — post it on Instagram or Facebook!");
        } else {
          await navigator.clipboard.writeText(text);
          flash("Copied — paste it anywhere!");
        }
      } catch {
        flash("Couldn't share — long-press to copy your score.");
      }
    }
  }

  function pressJump() {
    if (!game) return;
    game.press_jump(performance.now());
    afterInput();
  }

  function releaseJump() {
    game?.release_jump();
  }

  function setDuck(on) {
    game?.set_duck(on);
  }

  function useSpecial() {
    if (!game) return;
    game.use_special(performance.now());
    afterInput();
  }

  function pickStep(delta) {
    if (!game) return;
    game.pick_step(delta);
    sync();
  }

  function pause() {
    if (!game) return;
    game.pause();
    sync();
  }

  async function toggleFullscreen() {
    if (document.fullscreenElement) return document.exitFullscreen();
    try {
      await wrapper.requestFullscreen({ navigationUI: "hide" });
      // On phones, turn the game sideways: wide enough for the roomier layout (Android only).
      if (matchMedia("(pointer: coarse)").matches) await screen.orientation?.lock?.("landscape");
    } catch {
      // full screen refused, or the screen can't be turned: carry on as we are
    }
  }

  // ---------- sizing & input ----------

  function resize() {
    if (!wrapper || !canvas) return;
    const frameW = wrapper.clientWidth;
    W = frameW < 560 ? 480 : 720;
    // Full screen: as big as fits above the strip, letterboxed on the night sky.
    const fs = document.fullscreenElement === wrapper;
    const cssW = fs ? Math.min(frameW, ((wrapper.clientHeight - (strip?.offsetHeight ?? 0)) * W) / H) : frameW;
    canvas.style.width = fs ? `${cssW}px` : "";
    const dpr = window.devicePixelRatio || 1;
    canvas.width = Math.round(cssW * dpr);
    canvas.height = Math.round((cssW * dpr * H) / W);
    canvas.style.aspectRatio = `${W} / ${H}`;
    game?.set_context(canvas.getContext("2d"), W, canvas.width / W);
  }

  const JUMP_KEYS = ["Space", "ArrowUp", "KeyW", "Enter"];
  const DUCK_KEYS = ["ArrowDown", "KeyS"];
  const SPECIAL_KEYS = ["KeyX", "KeyE", "ShiftLeft", "ShiftRight"];
  const PICK_KEYS = { ArrowLeft: -1, KeyA: -1, ArrowRight: 1, KeyD: 1 };

  function handleKeyDown(event) {
    setTouch(false);
    if (JUMP_KEYS.includes(event.code)) {
      event.preventDefault();
      if (!event.repeat) pressJump();
    } else if (DUCK_KEYS.includes(event.code)) {
      event.preventDefault();
      setDuck(true);
    } else if (SPECIAL_KEYS.includes(event.code)) {
      event.preventDefault();
      if (!event.repeat) useSpecial();
    } else if (event.code in PICK_KEYS) {
      event.preventDefault();
      pickStep(PICK_KEYS[event.code]);
    } else if (event.code === "KeyF" && fullscreenOK) {
      event.preventDefault();
      if (!event.repeat) toggleFullscreen();
    } else if (event.code === "KeyP" || event.code === "Escape") {
      pause();
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
    if (!game) return;
    setTouch(event.pointerType === "touch");
    // canvas pixels → world units, so the engine can tell which friend or button was tapped
    const rect = canvas.getBoundingClientRect();
    game.tap(((event.clientX - rect.left) * W) / rect.width, ((event.clientY - rect.top) * H) / rect.height, performance.now());
    afterInput();
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

  onMount(() => {
    let destroyed = false;
    for (const [id, src] of Object.entries(sprites)) {
      const img = new Image();
      img.decoding = "async";
      img.src = src;
      images[id] = img;
    }

    resize();
    const ro = new ResizeObserver(() => resize());
    ro.observe(wrapper);

    const onVisibility = () => {
      if (document.hidden) pause();
    };
    document.addEventListener("visibilitychange", onVisibility);

    fullscreenOK = document.fullscreenEnabled === true;
    const onFullscreenChange = () => {
      isFullscreen = document.fullscreenElement === wrapper;
      if (!isFullscreen) {
        screen.orientation?.unlock?.();
        // the game jumps back to its place on the page, so give the player a moment
        pause();
      }
      resize();
      if (!isFullscreen) wrapper?.scrollIntoView({ block: "nearest" });
    };
    document.addEventListener("fullscreenchange", onFullscreenChange);

    // Fetch and start the engine (its .wasm is only downloaded on the game's page).
    import("#cosmic-dash")
      .then(async ({ default: init, Game }) => {
        await init();
        if (destroyed) return;
        game = new Game();
        game.set_hi_score(loadHiScore());
        setTouch(matchMedia("(pointer: coarse)").matches);
        for (const [id, img] of Object.entries(images)) {
          if (img.complete) game.set_sprite(id, img);
          else img.onload = () => game?.set_sprite(id, img);
        }
        resize();
        sync();
        // Redraw once web fonts are ready so canvas text uses Nunito.
        document.fonts?.ready.then(() => game?.draw());
      })
      .catch((err) => {
        // WebAssembly turned off, a dropped connection, or an old file gone after a redeploy
        if (destroyed || game) return;
        console.error("Cosmic Dash failed to load", err);
        loadError = typeof WebAssembly === "undefined" ? "nowasm" : "failed";
      });

    return () => {
      destroyed = true;
      cancelAnimationFrame(rafId);
      clearTimeout(shareTimer);
      ro.disconnect();
      document.removeEventListener("visibilitychange", onVisibility);
      document.removeEventListener("fullscreenchange", onFullscreenChange);
      game?.free();
      game = null;
    };
  });
</script>

<div class="@container flex w-full flex-col items-center gap-4">
  <!-- svelte-ignore a11y_no_noninteractive_tabindex, a11y_no_noninteractive_element_interactions -->
  <div
    bind:this={wrapper}
    class="game-frame relative w-full overflow-hidden rounded-[1.25rem] border-4 border-white bg-[#120f33] shadow-[0_10px_24px_rgba(53,47,71,0.22)]"
    tabindex="0"
    role="application"
    aria-label="Crosby's Cosmic Dash game"
    aria-describedby="cosmic-dash-help"
    onkeydown={handleKeyDown}
    onkeyup={handleKeyUp}
    onfocusout={handleFocusOut}
  >
    <canvas
      bind:this={canvas}
      class="block aspect-[720/280] w-full select-none"
      style="touch-action: {uiState === 'running' ? 'none' : 'manipulation'};"
      onpointerdown={handlePointerDown}
      onpointermove={handlePointerMove}
      onpointerup={handlePointerUp}
      onpointercancel={handlePointerUp}
      oncontextmenu={(e) => e.preventDefault()}
    ></canvas>
    {#if loadError}
      <div class="absolute inset-0 flex flex-col items-center justify-center gap-3 p-4 text-center" role="alert">
        <p class="max-w-md text-sm font-bold text-[#fff6e3]">
          {loadError === "nowasm"
            ? "This browser has WebAssembly turned off, so the game can't run. Allow it for this site in your browser's security settings, or play on another device."
            : "The game didn't load. Check your connection, then reload the page."}
        </p>
        {#if loadError === "failed"}
          <!-- stopPropagation: the game's key handler would otherwise swallow Space/Enter -->
          <button
            type="button"
            class="share-btn rounded-full px-5 py-2 text-sm font-bold lowercase text-white"
            onclick={() => location.reload()}
            onkeydown={(e) => e.stopPropagation()}
          >
            reload the page
          </button>
        {/if}
      </div>
    {/if}
    <div
      bind:this={strip}
      class="flex min-h-12 items-center justify-between gap-3 bg-[#241d4a] px-3 py-2"
      class:hidden={loadError}
    >
      {#if friend}
        <span class="text-xs font-bold text-[#fff6e3]/80">
          {#if uiActive}
            {friend.specialName}!
          {:else}
            stars {Math.min(uiCharge, friend.specialCost)} / {friend.specialCost}
          {/if}
        </span>
        <div class="flex items-center gap-4">
          {#if fullscreenOK}
            <!-- tabindex -1 and no focus on press, like the special button: keys go to the game (F toggles this) -->
            <button
              type="button"
              tabindex="-1"
              class="fs-btn grid h-10 w-10 place-items-center rounded-full text-[#fff6e3]"
              aria-label={isFullscreen ? "exit full screen" : "full screen"}
              title={isFullscreen ? "exit full screen (F)" : "full screen (F)"}
              onpointerdown={(e) => e.preventDefault()}
              onclick={toggleFullscreen}
            >
              <svg viewBox="0 0 20 20" class="h-5 w-5" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                {#if isFullscreen}
                  <path d="M7 3v4H3M13 3v4h4M7 17v-4H3M13 17v-4h4" />
                {:else}
                  <path d="M3 7V3h4M17 7V3h-4M3 13v4h4M17 13v4h-4" />
                {/if}
              </svg>
            </button>
          {/if}
          <button
            type="button"
            tabindex="-1"
            class="special-btn rounded-full px-4 py-1.5 text-sm font-bold lowercase text-[#352f47] disabled:opacity-40"
            style="background-color: {friend.accent};"
            disabled={uiState !== "running" || uiActive || uiCharge < friend.specialCost}
            onpointerdown={handleSpecialPointerDown}
          >
            ★ {friend.specialName}
          </button>
        </div>
      {/if}
    </div>
  </div>

  {#if uiState === "over" && friend}
    <div class="flex flex-col items-center gap-2">
      <p class="text-sm font-bold text-[#352f47]">
        You scored {lastScore.toLocaleString()} as {friend.name}{#if lastIsBest}{" — new best!"}{/if}
      </p>
      <button
        type="button"
        class="share-btn rounded-full px-5 py-2 text-sm font-bold lowercase text-white shadow-[0_6px_14px_rgba(210,35,76,0.3)]"
        onclick={shareScore}
      >
        🚀 share my score
      </button>
      <p class="min-h-[1.1rem] text-xs text-[#4c465f]" aria-live="polite">{shareMsg}</p>
    </div>
  {/if}

  <!-- Scaled down to a phone's width, the canvas is too small for paragraphs, so the engine skips
       the chosen friend's moves and the controls there and they show here instead (568px is the
       engine's 560px narrow layout plus the frame's border). Wider screens draw them on the
       canvas, and this stays readable to screen readers only. -->
  <div
    class={[
      "sr-only mx-auto w-full max-w-xl space-y-2 px-1 text-left text-sm text-[#352f47] @max-[568px]:not-sr-only",
      !loadError && "@max-[568px]:min-h-44",
    ]}
  >
    {#if friend}
      <p>
        <span class="font-bold">{friend.name}</span> · {friend.role.toLowerCase()}{#if friend.hero}&nbsp;· main hero{/if}
      </p>
      <p><span class="font-bold">Passive · {friend.passiveName}:</span> {friend.passiveText}</p>
      <p><span class="font-bold">Special · {friend.specialName} ({friend.specialCost} ★):</span> {friend.specialText}</p>
      <!-- in-run controls (the canvas shows how to start); the help text below already
           describes the controls to screen readers -->
      <p class="text-xs text-[#4c465f]" aria-hidden="true">{controls}</p>
    {/if}
  </div>

  <p id="cosmic-dash-help" class="sr-only">
    Choose Crosby, Carla or Tank with the left and right arrow keys, then press space to start. Up arrow or space to jump,
    hold for a higher jump. Down arrow to duck. X for your special move, charged by collecting stars. P to pause. F
    for full screen. On a
    touch screen, tap a friend to choose and tap again to start, tap to jump, swipe down to duck, and use the star button
    under the game for your special.
  </p>
  <p class="sr-only" aria-live="polite">{announcement}</p>
</div>

<style>
  .game-frame:focus-visible {
    outline: 3px solid #d2234c;
    outline-offset: 3px;
  }

  .game-frame:fullscreen {
    display: flex;
    flex-direction: column;
    justify-content: center;
    border: 0;
    border-radius: 0;
  }

  .game-frame:fullscreen canvas {
    margin-inline: auto;
  }

  .fs-btn {
    border: 1px solid rgba(255, 246, 227, 0.4);
  }

  .fs-btn:hover {
    background-color: rgba(255, 246, 227, 0.12);
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

  .share-btn {
    background-color: #d2234c;
  }

  .share-btn:hover {
    background-color: #b31d41;
  }
</style>
