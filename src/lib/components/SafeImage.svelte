<script>
  /**
   * SafeImage component with loading and error states
   * Provides fallback UI for failed image loads
   */

  let { src, alt, class: className = '', fallbackText = 'Image unavailable', ...rest } = $props();

  let loaded = $state(false);
  let error = $state(false);

  function handleLoad() {
    loaded = true;
    error = false;
  }

  function handleError() {
    loaded = false;
    error = true;
    console.warn(`Failed to load image: ${src}`);
  }
</script>

{#if error}
  <div
    class="flex items-center justify-center bg-gray-100 text-gray-500 {className}"
    role="img"
    aria-label={alt}
  >
    <span class="text-sm">{fallbackText}</span>
  </div>
{:else}
  <img
    {src}
    {alt}
    class={className}
    class:opacity-0={!loaded}
    class:opacity-100={loaded}
    style="transition: opacity 0.3s ease-in-out;"
    onload={handleLoad}
    onerror={handleError}
    {...rest}
  />
{/if}
