<script>
  import { onMount } from "svelte";
  import Seo from "#lib/Seo.svelte";
  import BuyButtons from "#lib/BuyButtons.svelte";
  import { AMAZON_URL, author, book } from "#lib/seo.js";

  const jsonLd = { "@context": "https://schema.org", "@graph": [book, author] };

  const mailerLiteAccountId = "2261437";
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

  onMount(() => {
    bootMailerLite();
  });
</script>

<Seo
  title="Crosby's Cosmic Adventure | Picture Book by Alice Foronda"
  description="Crosby the crocodile is nervous about trying something new. With help from Carla and Tank, he discovers how brightly he can shine. A starry picture book for ages 4–8."
  path="/book"
  {jsonLd}
/>

<h1 class="sr-only">Crosby's Cosmic Adventure by Alice Foronda</h1>

<section class="w-full">
  <div class="page-shell">
    <enhanced:img
      src="#lib/assets/lionking.jpeg"
      alt="Crosby, Carla and Tank gazing up at a starry night sky, an illustration from Crosby's Cosmic Adventure"
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
          src="#lib/assets/darkblob.png?w=472"
          sizes="472px"
          alt=""
          aria-hidden="true"
          class="w-full h-auto object-contain"
          decoding="async"
          loading="lazy"
        />
        <enhanced:img
          src="#lib/assets/crosby-cover.png"
          alt="Crosby's Cosmic Adventure cover"
          width="660"
          class="absolute left-1/2 top-1/2 z-10 w-[74%] h-auto -translate-x-1/2 -translate-y-1/2 -rotate-6 object-contain drop-shadow-[0_14px_24px_rgba(0,0,0,0.35)]"
          decoding="async"
          loading="lazy"
        />
      </div>
      <div class="w-full max-w-lg">
        <a
          href={AMAZON_URL}
          target="_blank"
          rel="noopener noreferrer"
          style="width: 100%; display: block;"
        >
          <enhanced:img
            src="#lib/assets/nowavailable.png?w=496"
            sizes="496px"
            alt="Now available: buy Crosby's Cosmic Adventure"
            class="w-full h-auto cursor-pointer object-contain"
            decoding="async"
            loading="lazy"
          />
        </a>
      </div>
    </div>
    <BuyButtons class="mt-6" />
    <div class="mt-8 flex w-full justify-center md:mt-10">
      <enhanced:img
        src="#lib/assets/description.png"
        alt="About the book"
        aria-describedby="book-description"
        sizes="(max-width: 768px) 92vw, 980px"
        class="w-full max-w-5xl h-auto object-contain"
        decoding="async"
        loading="lazy"
      />
      <!-- The description is artwork; this is its text for screen readers and search engines. -->
      <div id="book-description" class="sr-only">
        <h2>About the book</h2>
        <p>"My tail's too thick. And my claws too long. Don't you think I'll do this all wrong?"</p>
        <p>
          Have you ever felt a little nervous about trying something new? Or had a misunderstanding
          with friends? You're not alone.
        </p>
        <p>
          Crosby the crocodile faces these challenges too. But with a little courage and some help
          from his friends Carla the capybara and Tank the turtle, Crosby discovers just how
          brightly he can shine.
        </p>
        <p>
          Perfect for ages 4–8. Join Crosby and his crew on a starry adventure where they discover
          an obstacle course in space and learn what it means to be a true friend.
        </p>
        <p>A heartwarming story about bravery, belonging, and believing in yourself.</p>
      </div>
    </div>
  </div>
</section>
