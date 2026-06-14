<script>
  import { resolve } from "$app/paths";

  const crosbyColoringPageImage = resolve("/Crosby Coloring Page.png");
  const crosbyColoringPagePdf = resolve("/Crosby Coloring Page.pdf");
  const tankColoringPageImage = resolve("/Tank Coloring Page.png");
  const tankColoringPagePdf = resolve("/Tank Coloring Page.pdf");
  const carlaColoringPageImage = resolve("/Carla Coloring Page.png");
  const carlaColoringPagePdf = resolve("/Carla Coloring Page.pdf");
</script>

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
      <img
        src={crosbyColoringPageImage}
        alt="Crosby coloring page portrait icon"
        class="aspect-[1430/1912] w-full h-auto rounded-[0.6rem] object-cover"
        loading="lazy"
        decoding="async"
      />
    </a>
    <a
      href={tankColoringPagePdf}
      target="_blank"
      rel="noopener noreferrer"
      aria-label="Open Tank Coloring Page PDF"
      class="inline-block w-[min(22vw,4.75rem)] rounded-[0.85rem] border border-white/90 bg-white p-1 shadow-[0_10px_20px_rgba(53,47,71,0.22)]"
    >
      <img
        src={tankColoringPageImage}
        alt="Tank coloring page portrait icon"
        class="aspect-[1430/1912] w-full h-auto rounded-[0.6rem] object-cover"
        loading="lazy"
        decoding="async"
      />
    </a>
    <a
      href={carlaColoringPagePdf}
      target="_blank"
      rel="noopener noreferrer"
      aria-label="Open Carla Coloring Page PDF"
      class="inline-block w-[min(22vw,4.75rem)] rounded-[0.85rem] border border-white/90 bg-white p-1 shadow-[0_10px_20px_rgba(53,47,71,0.22)]"
    >
      <img
        src={carlaColoringPageImage}
        alt="Carla coloring page portrait icon"
        class="aspect-[1430/1912] w-full h-auto rounded-[0.6rem] object-cover"
        loading="lazy"
        decoding="async"
      />
    </a>
  </div>
  <p class="mt-4 max-w-2xl text-base text-[#4c465f] md:text-lg">
    Free printable activities and bonus goodies are coming soon.
  </p>
</section>
