<script>
  import { onMount } from "svelte";
  import { resolve } from "$app/paths";
  import Seo from "#lib/Seo.svelte";
  import logo from "#lib/assets/hoksi/hoksi-haus-logo.svg";
  import { FORM_URL, TURNAROUND_DAYS, sizes, addOns } from "#lib/hoksi/data.js";
  import { gallery } from "#lib/hoksi/gallery.js";

  // every gallery photo, processed by enhanced-img at build time (AVIF/WebP + srcset). 400–1080
  // cover a carousel photo up to 3×; 1600 is the full-size view.
  const photos = import.meta.glob("/src/lib/assets/hoksi/gallery/*.{jpg,jpeg,png,webp}", {
    eager: true,
    query: { enhanced: true, w: "400;720;1080;1600" },
    import: "default",
  });

  const banners = gallery.map((banner) => {
    const image = photos[`/src/lib/assets/hoksi/gallery/${banner.file}`];
    if (!image) throw new Error(`Hoksi Haus gallery: no photo named ${banner.file} in src/lib/assets/hoksi/gallery/`);
    // the popover's id, e.g. "banner-under-the-sea" for under-the-sea.jpg
    return { ...banner, image, ratio: image.img.w / image.img.h, id: `banner-${banner.file.replace(/\.[^.]+$/, "")}` };
  });

  // ---------- Endless carousel ----------
  // The photos are rendered three times in a row, and the strip is kept within the middle set:
  // whenever it passes either end of that set it jumps by one set's width, onto identical photos,
  // so the jump can't be seen. The outer sets are hidden from screen readers and the keyboard;
  // tapping one of their photos opens the same full-size view.
  //
  // It drifts left on its own and pauses while the mouse is over it, while a photo has keyboard
  // focus, while a full-size photo is open, for a few seconds after a touch or after closing a
  // photo, and when the pause button is pressed. Visitors who ask for reduced motion start
  // paused. It's still a native scroll strip, so swiping and the arrow buttons work as before.
  const sets = [0, 1, 2]; // 1 is the real one
  const SPEED = 30; // pixels per second, about a photo every 10 seconds
  const HOLD_MS = 3000;

  let track = $state(null);
  let playing = $state(true);
  let setWidth = 0;
  let position = 0; // the strip's scroll position, kept as a fraction between frames
  let hovering = false;
  let focused = false;
  let viewing = false;
  let holding = false;
  let holdTimer;
  let settleTimer;

  function wrap(x) {
    if (x < setWidth) return x + setWidth;
    if (x >= 2 * setWidth) return x - setWidth;
    return x;
  }

  function moveTo(x) {
    position = wrap(x);
    track.scrollLeft = position;
  }

  // set's width from the first photo of one set to the first of the next, keeping the strip at
  // the same point in the set when the photos change size (a resize changes their height)
  function measure() {
    const slides = track.children;
    const fraction = setWidth ? (position - setWidth) / setWidth : 0;
    // fractional, unlike offsetLeft, so the wrap lands exactly on the same spot
    setWidth = slides[banners.length].getBoundingClientRect().left - slides[0].getBoundingClientRect().left;
    moveTo(setWidth * (1 + fraction));
  }

  // the arrow buttons jump about a screenful, straight there with no scrolling animation
  function step(direction) {
    moveTo(position + direction * track.clientWidth * 0.8);
  }

  // scrolled by hand (swipe, trackpad, wheel): follow it, and wrap once it settles
  function handleScroll() {
    if (Math.abs(track.scrollLeft - position) < 2) return; // the drift's own move
    position = track.scrollLeft;
    clearTimeout(settleTimer);
    settleTimer = setTimeout(() => moveTo(track.scrollLeft), 150);
  }

  function holdBriefly() {
    holding = true;
    clearTimeout(holdTimer);
    holdTimer = setTimeout(() => (holding = false), HOLD_MS);
  }

  function handlePointer(event) {
    if (event.pointerType === "mouse") {
      if (event.type === "pointerenter") hovering = true;
      if (event.type === "pointerleave") hovering = false;
    } else if (event.type === "pointerdown") {
      holding = true;
      clearTimeout(holdTimer);
    } else if (event.type === "pointerup" || event.type === "pointercancel") {
      holdBriefly();
    }
  }

  // only keyboard focus: a photo clicked with the mouse keeps focus after the mouse has gone
  function handleFocus(event) {
    focused = event.type === "focusin" && event.target.matches(":focus-visible");
  }

  function photoToggled(event) {
    viewing = event.newState === "open";
    if (!viewing) holdBriefly();
  }

  // Stacked sections stick under the header (see the CSS). One taller than the screen would
  // have its bottom covered before anyone saw it, so it gets --overflow: how much taller it is,
  // and sticks that much higher. It scrolls fully into view first, then holds still.
  let stack = $state(null);

  onMount(() => {
    const sections = [...stack.children];
    const fit = () => {
      const header = parseFloat(getComputedStyle(stack).getPropertyValue("--header")) || 0;
      for (const section of sections) {
        const overflow = Math.max(0, section.offsetHeight - (window.innerHeight - header));
        section.style.setProperty("--overflow", `${overflow}px`);
      }
      if (track) measure();
    };
    const observer = new ResizeObserver(fit);
    for (const section of sections) observer.observe(section);
    window.addEventListener("resize", fit);
    fit();

    if (window.matchMedia("(prefers-reduced-motion: reduce)").matches) playing = false;
    let frame;
    let last;
    const drift = (now) => {
      const moving = playing && !hovering && !focused && !viewing && !holding && setWidth > 0;
      if (moving && last !== undefined) moveTo(position + (SPEED * (now - last)) / 1000);
      last = now;
      frame = requestAnimationFrame(drift);
    };
    if (track) frame = requestAnimationFrame(drift);

    return () => {
      observer.disconnect();
      window.removeEventListener("resize", fit);
      cancelAnimationFrame(frame);
      clearTimeout(holdTimer);
      clearTimeout(settleTimer);
    };
  });
