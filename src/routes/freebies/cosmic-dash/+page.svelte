<script>
  import { resolve } from "$app/paths";
  import CosmicDash from "#lib/CosmicDash.svelte";
  import Seo from "#lib/Seo.svelte";
</script>

<Seo
  title="Crosby's Cosmic Dash — a Free Game | Crosby's Cosmic Adventure"
  description="Play Crosby's Cosmic Dash, a free browser game: run with Crosby, Carla or Tank, hop over moon rocks, dodge comets and collect stars to power up special moves."
  path="/freebies/cosmic-dash"
/>

<section class="page-shell flex flex-col items-center py-12 text-center md:py-16">
  <a
    href={resolve("/freebies")}
    class="self-start text-sm font-bold lowercase text-[#4c465f] hover:text-[#d2234c]"
  >
    ← all freebies
  </a>
  <h1 class="mt-4 text-4xl font-black lowercase tracking-[0.04em] text-[#352f47] md:text-5xl">
    crosby's cosmic dash
  </h1>
  <p class="mx-auto mt-2 mb-6 max-w-xl text-base text-[#4c465f]">
    Blast off with Crosby and friends! Pick your friend, then hop over moon rocks and space crystals, dodge shooting
    comets, and collect stars to power up a special move.
  </p>
  <div class="w-full max-w-3xl">
    <CosmicDash />
  </div>
</section>
