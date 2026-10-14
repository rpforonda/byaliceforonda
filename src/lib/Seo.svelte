<script>
  import { SITE_URL, SITE_NAME, OG_IMAGE } from "#lib/seo.js";

  // Per-page <head> metadata. Every route renders one of these; the pages are
  // prerendered, so crawlers and link previews get it in the HTML.
  let { title, description, path, noindex = false, jsonLd = null } = $props();

  const canonical = $derived(`${SITE_URL}${path === "/" ? "/" : path}`);

  // JSON-LD goes in via {@html}; escape "<" so content can't close the script tag.
  const jsonLdScript = $derived(
    jsonLd
      ? `<script type="application/ld+json">${JSON.stringify(jsonLd).replace(/</g, "\\u003c")}</` + "script>"
      : "",
  );
</script>

<svelte:head>
  <title>{title}</title>
  <meta name="description" content={description} />
  {#if noindex}
    <meta name="robots" content="noindex, follow" />
  {:else}
    <link rel="canonical" href={canonical} />
  {/if}

  <meta property="og:site_name" content={SITE_NAME} />
  <meta property="og:type" content="website" />
  <meta property="og:locale" content="en_US" />
  <meta property="og:title" content={title} />
  <meta property="og:description" content={description} />
  <meta property="og:url" content={canonical} />
  <meta property="og:image" content={OG_IMAGE.url} />
  <meta property="og:image:width" content={String(OG_IMAGE.width)} />
  <meta property="og:image:height" content={String(OG_IMAGE.height)} />
  <meta property="og:image:alt" content={OG_IMAGE.alt} />

  <meta name="twitter:card" content="summary_large_image" />
  <meta name="twitter:title" content={title} />
  <meta name="twitter:description" content={description} />
  <meta name="twitter:image" content={OG_IMAGE.url} />
  <meta name="twitter:image:alt" content={OG_IMAGE.alt} />

  {#if jsonLdScript}
    {@html jsonLdScript}
  {/if}
</svelte:head>
