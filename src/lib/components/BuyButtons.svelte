<script>
  const props = $props(); // { amazonAsin?, amazonTag?, ingramLink? }

  const amazonUrl = $derived(
    props.amazonAsin
      ? `https://www.amazon.com/dp/${props.amazonAsin}${props.amazonTag ? `?tag=${props.amazonTag}` : ''}`
      : null
  );

  const go = (url) => url && window.open(url, '_blank', 'noopener');
</script>

<div class="flex flex-wrap gap-3">
  {#if amazonUrl}
    <button class="px-4 py-2 rounded-lg border border-gray-200 bg-white hover:bg-gray-50" onclick={() => go(amazonUrl)}>
      Buy on Amazon
    </button>
  {/if}
  {#if props.ingramLink}
    <button class="px-4 py-2 rounded-lg border border-gray-200 bg-white hover:bg-gray-50" onclick={() => go(props.ingramLink)}>
      Buy from Ingram
    </button>
  {/if}
</div>
