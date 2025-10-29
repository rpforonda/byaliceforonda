<script>
  import "../app.css";
  import { resolve } from "$app/paths";

  const site = {
    title: "Alice Foronda - Official Site",
    desc: "Alice Foronda: about me, book info, and character quiz.",
    //image: "/og-cover.jpg",
  };

  const navItems = [
    { href: resolve("/"), label: "home" },
    { href: resolve("/about"), label: "about" },
    { href: resolve("/book"), label: "books" },
    { href: resolve("/quiz"), label: "quiz" },
  ];

  const logoSrc = resolve("/al_wordm.png");
  const logoWhiteSrc = resolve("/al_wordm_white.png");
  let { children } = $props();
  let menuOpen = $state(false);
  let menuButtonRef = $state(null);
  let menuPanelRef = $state(null);

  function toggleMenu() {
    menuOpen = !menuOpen;
  }

  function handleWindowClick(event) {
    if (!menuOpen) return;
    const target = event.target;
    if (menuButtonRef?.contains(target) || menuPanelRef?.contains(target)) {
      return;
    }
    menuOpen = false;
  }
</script>

<svelte:window onclick={handleWindowClick} />

<svelte:head>
  <title>{site.title}</title>
  <meta name="description" content={site.desc} />
  <meta property="og:title" content={site.title} />
  <meta property="og:description" content={site.desc} />
  <meta property="og:type" content="website" />
  <meta property="og:image" content={site.image} />
  <meta name="twitter:card" content="summary_large_image" />
  <link
    rel="stylesheet"
    href="https://fonts.googleapis.com/css2?family=Great+Vibes&family=Nunito:wght@400;600;700&display=swap"
  />
</svelte:head>

<div
  class="min-h-screen bg-pink-50 text-gray-900 flex flex-col"
  style="font-family: 'Nunito', sans-serif;"
>
  <header class="relative z-40">
    <div class="bg-white">
      <div
        class="relative mx-auto flex max-w-6xl items-center justify-between gap-3 px-6 py-5 md:block"
      >
        <a
          href={resolve("/")}
          class="flex w-fit justify-start md:mx-auto md:justify-center"
        >
          <img
            src={logoSrc}
            alt="Alice Foronda wordmark"
            class="h-12 w-auto md:h-16"
          />
        </a>
        <div
          class="flex flex-shrink-0 items-center gap-3 md:absolute md:right-6 md:top-1/2 md:-translate-y-1/2"
        >
          <a
            href={resolve("/book")}
            class="inline-flex h-9 w-9 items-center justify-center rounded-full border border-rose-600 text-rose-600 shadow-sm transition hover:bg-pink-50 sm:h-10 sm:w-10"
            aria-label="Shop books"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="1.5"
              class="h-4 w-4 sm:h-5 sm:w-5"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M9 7V6a3 3 0 0 1 6 0v1"
              />
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M5.25 7h13.5l-.84 10.08A2.25 2.25 0 0 1 15.69 19.5H8.31a2.25 2.25 0 0 1-2.22-2.42L5.25 7Z"
              />
            </svg>
          </a>
          <div class="relative md:hidden">
            <button
              class="inline-flex items-center rounded-md bg-rose-600 px-2.5 py-1.5 text-xs font-semibold text-white shadow-sm hover:bg-rose-700 focus:outline-none focus-visible:ring-2 focus-visible:ring-rose-600 focus-visible:ring-offset-2 sm:px-3 sm:py-2 sm:text-sm"
              type="button"
              onclick={toggleMenu}
              aria-expanded={menuOpen ? "true" : "false"}
              aria-label="Toggle navigation"
              bind:this={menuButtonRef}
            >
              Menu
            </button>
            {#if menuOpen}
              <div
                class="absolute right-0 mt-2 w-44 rounded-md border border-pink-200 bg-white py-2 text-gray-800 shadow-xl z-50"
                bind:this={menuPanelRef}
              >
                {#each navItems as item}
                  <a
                    class="block px-4 py-2 text-sm font-semibold lowercase hover:bg-pink-50"
                    href={item.href}
                    onclick={() => {
                      menuOpen = false;
                    }}
                  >
                    {item.label}
                  </a>
                {/each}
              </div>
            {/if}
          </div>
        </div>
      </div>
    </div>

    <div class="bg-black text-white">
      <div class="mx-auto max-w-6xl px-6">
        <nav class="hidden justify-center gap-8 py-4 text-sm md:flex">
          {#each navItems as item}
            <a
              class="font-semibold lowercase tracking-[0.05em] transition hover:text-pink-200"
              href={item.href}
            >
              {item.label}
            </a>
          {/each}
        </nav>
      </div>
    </div>
  </header>

  <main class="flex flex-1 flex-col gap-0 overflow-x-hidden">
    {@render children?.()}
  </main>

  <footer class="relative overflow-hidden text-center text-sm">
    <div class="relative h-[12rem] bg-pink-100">
      <div class="absolute inset-x-0 bottom-0 h-10 bg-rose-200"></div>
      <div class="absolute left-1/2 bottom-[.1rem] z-20 -translate-x-1/2">
        <span
          class="inline-flex items-center justify-center rounded-full bg-rose-200 p-2"
        >
          <img
            src={resolve("/alice_logopink.jpg")}
            alt="Alice Foronda emblem"
            class="h-20 w-20 rounded-full object-cover"
          />
        </span>
      </div>
    </div>
    <div
      class="relative border-t border-pink-200 bg-stone-800 pb-10 pt-12 text-white"
    >
      <div class="mb-6 flex justify-center">
        <img
          src={logoWhiteSrc}
          alt="Alice Foronda wordmark"
          class="h-12 w-auto md:h-16"
        />
      </div>
      <div
        class="mx-auto flex max-w-6xl flex-col items-center gap-3 px-6 text-pink-200"
      >
        <a
          class="inline-flex items-center text-sm transition hover:text-white"
          href="https://www.instagram.com/byaliceforonda"
          target="_blank"
          rel="noreferrer"
          aria-label="Follow me @byaliceforonda"
        >
          <span
            class="inline-flex h-8 w-8 items-center justify-center rounded-full border border-pink-200"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="1.5"
              class="h-4 w-4"
            >
              <rect width="14" height="14" x="5" y="5" rx="4" ry="4"></rect>
              <circle cx="12" cy="12" r="3.5"></circle>
              <circle cx="16.5" cy="7.5" r=".8" fill="currentColor"></circle>
            </svg>
          </span>
        </a>
        <span
          >&copy; {new Date().getFullYear()} Alice Foronda. All rights reserved.
          No personal data collected. Purchases happen on retailer sites.</span
        >
      </div>
    </div>
  </footer>
</div>
