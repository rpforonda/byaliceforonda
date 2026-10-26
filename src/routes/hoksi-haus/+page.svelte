<script>
  import { resolve } from "$app/paths";
  import Seo from "#lib/Seo.svelte";
  import logo from "#lib/assets/hoksi/hoksi-haus-logo.svg";
  import { FORM_URL, TURNAROUND_DAYS, sizes, addOns } from "#lib/hoksi/data.js";
  import { gallery } from "#lib/hoksi/gallery.js";

  // every gallery photo, processed by enhanced-img at build time (AVIF/WebP + srcset). The
  // widths cover a half-width phone column up to a 360px desktop column at 3×.
  const photos = import.meta.glob("/src/lib/assets/hoksi/gallery/*.{jpg,jpeg,png,webp}", {
    eager: true,
    query: { enhanced: true, w: "400;720;1080" },
    import: "default",
  });

  const banners = gallery.map((banner) => {
    const image = photos[`/src/lib/assets/hoksi/gallery/${banner.file}`];
    if (!image) throw new Error(`Hoksi Haus gallery: no photo named ${banner.file} in src/lib/assets/hoksi/gallery/`);
    return { ...banner, image };
  });
</script>

<Seo
  title="Hoksi Haus | Hand-Painted Party Banners"
  description="Custom hand-painted banners for birthdays, holidays, graduations and every kind of celebration, in five sizes from €15. Request yours through the order form."
  path="/hoksi-haus"
/>

