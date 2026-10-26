<script>
  // A photo pinned behind the page: it fills the screen below the header and stays put while the
  // page's content scrolls over it. `src` is an image imported with ?enhanced. The content that
  // goes over it must be positioned (e.g. `relative`) and come after it, or the photo paints on
  // top of it.
  let { src, alt } = $props();
</script>

<div class="page-backdrop">
  <enhanced:img {src} {alt} sizes="100vw" class="page-backdrop-image" fetchpriority="high" />
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
</style>
