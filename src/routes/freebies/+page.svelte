<script>
  import { asset, resolve } from "$app/paths";
  import Seo from "#lib/Seo.svelte";
  // the game's own sprites, so they're already cached when someone clicks through to play
  import crosbySprite from "#lib/assets/game/crosby.webp";
  import carlaSprite from "#lib/assets/game/carla.webp";
  import tankSprite from "#lib/assets/game/tank.webp";

  const crosbyColoringPagePdf = asset('Crosby Coloring Page.pdf');
  const tankColoringPagePdf = asset('Tank Coloring Page.pdf');
  const carlaColoringPagePdf = asset('Carla Coloring Page.pdf');
</script>

<Seo
  title="Free Coloring Pages & Game | Crosby's Cosmic Adventure"
  description="Free printable Crosby, Carla and Tank coloring pages, plus Crosby's Cosmic Dash, a free browser game starring all three friends."
  path="/freebies"
/>

<section class="page-shell flex min-h-[55vh] flex-col items-center justify-center py-16 text-center">
  <h1 class="text-4xl font-black lowercase tracking-[0.04em] text-[#352f47] md:text-5xl">
    freebies
  </h1>

  <a
    href={resolve("/freebies/cosmic-dash")}
    class="game-promo relative mt-8 block w-full max-w-xl overflow-hidden rounded-[1.25rem] border-4 border-white bg-gradient-to-b from-[#120f33] via-[#2b2466] to-[#4f3a8c] px-6 pt-6 text-center shadow-[0_10px_24px_rgba(53,47,71,0.22)]"
  >
    <span aria-hidden="true" class="pointer-events-none absolute inset-0">
      <span class="absolute left-[8%] top-[18%] text-[#f3d36f]">★</span>
      <span class="absolute left-[22%] top-[8%] text-xs text-white/70">✦</span>
      <span class="absolute right-[10%] top-[14%] text-sm text-[#f3d36f]">★</span>
      <span class="absolute right-[7%] top-[58%] text-xs text-white/60">✦</span>
      <span class="absolute left-[14%] top-[48%] text-xs text-white/50">✦</span>
    </span>
    <span class="relative block text-xs font-bold uppercase tracking-[0.2em] text-[#ffd28d]">free game</span>
    <span class="game-promo-title relative mt-1 block text-3xl font-black uppercase text-[#f3d36f] md:text-4xl">
      crosby's cosmic dash
    </span>
    <span class="relative mx-auto mt-2 block max-w-sm text-sm text-[#fff6e3]/90 md:text-base">
      Run, jump and collect stars with Crosby, Carla and Tank, each with their own special move.
    </span>
    <span class="play-btn relative mt-4 inline-block rounded-full bg-[#d2234c] px-6 py-2 text-sm font-bold lowercase text-white shadow-[0_6px_14px_rgba(210,35,76,0.4)]">
      play now ▶
    </span>
    <span aria-hidden="true" class="relative -mx-6 mt-5 flex items-end justify-center gap-5">
      <img src={carlaSprite} alt="" width="172" height="240" class="h-16 w-auto -scale-x-100" loading="lazy" />
      <img src={crosbySprite} alt="" width="288" height="240" class="h-20 w-auto" loading="lazy" />
      <img src={tankSprite} alt="" width="211" height="240" class="h-[4.5rem] w-auto" loading="lazy" />
    </span>
    <span aria-hidden="true" class="relative -mx-6 block h-4 border-t-2 border-[#e9e1ff] bg-gradient-to-b from-[#a99ad8] to-[#7d6cb8]"></span>
  </a>

  <h2 class="mt-12 text-2xl font-black lowercase tracking-[0.04em] text-[#352f47] md:text-3xl">
    coloring pages
  </h2>
  <div class="mt-6 grid grid-cols-3 place-items-center gap-[clamp(0.7rem,2.6vw,1.4rem)]">
    <a
      href={crosbyColoringPagePdf}
      target="_blank"
      rel="noopener noreferrer"
      aria-label="Open Crosby Coloring Page PDF"
      class="inline-block w-[min(22vw,4.75rem)] rounded-[0.85rem] border border-white/90 bg-white p-1 shadow-[0_10px_20px_rgba(53,47,71,0.22)]"
    >
      <enhanced:img
        src="#lib/assets/crosby-coloring-page.png"
        width="152"
        alt="Crosby the crocodile coloring page"
        class="aspect-[1430/1912] w-full h-auto rounded-[0.6rem] object-cover"
        loading="lazy"
        decoding="async"
      ></enhanced:img>
    </a>
    <a
      href={tankColoringPagePdf}
      target="_blank"
      rel="noopener noreferrer"
      aria-label="Open Tank Coloring Page PDF"
      class="inline-block w-[min(22vw,4.75rem)] rounded-[0.85rem] border border-white/90 bg-white p-1 shadow-[0_10px_20px_rgba(53,47,71,0.22)]"
    >
      <enhanced:img
        src="#lib/assets/tank-coloring-page.png"
        width="152"
        alt="Tank the turtle coloring page"
        class="aspect-[1430/1912] w-full h-auto rounded-[0.6rem] object-cover"
        loading="lazy"
        decoding="async"
      ></enhanced:img>
    </a>
    <a
      href={carlaColoringPagePdf}
      target="_blank"
      rel="noopener noreferrer"
      aria-label="Open Carla Coloring Page PDF"
      class="inline-block w-[min(22vw,4.75rem)] rounded-[0.85rem] border border-white/90 bg-white p-1 shadow-[0_10px_20px_rgba(53,47,71,0.22)]"
    >
      <enhanced:img
        src="#lib/assets/carla-coloring-page.png"
        width="152"
        alt="Carla the capybara coloring page"
        class="aspect-[1430/1912] w-full h-auto rounded-[0.6rem] object-cover"
        loading="lazy"
        decoding="async"
      ></enhanced:img>
    </a>
  </div>
  <p class="mt-4 max-w-2xl text-base text-[#4c465f] md:text-lg">
    Free printable activities and bonus goodies are coming soon.
  </p>
</section>

<style>
  .game-promo-title {
    text-shadow: 2px 3px 0 #7c2b28;
  }

  .game-promo:hover .play-btn {
    background-color: #b31d41;
  }

  /* the site-wide hover zoom on images would jostle the sprites */
  .game-promo img:hover {
    transform: none;
  }
</style>
