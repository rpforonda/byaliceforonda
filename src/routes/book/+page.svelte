<script>
  import { resolve } from "$app/paths";
  import { onMount } from "svelte";

  const lionKingImage = resolve("/lionking.jpeg");
  const darkBlobImage = resolve("/darkblob.png");
  const crosbyCoverImage = resolve("/CrosbyCover.PNG");
  const preorderImage = resolve("/comingsoon.png");
  const descriptionImage = resolve("/description.png");
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
    <img
      src={lionKingImage}
      alt="Lion King"
      width="2240"
      height="1053"
      sizes="100vw"
      class="w-full h-auto object-contain"
      decoding="async"
      loading="lazy"
    />
  </div>
</section>

<section class="w-full min-h-screen">
  <div class="page-shell pt-14 pb-10 md:pt-20 md:pb-14">
    <div class="flex w-full flex-col items-center gap-8 lg:flex-row lg:items-center lg:justify-center lg:gap-8">
      <div class="relative w-full max-w-md">
        <img
          src={darkBlobImage}
          alt="Dark blob"
          width="472"
          height="367"
          sizes="(max-width: 768px) 100vw, 472px"
          class="w-full h-auto object-contain"
          decoding="async"
          loading="lazy"
        />
        <img
          src={crosbyCoverImage}
          alt="Crosby's Cosmic Adventure cover"
          width="1080"
          height="1350"
          sizes="(max-width: 768px) 68vw, 330px"
          class="absolute left-1/2 top-1/2 z-10 w-[74%] h-auto -translate-x-1/2 -translate-y-1/2 -rotate-6 object-contain drop-shadow-[0_14px_24px_rgba(0,0,0,0.35)]"
          decoding="async"
          loading="lazy"
        />
      </div>
      <div class="w-full max-w-lg">
        <button
          type="button"
          class="ml-onclick-form"
          onclick={openMailerLitePopup}
          style="width: 100%; border: 0; background: transparent; padding: 0;"
        >
          <img
            src={preorderImage}
            alt="Join newsletter"
            width="545"
            height="459"
            sizes="(max-width: 768px) 92vw, 560px"
            class="w-full h-auto cursor-pointer object-contain"
            decoding="async"
            loading="lazy"
          />
        </button>
      </div>
    </div>
    <div class="mt-8 flex w-full justify-center md:mt-10">
      <img
        src={descriptionImage}
        alt="Description"
        width="1054"
        height="558"
        sizes="(max-width: 768px) 92vw, 980px"
        class="w-full max-w-5xl h-auto object-contain"
        decoding="async"
        loading="lazy"
      />
    </div>
  </div>
</section>
