<script>
  // A photo pinned behind the page: it fills the screen below the header and stays put while the
  // page's content scrolls over it. `src` is an image imported with ?enhanced. The content that
  // goes over it must be positioned (e.g. `relative`) and come after it, or the photo paints on
  // top of it.
  // With `natural`, a photo too small to fill the screen without looking grainy is shown at its
  // own size at most (smaller only when the screen is), centered like a framed print instead.
  let { src, alt, natural = false } = $props();

  const sizes = $derived(natural ? `min(100vw, ${src.img.w}px)` : "100vw");
</script>

<div class={["page-backdrop", natural && "natural"]}>
  <enhanced:img {src} {alt} {sizes} class="page-backdrop-image" fetchpriority="high" />
</div>

<style>
  .page-backdrop {
    position: fixed;
    inset: 0;
    /* --header (app.css) is the fixed header's height from md up, 0 on phones */
    top: var(--header, 0px);
  }

  .page-backdrop :global(.page-backdrop-image) {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .page-backdrop.natural {
    display: grid;
    place-items: center;
    padding: 1rem;
  }

  /* its own size at most, and within the screen below the header (minus the 1rem padding) */
  .page-backdrop.natural :global(.page-backdrop-image) {
    width: auto;
    height: auto;
    max-width: calc(100vw - 2rem);
    max-height: calc(100svh - var(--header, 0px) - 2rem);
    border-radius: 1rem;
    box-shadow: 0 18px 40px rgba(53, 47, 71, 0.25);
  }

  /* On phones a natural-size photo is too small to pin: it sits at the top of the page instead,
     and the content follows it directly (app.css drops .over-backdrop's gap after it). */
  @media (max-width: 767px) {
    .page-backdrop.natural {
      position: relative;
      inset: auto;
    }
  }
</style>