</script>

<Seo
  title="Hoksi Haus | Hand-Painted Party Banners"
  description="Custom hand-painted banners for birthdays, holidays, graduations and every kind of celebration, in five sizes from €15. Request yours through the order form."
  path="/hoksi-haus"
/>

<div bind:this={stack} class="hoksi-page text-[#3c1f08]">
  <section class="hoksi-sheet">
    <div class="page-shell flex flex-col items-center py-12 text-center md:py-16">
      <h1>
        <img src={logo} alt="Hoksi Haus" width="268" height="220" class="h-auto w-48 md:w-60" fetchpriority="high" />
      </h1>
      <p class="mt-6 max-w-xl text-lg font-semibold md:text-xl">
        Custom hand-painted banners for birthdays, holidays, graduations and every kind of celebration.
      </p>
      <a href="#how-to-order" class="hoksi-button mt-6">how to order</a>
    </div>
  </section>

  <section aria-labelledby="gallery-heading" class="hoksi-sheet">
    <div class="page-shell py-8">
      <h2 id="gallery-heading" class="text-center text-2xl font-black lowercase tracking-[0.04em] md:text-3xl">
        recent work
      </h2>
      {#if banners.length}
        <!-- An endless filmstrip: the photos sit side by side at one height, each at its own
             width, and drift slowly to the left (see the script). Tapping one opens it full size
             in a popover, with no JavaScript. -->
        <div
          class="mt-6"
          role="region"
          aria-roledescription="carousel"
          aria-labelledby="gallery-heading"
          onpointerenter={handlePointer}
          onpointerleave={handlePointer}
          onpointerdown={handlePointer}
          onpointerup={handlePointer}
          onpointercancel={handlePointer}
          onfocusin={handleFocus}
          onfocusout={handleFocus}
        >
          <div class="relative">
            <ul bind:this={track} onscroll={handleScroll} class="carousel-track">
              {#each sets as set (set)}
                {#each banners as banner, i (banner.file)}
                  {@const real = set === 1}
                  <li
                    class="carousel-slide"
                    aria-hidden={real ? undefined : "true"}
                    aria-roledescription={real ? "slide" : undefined}
                    aria-label={real ? `${i + 1} of ${banners.length}` : undefined}
                  >
                    <button
                      type="button"
                      popovertarget={banner.id}
                      tabindex={real ? undefined : -1}
                      class="banner-thumb hoksi-card block cursor-zoom-in overflow-hidden"
                      aria-label="View {banner.title} full size"
                    >
                      <enhanced:img
                        src={banner.image}
                        alt={banner.alt}
                        sizes="(max-width: 640px) 85vw, {Math.round(440 * banner.ratio)}px"
                        class="carousel-image"
                        style="--ratio: {banner.ratio}"
                        loading="lazy"
                        decoding="async"
                      />
                    </button>

                    {#if real}
                      <!-- Closed popovers aren't rendered, so the lazy full-size photo only
                           downloads when it's opened. Esc, a click outside or the close button
                           shuts it. -->
                      <div id={banner.id} popover class="lightbox" role="dialog" aria-label={banner.title} ontoggle={photoToggled}>
                        <enhanced:img
                          src={banner.image}
                          alt={banner.alt}
                          sizes="94vw"
                          class="lightbox-image"
                          style="--ratio: {banner.ratio}"
                          loading="lazy"
                          decoding="async"
                        />
                        <div class="mt-2 flex justify-end">
                          <button type="button" popovertarget={banner.id} popovertargetaction="hide" class="hoksi-button py-1.5! text-sm!">
                            close <span aria-hidden="true">✕</span>
                          </button>
                        </div>
                      </div>
                    {/if}
                  </li>
                {/each}
              {/each}
            </ul>
            <button type="button" class="carousel-button left-1" onclick={() => step(-1)} aria-label="Previous photos">
              <span aria-hidden="true">‹</span>
            </button>
            <button type="button" class="carousel-button right-1" onclick={() => step(1)} aria-label="Next photos">
              <span aria-hidden="true">›</span>
            </button>
          </div>
          <div class="flex justify-center">
            <button
              type="button"
              class="carousel-toggle"
              onclick={() => (playing = !playing)}
              aria-label={playing ? "Pause the photos" : "Play the photos"}
            >
              <span aria-hidden="true">{playing ? "❚❚" : "▶"}</span>
            </button>
          </div>
        </div>
      {:else}
        <p class="hoksi-card mx-auto mt-6 max-w-md px-6 py-8 text-center">
          Photos of recent work are coming soon.
        </p>
      {/if}
    </div>
  </section>

  <section aria-labelledby="prices-heading" class="hoksi-sheet">
    <div class="page-shell py-8">
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
    </div>
  </section>

  <section
    id="how-to-order"
    aria-labelledby="order-heading"
    class="hoksi-sheet scroll-mt-6 md:scroll-mt-[156px]"
  >
    <div class="page-shell pt-8 pb-16 md:pb-20">
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

  /* ---------- Stacked sections ----------
     Each section is a full-width sheet at least a screen tall that sticks under the header,
     so the next one slides up over it like paper laid on a pile. Nothing animates: it's plain
     sticky positioning. --header is how much of the screen the fixed header covers (none on
     phones, where it scrolls away); --overflow is set by the script for sheets taller than the
     screen. The last sheet doesn't stick, so the footer follows it normally. */
  .hoksi-page {
    --header: 0px;
  }

  @media (min-width: 768px) {
    .hoksi-page {
      --header: 156px;
    }
  }

  /* main's overflow-x: hidden makes it a scroll container, so the sheets would stick inside it
     instead of the page; clip hides the overflow the same way without that */
  :global(main:has(> .hoksi-page)) {
    overflow-x: clip;
  }

  .hoksi-sheet {
    /* positioned like the sticky ones, so the last sheet still paints over the pile */
    position: relative;
    display: grid;
    align-content: center;
    min-height: calc(100svh - var(--header));
    background-color: #d7e6ff;
  }

  .hoksi-sheet:not(:last-child) {
    position: sticky;
    top: calc(var(--header) - var(--overflow, 0px));
  }

  /* a soft shadow along each sheet's top edge, cast onto the sheet it covers */
  .hoksi-sheet + .hoksi-sheet {
    box-shadow: 0 -12px 28px rgba(60, 31, 8, 0.12);
  }

  /* ---------- Carousel ---------- */
  .carousel-track {
    display: flex;
    /* a wide photo that had to get shorter sits level with the middle of its neighbors */
    align-items: center;
    gap: 1rem;
    overflow-x: auto;
    overscroll-behavior-x: contain;
    scrollbar-width: none;
    /* room for the cards' shadows, which the scroller would otherwise clip */
    padding: 0.5rem 0.25rem 1.5rem;
  }

  .carousel-track::-webkit-scrollbar {
    display: none;
  }

  .carousel-slide {
    flex: none;
  }

  /* One height for every photo, each keeping its own shape (--ratio is width / height), except
     that a wide photo gets shorter rather than wider than the screen. */
  .carousel-track :global(.carousel-image) {
    display: block;
    height: min(clamp(260px, 48vh, 440px), calc(85vw / var(--ratio)));
    width: auto;
  }

  /* the card clips its corners, so keep the photo button's focus ring inside it */
  .banner-thumb:focus-visible {
    outline-offset: -4px;
  }

  .carousel-button {
    position: absolute;
    top: calc(50% - 0.5rem);
    translate: 0 -50%;
    display: grid;
    place-items: center;
    width: 2.75rem;
    height: 2.75rem;
    border-radius: 9999px;
    background-color: #3c1f08;
    color: #fff6e3;
    font-size: 1.75rem;
    line-height: 1;
    box-shadow: 0 6px 14px rgba(60, 31, 8, 0.3);
  }

  .carousel-toggle {
    display: grid;
    place-items: center;
    width: 2.25rem;
    height: 2.25rem;
    border-radius: 9999px;
    background-color: #3c1f08;
    color: #fff6e3;
    font-size: 0.8rem;
    line-height: 1;
    box-shadow: 0 4px 10px rgba(60, 31, 8, 0.3);
  }

  .carousel-button:hover,
  .carousel-toggle:hover {
    background-color: #5a3214;
  }

  /* ---------- Full-size view ----------
     A popover per photo: the browser puts it in the top layer, centers it and closes it on Esc
     or a click outside. */
  .lightbox {
    /* browsers center popovers with margin: auto, which Tailwind's reset sets to 0 */
    margin: auto;
    max-width: 94vw;
    max-height: 94vh;
    padding: 0.75rem;
    border: 2px solid #3c1f08;
    border-radius: 1rem;
    background-color: #fff6e3;
    color: #3c1f08;
    box-shadow: 0 20px 50px rgba(0, 0, 0, 0.35);
  }

  .lightbox::backdrop {
    background-color: rgba(60, 31, 8, 0.75);
  }

  /* As large as the screen allows for the photo's shape (--ratio is width / height). The size
     is set before the photo loads, because a lazy image with no size is never fetched. */
  .lightbox :global(.lightbox-image) {
    width: min(calc(94vw - 1.75rem), calc((94vh - 5rem) * var(--ratio)));
    height: auto;
    border-radius: 0.5rem;
  }
</style>
