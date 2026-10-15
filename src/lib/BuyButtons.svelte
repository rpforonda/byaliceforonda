<script>
  import { AMAZON_URL } from "#lib/seo.js";

  // Where to buy the book: Amazon (paperback). The Shopify store is closed.
  let { class: className = "" } = $props();

  const starPath =
    "M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z";
</script>

{#snippet star()}
  <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" aria-hidden="true">
    <path stroke-linecap="round" stroke-linejoin="round" d={starPath} />
  </svg>
{/snippet}

<div class="flex flex-wrap items-center justify-center gap-3 {className}">
  <a
    href={AMAZON_URL}
    target="_blank"
    rel="noopener noreferrer"
    class="inline-flex items-center gap-2 rounded-full bg-rose-600 px-8 py-3 text-lg font-black uppercase tracking-widest text-white shadow-lg transition hover:bg-rose-700 active:scale-95"
  >
    {@render star()}
    buy on amazon
    {@render star()}
    <span class="rounded-full bg-white/20 px-2 py-0.5 text-[0.65rem] tracking-wider normal-case">paperback</span>
  </a>
</div>
