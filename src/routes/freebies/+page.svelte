<script>
  import { asset } from "$app/paths";
  import CosmicDash from "#lib/CosmicDash.svelte";
  import Seo from "#lib/Seo.svelte";

  const crosbyColoringPagePdf = asset('Crosby Coloring Page.pdf');
  const tankColoringPagePdf = asset('Tank Coloring Page.pdf');
  const carlaColoringPagePdf = asset('Carla Coloring Page.pdf');
</script>

<Seo
  title="Free Coloring Pages & Game | Crosby's Cosmic Adventure"
  description="Free printable Crosby, Carla and Tank coloring pages, plus Crosby's Cosmic Dash, a free browser game: hop over moon rocks, dodge comets and collect stars."
  path="/freebies"
/>

<section class="page-shell flex min-h-[55vh] flex-col items-center justify-center py-16 text-center">
  <h1 class="text-4xl font-black lowercase tracking-[0.04em] text-[#352f47] md:text-5xl">
    freebies
  </h1>
  <div class="mt-8 grid grid-cols-3 place-items-center gap-[clamp(0.7rem,2.6vw,1.4rem)]">
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

  <div class="mt-14 w-full max-w-3xl">
    <h2 class="text-3xl font-black lowercase tracking-[0.04em] text-[#352f47] md:text-4xl">
      crosby's cosmic dash
    </h2>
    <p class="mx-auto mt-2 mb-6 max-w-xl text-base text-[#4c465f]">
      Blast off with Crosby and friends! Hop over moon rocks and space crystals, dodge shooting comets, and collect
      stars to power up each friend's special move.
    </p>
    <CosmicDash />
  </div>
</section>