<div class="text-[#3c1f08]">
  <section class="page-shell flex flex-col items-center pt-12 pb-8 text-center md:pt-16">
    <h1>
      <img src={logo} alt="Hoksi Haus" width="268" height="220" class="h-auto w-48 md:w-60" fetchpriority="high" />
    </h1>
    <p class="mt-6 max-w-xl text-lg font-semibold md:text-xl">
      Custom hand-painted banners for birthdays, holidays, graduations and every kind of celebration.
    </p>
    <a href="#how-to-order" class="hoksi-button mt-6">how to order</a>
  </section>

  <section aria-labelledby="gallery-heading" class="page-shell py-8">
    <h2 id="gallery-heading" class="text-center text-2xl font-black lowercase tracking-[0.04em] md:text-3xl">
      recent banners
    </h2>
    {#if banners.length}
      <ul class="mt-6 columns-2 gap-4 md:columns-3">
        {#each banners as banner (banner.file)}
          <li class="mb-4 break-inside-avoid">
            <figure class="hoksi-card overflow-hidden">
              <enhanced:img
                src={banner.image}
                alt={banner.alt}
                sizes="(min-width: 768px) 360px, 50vw"
                class="h-auto w-full"
                loading="lazy"
                decoding="async"
              />
              <figcaption class="px-3 py-2 text-sm font-semibold">{banner.title}</figcaption>
            </figure>
          </li>
        {/each}
      </ul>
    {:else}
      <p class="hoksi-card mx-auto mt-6 max-w-md px-6 py-8 text-center">
        Photos of recent banners are coming soon.
      </p>
    {/if}
  </section>

  <section aria-labelledby="prices-heading" class="page-shell py-8">
    <h2 id="prices-heading" class="text-center text-2xl font-black lowercase tracking-[0.04em] md:text-3xl">
      sizes &amp; prices
    </h2>
    <div class="hoksi-card mx-auto mt-6 max-w-lg p-5 md:p-6">
      <table class="w-full text-left">
        <caption class="sr-only">Banner sizes and prices</caption>
        <thead>
          <tr class="border-b-2 border-[#3c1f08]/25 text-xs uppercase tracking-[0.12em]">
            <th scope="col" class="pb-2">size</th>
            <th scope="col" class="pb-2">dimensions</th>
            <th scope="col" class="pb-2 text-right">price</th>
          </tr>
        </thead>
        <tbody>
          {#each sizes as { size, dimensions, price } (size)}
            <tr class="border-b border-[#3c1f08]/15 last:border-0">
              <th scope="row" class="py-2.5 font-black">{size}</th>
              <td class="py-2.5">{dimensions}</td>
              <td class="py-2.5 text-right font-bold">€{price}</td>
            </tr>
          {/each}
        </tbody>
      </table>

      <h3 class="mt-6 text-lg font-black lowercase">add-ons</h3>
      <ul class="mt-2 space-y-2">
        {#each addOns as { label, price } (label)}
          <li class="flex justify-between gap-4">
            <span>{label}</span>
            <span class="whitespace-nowrap font-bold">{price}</span>
          </li>
        {/each}
      </ul>

      <p class="mt-6 border-t-2 border-[#3c1f08]/25 pt-4 font-semibold">
        Standard turnaround is {TURNAROUND_DAYS} days.
      </p>
    </div>
  </section>

  <section
    id="how-to-order"
    aria-labelledby="order-heading"
    class="page-shell scroll-mt-6 pt-8 pb-16 md:scroll-mt-44 md:pb-20"
  >
    <h2 id="order-heading" class="text-center text-2xl font-black lowercase tracking-[0.04em] md:text-3xl">
      how to order
    </h2>
    <ol class="mx-auto mt-6 grid max-w-4xl gap-4 md:grid-cols-3">
      <li class="hoksi-card p-5">
        <span class="hoksi-step" aria-hidden="true">1</span>
        <h3 class="mt-3 text-lg font-black lowercase">send a request</h3>
        <p class="mt-1">Fill in the order form with the date you need it, the size and your design ideas.</p>
      </li>
      <li class="hoksi-card p-5">
        <span class="hoksi-step" aria-hidden="true">2</span>
        <h3 class="mt-3 text-lg font-black lowercase">agree the details</h3>
        <p class="mt-1">
          Hoksi Haus replies by email about the design, price and availability. Your order is
          confirmed then; sending the form doesn't reserve a date.
        </p>
      </li>
      <li class="hoksi-card p-5">
        <span class="hoksi-step" aria-hidden="true">3</span>
        <h3 class="mt-3 text-lg font-black lowercase">it's painted for you</h3>
        <p class="mt-1">Your banner is painted by hand, with a standard turnaround of {TURNAROUND_DAYS} days.</p>
      </li>
    </ol>

    <div class="mt-10 flex flex-col items-center gap-4 text-center">
      <a href={FORM_URL} target="_blank" rel="noopener noreferrer" class="hoksi-button">
        request a banner <span aria-hidden="true">↗</span>
        <span class="sr-only">(opens the Google Form in a new tab)</span>
      </a>
      <p class="max-w-md text-sm">
        Under 13? Please ask a parent or guardian to send the request for you. The order form is
        a Google Form, and what you send is used only to reply about your banner.
        <a href={resolve("/privacy")} class="font-semibold underline underline-offset-2">Privacy</a>
      </p>
    </div>
  </section>
</div>

<style>
  /* cream panels barely stand out from the light-blue page, so they get a brown edge */
  .hoksi-card {
    border: 2px solid #3c1f08;
    border-radius: 1rem;
    background-color: #fff6e3;
    box-shadow: 0 10px 24px rgba(60, 31, 8, 0.12);
  }

  .hoksi-button {
    display: inline-block;
    border-radius: 9999px;
    background-color: #3c1f08;
    padding: 0.625rem 1.5rem;
    font-weight: 700;
    text-transform: lowercase;
    color: #fff6e3;
    box-shadow: 0 6px 14px rgba(60, 31, 8, 0.3);
  }

  .hoksi-button:hover {
    background-color: #5a3214;
  }

  .hoksi-step {
    display: inline-flex;
    height: 2rem;
    width: 2rem;
    align-items: center;
    justify-content: center;
    border-radius: 9999px;
    background-color: #3c1f08;
    font-weight: 900;
    color: #fff6e3;
  }
</style>
