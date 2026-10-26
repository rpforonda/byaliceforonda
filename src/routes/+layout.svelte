<script>
  import "../app.css";
  // Self-hosted fonts (bundled from node_modules at build time) so no visitor
  // data is sent to Google Fonts. Only the weights the site actually uses.
  import "@fontsource/nunito/400.css";
  import "@fontsource/nunito/600.css";
  import "@fontsource/nunito/700.css";
  import "@fontsource/poppins/400.css";
  import { resolve } from "$app/paths";
  import { onNavigate } from "$app/navigation";

  const navItems = [
    { href: resolve("/"), label: "home" },
    { href: resolve('about'), label: "about" },
    { href: resolve('book'), label: "book" },
    { href: resolve('freebies'), label: "freebies" },
    { href: resolve('hoksi-haus'), label: "hoksi haus" }
  ];

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

  function handleKeyDown(event) {
    if (event.key === "Escape" && menuOpen) {
      menuOpen = false;
      menuButtonRef?.focus();
    }
  }

  // View Transitions API for page navigation
  onNavigate((navigation) => {
    if (navigation.shallow && navigation.type === 'goto') return;
    if (!document.startViewTransition) return;

    return new Promise((resolve) => {
      document.startViewTransition(async () => {
        resolve();
        await navigation.complete;
      });
    });
  });
</script>

<svelte:window
  onclick={handleWindowClick}
  onkeydown={handleKeyDown}
></svelte:window>

<div
  class="min-h-screen bg-[#d7e6ff] text-gray-900 flex flex-col"
  style="font-family: 'Nunito', sans-serif;"
>
  <!-- Skip to main content link for keyboard users -->
  <a
    href="#main-content"
    class="sr-only focus:not-sr-only focus:absolute focus:z-50 focus:left-4 focus:top-4 focus:rounded-md focus:bg-rose-600 focus:px-4 focus:py-2 focus:text-white focus:shadow-lg focus:outline-none focus:ring-2 focus:ring-rose-600 focus:ring-offset-2"
  >
    Skip to main content
  </a>

  <header class="relative md:fixed md:top-0 left-0 right-0 z-50">
    <div class="bg-white">
      <div
        class="page-shell relative flex items-center justify-between gap-3 py-5 md:block"
      >
        <div class="flex items-center justify-start md:justify-center gap-3">
          <a
            href={resolve("/")}
            class="header-logo-link flex w-fit select-none leading-none"
            aria-label="Go to home"
          >
            <enhanced:img
              src="#lib/assets/Crosby.png"
              alt="Crosby"
              width="144"
              class="block h-12 w-auto md:h-16"
            ></enhanced:img>
          </a>
        </div>
        <div
          class="flex flex-shrink-0 items-center gap-3 md:absolute md:right-6 md:top-1/2 md:-translate-y-1/2"
        >
          <div class="relative md:hidden">
            <button
              class="inline-flex items-center rounded-md bg-rose-600 px-2.5 py-1.5 text-xs font-semibold text-white shadow-sm hover:bg-rose-700 focus:outline-none focus-visible:ring-2 focus-visible:ring-rose-600 focus-visible:ring-offset-2 sm:px-3 sm:py-2 sm:text-sm"
              type="button"
              onclick={toggleMenu}
              aria-expanded={menuOpen ? "true" : "false"}
              aria-controls="mobile-menu"
              aria-label="Toggle navigation menu"
              bind:this={menuButtonRef}
            >
              Menu
            </button>
            {#if menuOpen}
              <div
                id="mobile-menu"
                class="absolute right-0 mt-2 w-44 rounded-md border border-pink-200 bg-white py-2 text-gray-800 shadow-xl z-50"
                role="menu"
                bind:this={menuPanelRef}
              >
                {#each navItems as item}
                  <a
                    class="block px-4 py-2 text-sm font-semibold lowercase hover:bg-[#d7e6ff]"
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

    <div class="bg-[#352f47] text-white">
      <div class="page-shell">
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

  <main id="main-content" class="flex-1 overflow-x-hidden pt-0 pb-0 md:pt-36 lg:pt-40">
    {@render children?.()}
  </main>

  <footer class="relative left-0 right-0 overflow-hidden text-center text-sm">
    <div class="relative h-24 bg-[#d7e6ff]">
      <div class="absolute inset-x-0 bottom-0 h-10 bg-[#c4d7f6]"></div>
      <div class="absolute left-1/2 bottom-[.1rem] z-20 -translate-x-1/2">
        <span
          class="inline-flex items-center justify-center rounded-full bg-[#c4d7f6] p-2"
        >
          <enhanced:img
            src="#lib/assets/alice_logopink.jpg"
            alt="Alice Foronda logo featuring the author"
            width="160"
            class="h-20 w-20 rounded-full object-cover"
            loading="lazy"
          ></enhanced:img>
        </span>
      </div>
    </div>
    <div
      class="relative border-t border-pink-200 bg-stone-800 text-white"
    >
      <div class="flex justify-center pt-3 pb-4">
        <enhanced:img
          src="#lib/assets/al_wordm_white.png"
          alt="Alice Foronda wordmark"
          width="320"
          class="h-6 w-auto md:h-8"
          loading="lazy"
        ></enhanced:img>
      </div>
      <div
        class="page-shell flex flex-col items-center gap-3 pb-6 text-pink-200"
      >
        <div class="flex items-center gap-4">
          <a
            href="https://instagram.com/byaliceforonda"
            target="_blank"
            rel="noopener noreferrer"
            aria-label="Visit Alice Foronda on Instagram"
            class="flex h-10 w-10 items-center justify-center"
          >
            <enhanced:img
              src="#lib/assets/whiteig.png"
              alt="Instagram"
              width="80"
              class="h-10 w-10 cursor-pointer object-contain"
              loading="lazy"
            ></enhanced:img>
          </a>
          <a
            href="https://www.tiktok.com/@catchupwithalice"
            target="_blank"
            rel="noopener noreferrer"
            aria-label="Visit Alice Foronda on TikTok"
            class="flex h-10 w-10 items-center justify-center"
          >
            <enhanced:img
              src="#lib/assets/tiktok.png"
              alt="TikTok"
              width="130"
              class="h-10 w-10 cursor-pointer object-cover"
              loading="lazy"
            ></enhanced:img>
          </a>
        </div>

        <span>
          © {new Date().getFullYear()} Alice Foronda. All rights reserved. This site sets no cookies and uses no analytics.
          <a href={resolve('privacy')} class="font-semibold underline underline-offset-2 hover:text-white">Privacy</a>
        </span>
      </div>
    </div>
  </footer>
</div>

<style>
  .header-logo-link {
    user-select: none;
    -webkit-user-select: none;
    caret-color: transparent;
  }
</style>
