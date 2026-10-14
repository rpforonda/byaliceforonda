<script>
  import { onMount } from "svelte";

  const mailerLiteAccountId = "2261437";
  const mailerLiteFormId = "QQryIS";
  const mailerLiteScriptUrl = "https://assets.mailerlite.com/js/universal.js";

  function bootMailerLite() {
    if (typeof window === "undefined") return;

    if (!window.__aliceMailerLiteBooted) {
      (function(w, d, e, u, f, l, n) {
        w[f] = w[f] || function() {
          (w[f].q = w[f].q || []).push(arguments);
        };
        l = d.createElement(e);
        l.async = 1;
        l.src = u;
        n = d.getElementsByTagName(e)[0];
        n.parentNode.insertBefore(l, n);
      })(window, document, "script", mailerLiteScriptUrl, "ml");

      window.__aliceMailerLiteBooted = true;
    }

    if (typeof window.ml === "function") {
      window.ml("account", mailerLiteAccountId);
    }
  }

  function openMailerLitePopup() {
    bootMailerLite();
    window.ml?.("show", mailerLiteFormId, true);
  }

  onMount(() => {
    bootMailerLite();
  });
</script>

<section class="w-full">
  <div class="page-shell">
    <enhanced:img
      src="$lib/assets/lionking.jpeg"
      alt="Lion King"
      sizes="(min-width: 1280px) 1100px, 100vw"
      class="w-full h-auto object-contain"
      fetchpriority="high"
    />
  </div>
</section>

<section class="w-full min-h-screen">
  <div class="page-shell pt-14 pb-10 md:pt-20 md:pb-14">
    <div class="flex w-full flex-col items-center gap-8 lg:flex-row lg:items-center lg:justify-center lg:gap-8">
      <div class="relative w-full max-w-md">
        <enhanced:img
          src="$lib/assets/darkblob.png?w=472"
          sizes="472px"
          alt="Dark blob"
          class="w-full h-auto object-contain"
          decoding="async"
          loading="lazy"
        />
        <enhanced:img
          src="$lib/assets/crosby-cover.png"
          alt="Crosby's Cosmic Adventure cover"
          width="660"
          class="absolute left-1/2 top-1/2 z-10 w-[74%] h-auto -translate-x-1/2 -translate-y-1/2 -rotate-6 object-contain drop-shadow-[0_14px_24px_rgba(0,0,0,0.35)]"
          decoding="async"
          loading="lazy"
        />
      </div>
      <div class="w-full max-w-lg">
        <a
          href="https://catchupwithalice.myshopify.com/"
          target="_blank"
          rel="noopener noreferrer"
          style="width: 100%; display: block;"
        >
          <enhanced:img
            src="$lib/assets/nowavailable.png?w=496"
            sizes="496px"
            alt="Now available - shop now"
            class="w-full h-auto cursor-pointer object-contain"
            decoding="async"
            loading="lazy"
          />
        </a>
      </div>
    </div>
    <div class="mt-6 flex w-full justify-center">
      <a
        href="https://catchupwithalice.myshopify.com/"
        target="_blank"
        rel="noopener noreferrer"
        class="inline-flex items-center gap-2 rounded-full bg-rose-600 px-8 py-3 text-lg font-black uppercase tracking-widest text-white shadow-lg transition hover:bg-rose-700 active:scale-95"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" aria-hidden="true">
          <path stroke-linecap="round" stroke-linejoin="round" d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z" />
        </svg>
        buy now
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" aria-hidden="true">
          <path stroke-linecap="round" stroke-linejoin="round" d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z" />
        </svg>
      </a>
    </div>
    <div class="mt-8 flex w-full justify-center md:mt-10">
      <enhanced:img
        src="$lib/assets/description.png"
        alt="Description"
        sizes="(max-width: 768px) 92vw, 980px"
        class="w-full max-w-5xl h-auto object-contain"
        decoding="async"
        loading="lazy"
      />
    </div>
  </div>
</section>
