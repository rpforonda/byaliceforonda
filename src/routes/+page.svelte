<script>
  import { resolve } from "$app/paths";
  import { onMount } from "svelte";

  const crosbyFrameImage = resolve("/Crosbyframe.png");
  const greenBlobImage = resolve("/greenblob.png");
  const heroPopupImage = resolve("/cosmic.jpeg");
  const meetCharactersImage = resolve("/mtc.png");
  const joinCrosbyImage = resolve("/joincrosby.png");
  const creamBlobImage = resolve("/creamblob.png");
  const crosbyCoverHomeImage = resolve("/crosbycoverhome.png");
  const crosbyGreenImage = resolve("/crosbygreen.png");
  const carlaPinkImage = resolve("/carlapink.png");
  const tankYellowImage = resolve("/tankyellow.png");
  const mailerLiteFormId = "QQryIS";
  const mailerLitePopupUrl = `https://assets.mailerlite.com/jsonp/2261437/forms/${mailerLiteFormId}/content`;
  const characterStars = [
    {
      left: "6%",
      size: "1.625rem",
      duration: "4s",
      delay: "0.3s",
      rotate: "-8deg",
    },
    {
      left: "14%",
      size: "2.125rem",
      duration: "4.5s",
      delay: "0.6s",
      rotate: "12deg",
    },
    {
      left: "22%",
      size: "1.75rem",
      duration: "3.8s",
      delay: "0.1s",
      rotate: "-6deg",
    },
    {
      left: "30%",
      size: "2.625rem",
      duration: "4.8s",
      delay: "0.9s",
      rotate: "10deg",
    },
    {
      left: "38%",
      size: "1.875rem",
      duration: "3.6s",
      delay: "0.4s",
      rotate: "-12deg",
    },
    {
      left: "46%",
      size: "3.125rem",
      duration: "5s",
      delay: "0.2s",
      rotate: "8deg",
    },
    {
      left: "54%",
      size: "2rem",
      duration: "3.9s",
      delay: "1s",
      rotate: "-10deg",
    },
    {
      left: "62%",
      size: "2.25rem",
      duration: "4.2s",
      delay: "0.5s",
      rotate: "6deg",
    },
    {
      left: "70%",
      size: "2.75rem",
      duration: "4.6s",
      delay: "0.7s",
      rotate: "-9deg",
    },
    {
      left: "78%",
      size: "1.75rem",
      duration: "3.5s",
      delay: "1.2s",
      rotate: "11deg",
    },
    {
      left: "86%",
      size: "2.375rem",
      duration: "4.3s",
      delay: "0.8s",
      rotate: "-7deg",
    },
    {
      left: "94%",
      size: "1.875rem",
      duration: "3.7s",
      delay: "1.4s",
      rotate: "5deg",
    },
  ];
  const characters = [
    {
      name: "Crosby the Crocodile",
      role: "Gentle dreamer",
      summary:
        "Crosby is gentle, curious, and a little shy. With friends who cheer him on, he learns that the very traits he once hid make him a hero in the hearts of others. Courage, kindness, and a dash of starlight guide his adventures.",
      traits: [
        { label: "Personality", value: "Gentle, curious, thoughtful" },
        { label: "Loves", value: "Moonlit swims, sketching stars" },
        { label: "Needs", value: "Encouragement and warm hugs" },
        { label: "Superpower", value: "Soft bravery that grows" },
      ],
      image: resolve("/Crosby.png"),
      imageWebp: resolve("/Crosby.png"),
      accent: "#b8cee8",
    },
    {
      name: "Carla the Capybara",
      role: "Fearless navigator",
      summary:
        "Carla keeps the crew together with calm confidence. She loves planning adventures, lifting spirits, and reminding everyone that bravery is something we practice together.",
      traits: [
        { label: "Personality", value: "Steady, kind, collaborative" },
        { label: "Loves", value: "Maps, music, and midnight snacks" },
        { label: "Needs", value: "Quiet time to refuel her heart" },
        { label: "Superpower", value: "Leading with empathy" },
      ],
      image: resolve("/Carla.png"),
      imageWebp: resolve("/Carla.webp"),
      accent: "#ef6f3c",
    },
    {
      name: "Tank the Turtle",
      role: "Inventor and tinkerer",
      summary:
        "Tank is endlessly curious and always building something new. He's the friend who experiments until the impossible suddenly works, proving that patience can be its own kind of magic.",
      traits: [
        { label: "Personality", value: "Inventive, patient, playful" },
        { label: "Loves", value: "Gadgets, puzzles, cozy corners" },
        { label: "Needs", value: "Time to think and tinker" },
        { label: "Superpower", value: "Turning ideas into reality" },
      ],
      image: resolve("/Tank.png"),
      imageWebp: resolve("/Tank.webp"),
      accent: "#afab23",
    },
  ];

  // Track scroll state for welcome and logo visibility
  let showWelcome = $state(true);
  let isMailerLiteOpen = $state(false);
  let mailerLiteFrameSrc = $state("");
  let mailerLiteAutoOpened = false;

  function updateMailerLiteFrameSrc() {
    if (typeof window === "undefined") return;

    const frameWidth = Math.max(320, Math.min(window.innerWidth - 32, 765));
    mailerLiteFrameSrc = `${mailerLitePopupUrl}?windowWidth=${Math.round(frameWidth)}`;
  }

  function openMailerLitePopup() {
    isMailerLiteOpen = true;
  }

  function closeMailerLitePopup() {
    isMailerLiteOpen = false;
  }

  function maybeAutoOpenMailerLite() {
    if (mailerLiteAutoOpened || isMailerLiteOpen) return;

    mailerLiteAutoOpened = true;
    openMailerLitePopup();
  }

  function handleMailerLiteBackdropClick(event) {
    if (event.currentTarget === event.target) {
      closeMailerLitePopup();
    }
  }

  $effect(() => {
    if (typeof document === "undefined") return;

    const previousOverflow = document.body.style.overflow;
    document.body.style.overflow = isMailerLiteOpen ? "hidden" : "";

    return () => {
      document.body.style.overflow = previousOverflow;
    };
  });

  // Handle scroll to hide welcome and logo
  $effect(() => {
    if (typeof window === 'undefined') return;

    const handleScroll = () => {
      showWelcome = window.scrollY <= 50;
    };

    window.addEventListener('scroll', handleScroll);
    return () => window.removeEventListener('scroll', handleScroll);
  });

  // Scroll-triggered animations - repeatable on every scroll
  onMount(() => {
    updateMailerLiteFrameSrc();

    const autoPopupTimeout = window.setTimeout(() => {
      maybeAutoOpenMailerLite();
    }, 5000);

    const handleResize = () => {
      updateMailerLiteFrameSrc();
    };

    const handlePopupScroll = () => {
      const scrollableHeight =
        document.documentElement.scrollHeight - window.innerHeight;

      if (scrollableHeight <= 0) return;

      if (window.scrollY / scrollableHeight >= 0.75) {
        maybeAutoOpenMailerLite();
      }
    };

    const handlePopupKeyDown = (event) => {
      if (event.key === "Escape" && isMailerLiteOpen) {
        closeMailerLitePopup();
      }
    };

    window.addEventListener("resize", handleResize);
    window.addEventListener("scroll", handlePopupScroll, { passive: true });
    window.addEventListener("keydown", handlePopupKeyDown);

    // Small delay to ensure layout is complete
    let observer;
    const revealTimer = window.setTimeout(() => {
      observer = new IntersectionObserver(
        (entries) => {
          entries.forEach((entry) => {
            if (entry.isIntersecting) {
              entry.target.classList.add('animate-in');
            } else {
              entry.target.classList.remove('animate-in');
            }
          });
        },
        {
          threshold: 0.05,
          rootMargin: '0% 0% -12% 0%',
        }
      );

      const elements = document.querySelectorAll('.scroll-reveal');
      elements.forEach((el) => {
        observer.observe(el);
      });
    }, 150);

    return () => {
      window.clearTimeout(autoPopupTimeout);
      window.clearTimeout(revealTimer);
      observer?.disconnect();
      window.removeEventListener("resize", handleResize);
      window.removeEventListener("scroll", handlePopupScroll);
      window.removeEventListener("keydown", handlePopupKeyDown);
    };
  });

</script>

<section
  class="relative w-full overflow-hidden bg-[#d7e6ff]"
>
  <div class="page-shell relative z-10 transition-all duration-700 ease-out" style={showWelcome ? 'padding-top: 4rem; padding-bottom: 2.5rem;' : 'padding-top: 1.5rem; padding-bottom: 1.5rem;'}>
    <div class="flex flex-col lg:flex-row items-start justify-start gap-8 lg:gap-8 transition-all duration-700 ease-out">
      <div class="w-full max-w-[21rem] lg:max-w-[26rem] shrink-0 mx-auto lg:mx-0 self-center lg:self-auto">
        <img
          src={crosbyFrameImage}
          alt="Crosby frame artwork"
          class="w-full h-auto object-contain"
          loading="lazy"
        />
      </div>
      <!-- Welcome Section -->
      <div
        class="welcome-content max-w-xl transition-all duration-700 ease-out origin-left"
        style={showWelcome
          ? 'opacity: 1; transform: scale(1) translateX(0); max-height: 62.5rem; pointer-events: auto;'
          : 'opacity: 0; transform: scale(0.9) translateX(-1.25rem); max-height: 0; pointer-events: none; overflow: hidden;'}
      >
        <img
          src={heroPopupImage}
          alt="Crosby's Cosmic Adventure"
          class="hero-popup-image"
          loading="lazy"
        />
      </div>
    </div>
  </div>
</section>

<section class="relative w-full overflow-hidden bg-[#d7e6ff] pt-6 pb-8 sm:pt-3 sm:pb-10">
  <div class="page-shell green-blob-mask">
    <img
      src={greenBlobImage}
      alt=""
      aria-hidden="true"
      class="green-blob-bg"
      loading="lazy"
    />
    <div class="green-blob-content">
      <div class="green-blob-col">
        <div class="mt-2 flex justify-center sm:mt-4 lg:mt-6">
          <img
            src={joinCrosbyImage}
            alt="Join Crosby"
            class="w-full max-w-lg h-auto object-contain lg:max-w-xl"
            loading="lazy"
          />
        </div>
        <div class="mt-3 flex justify-center">
          <div class="relative w-full max-w-md lg:max-w-[30rem]">
            <img
              src={creamBlobImage}
              alt="Cream blob"
              width="591"
              height="422"
              sizes="(max-width: 768px) 86vw, 560px"
              class="w-full h-auto object-contain"
              decoding="async"
              loading="lazy"
            />
            <div class="pointer-events-none absolute inset-0 flex items-center justify-center px-6 sm:px-10">
              <p
                class="poppins-regular max-w-[34ch] text-center text-black text-[clamp(0.8rem,1.25vw,1.05rem)] leading-[1.35]"
              >
                Join Crosby, Carla the capybara, and Tank the turtle on an
                adventure of a lifetime!
                <br /><br />
                But even the best adventures can bring big feelings.
                <br /><br />
                When misunderstandings pop up and self-doubt sneaks in, the
                friends must find the courage to listen, speak up, and be there
                for one another. A heartwarming story about friendship, bravery,
                and believing in yourself.
              </p>
            </div>
          </div>
        </div>
      </div>
      <div class="green-blob-col green-blob-col-right">
        <img
          src={crosbyCoverHomeImage}
          alt="Crosby Cover"
          class="w-[85%] h-auto object-contain"
          loading="lazy"
        />
      </div>
    </div>
  </div>
</section>

{#if isMailerLiteOpen}
  <div
    class="mailerlite-modal-backdrop"
    role="presentation"
    onclick={handleMailerLiteBackdropClick}
  >
    <div
      class="mailerlite-modal-shell"
      role="dialog"
      aria-modal="true"
      aria-labelledby="mailerlite-modal-title"
    >
      <h2 id="mailerlite-modal-title" class="sr-only">
        Join Crosby's Crew newsletter signup
      </h2>
      <button
        class="mailerlite-modal-close"
        type="button"
        aria-label="Close newsletter popup"
        onclick={closeMailerLitePopup}
      >
        <span aria-hidden="true">&times;</span>
      </button>
      <div class="mailerlite-modal-frame-wrap">
        <div class="mailerlite-modal-mask mailerlite-modal-mask-top" aria-hidden="true"></div>
        <iframe
          title="Join Crosby's Crew newsletter signup"
          src={mailerLiteFrameSrc}
          class="mailerlite-modal-frame"
          loading="eager"
          allow="clipboard-write"
        ></iframe>
      </div>
    </div>
  </div>
{/if}

<section
  class="characters-section relative w-full overflow-hidden bg-blue-900 text-white scroll-reveal"
>
  <div class="pointer-events-none absolute inset-0 opacity-80">
    {#each characterStars as star}
      <span
        class="home-falling-star"
        style={`left:${star.left}; --size:${star.size}; animation-duration:${star.duration}; animation-delay:${star.delay}; --rotate:${star.rotate};`}
        aria-hidden="true"
      >
        <svg viewBox="0 0 100 100" aria-hidden="true" focusable="false">
          <polygon
            points="50,6 62,38 95,38 68,57 78,91 50,72 22,91 32,57 5,38 38,38"
            fill="white"
            stroke="white"
            stroke-width="6"
            stroke-linejoin="round"
            stroke-linecap="round"
          />
        </svg>
      </span>
    {/each}
  </div>
  <div class="page-shell relative z-10 py-16 sm:py-20">
    <div class="-mt-4 mb-10 flex justify-center sm:-mt-6">
      <img
        src={meetCharactersImage}
        alt="Meet the Characters"
        width="1252"
        height="199"
        sizes="(max-width: 768px) 92vw, 980px"
        class="w-full max-w-5xl h-auto object-contain"
        decoding="async"
        loading="lazy"
      />
    </div>
    <div class="grid items-stretch gap-8 sm:grid-cols-2 lg:grid-cols-3">
      {#each characters as character, index}
        <article class="flex h-full flex-col rounded-3xl bg-white/10 p-6 backdrop-blur-sm shadow-[0_15px_40px_rgba(0,0,0,0.2)]">
          <div class="flex flex-col h-full">
            {#if index === 0}
              <img
                src={crosbyGreenImage}
                alt="Crosby the Crocodile"
                class="mx-auto w-full max-w-xs h-auto object-contain mb-6"
                loading="lazy"
              />
            {:else if index === 1}
              <img
                src={carlaPinkImage}
                alt="Carla the Capybara"
                class="mx-auto w-full max-w-xs h-auto object-contain mb-6"
                loading="lazy"
              />
            {:else if index === 2}
              <img
                src={tankYellowImage}
                alt="Tank the Turtle"
                class="mx-auto w-full max-w-xs h-auto object-contain mb-6"
                loading="lazy"
              />
            {/if}

            <div class="flex-1 flex flex-col">
              <h2 class="text-center text-2xl font-['Inter',_sans-serif] font-semibold tracking-tight">
                {character.name}
              </h2>
              <p
                class="mt-1 text-center text-sm font-semibold uppercase tracking-[0.12em]"
                style={`color:${character.accent}`}
              >
                {character.role}
              </p>
              <p class="mt-4 text-center text-sm leading-relaxed text-[#ffe8f0]">
                {character.summary}
              </p>

              <div class="mt-5 grid gap-3 grid-cols-1">
                {#each character.traits as trait}
                  <div class="rounded-2xl bg-white/10 p-3 text-sm flex flex-col min-h-[5rem]">
                    <div
                      class="text-[0.6875rem] font-semibold uppercase tracking-[0.2em]"
                      style={`color:${character.accent}`}
                    >
                      {trait.label}
                    </div>
                    <div class="mt-1 text-white/90 flex-1">{trait.value}</div>
                  </div>
                {/each}
              </div>
            </div>
          </div>
        </article>
      {/each}
    </div>
  </div>
</section>

<!-- About the Author section - commented out
<section class="relative w-full overflow-hidden bg-[#d7e6ff] flex items-center justify-center" style="min-height: calc(100vh - 200px);">
  <div class="relative z-10 mx-auto max-w-6xl px-6 py-20 sm:py-24 w-full">
    <div
      class="grid items-center gap-10 lg:grid-cols-[1fr_minmax(220px,320px)] lg:gap-16"
    >
      <div>
        <p class="text-2xl font-['Great_Vibes',_cursive] text-[#d2234c]">
          About the Author
        </p>
        <h2
          class="mt-2 text-3xl font-semibold tracking-tight text-[#1b1b1d] sm:text-4xl"
        >
          Alice Foronda
        </h2>
        <p class="mt-5 text-base leading-relaxed text-gray-700">
          Based in a cozy studio overflowing with picture books, Alice writes
          stories that spark conversations around courage, empathy, and
          belonging. When she is not drafting whimsical adventures, you can find
          her exploring forest trails, crafting, or sharing read-aloud moments
          with her family.
        </p>
        <p class="mt-4 text-base leading-relaxed text-gray-700">
          Her debut book, "Crosby's Cosmic Adventure," grew from a desire to
          help kids feel brave enough to shine exactly as they are. She believes
          that storytelling is a bridge between hearts.
        </p>
        <a
          class="mt-8 inline-flex items-center rounded-full bg-[#d2234c] px-6 py-3 text-sm font-semibold uppercase tracking-[0.2em] text-white shadow-lg transition hover:bg-[#d2234c]-dark"
          href={resolve("/about")}
        >
          Learn More
        </a>
      </div>
      <div class="relative mx-auto flex max-w-sm items-center justify-center">
        <div
          class="relative h-64 w-64 overflow-hidden rounded-full border-[12px] border-[#ffd28d] shadow-xl"
        >
          <img
            alt="Author portrait"
            src={authorImage}
            class="h-full w-full object-cover"
          />
        </div>
      </div>
    </div>
  </div>
</section>
-->

<style>
  .green-blob-mask {
    position: relative;
    isolation: isolate;
    width: 100%;
    display: grid;
  }

  .green-blob-bg {
    grid-area: 1 / 1;
    width: 100%;
    height: 100%;
    min-height: 100%;
    display: block;
    object-fit: cover;
    object-position: center;
    z-index: 0;
  }

  .green-blob-content {
    grid-area: 1 / 1;
    position: relative;
    z-index: 1;
    width: 100%;
    display: grid;
    grid-template-columns: 1fr;
    align-items: center;
    justify-items: center;
    text-align: center;
    gap: clamp(10px, 2.2vw, 18px);
    padding: clamp(8px, 2.2vw, 16px) 0 0;
  }

  .green-blob-col {
    width: 100%;
    min-width: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }


  .green-blob-col-right {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .green-blob-col-right img {
    width: min(100%, 26.25rem);
    margin: 0 auto;
  }


  .welcome-content {
    will-change: opacity, transform, max-height;
    transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    overflow: hidden;
  }

  .hero-popup-image {
    width: min(100%, 40rem);
    height: auto;
    display: block;
    margin: 0 auto;
    border-radius: 1.25rem;
  }

  .mailerlite-modal-backdrop {
    position: fixed;
    inset: 0;
    z-index: 12000;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 1rem;
    background: rgba(26, 18, 35, 0.62);
    backdrop-filter: blur(6px);
  }

  .mailerlite-modal-shell {
    position: relative;
    width: min(100%, 48rem);
    border-radius: 1.5rem;
    overflow: hidden;
    background: #8fc279;
    box-shadow: 0 28px 60px rgba(18, 10, 24, 0.28);
  }

  .mailerlite-modal-close {
    position: absolute;
    top: 0.65rem;
    right: 0.65rem;
    z-index: 4;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 2.5rem;
    height: 2.5rem;
    border: 0;
    border-radius: 999px;
    background: rgba(20, 18, 27, 0.76);
    color: #fff;
    padding: 0;
    font-size: 1.6rem;
    font-weight: 700;
    line-height: 1;
  }

  .mailerlite-modal-frame-wrap {
    position: relative;
    overflow: hidden;
    border-radius: inherit;
  }

  .mailerlite-modal-frame {
    display: block;
    width: 100%;
    height: min(85vh, 43rem);
    border: 0;
    background: #8fc279;
  }

  .mailerlite-modal-mask {
    position: absolute;
    pointer-events: none;
    z-index: 3;
  }

  .mailerlite-modal-mask-top {
    top: 0;
    right: 0;
    width: 4.25rem;
    height: 4.25rem;
    background: #8fc279;
    border-bottom-left-radius: 1.25rem;
  }


  @media (min-width: 768px) {
    .green-blob-content {
      grid-template-columns: 1fr 1fr;
      gap: clamp(8px, 1vw, 16px);
      padding: clamp(12px, 3vw, 36px);
    }

    .green-blob-col-right img {
      width: 85%;
    }
  }

  @media (min-width: 1024px) {
    .green-blob-content {
      grid-template-columns: minmax(0, 1.2fr) minmax(0, 0.8fr);
    }
  }

  @media (min-width: 1280px) {
    .green-blob-mask {
      width: min(100%, 68.75rem);
      margin-inline: auto;
    }
  }

  @media (max-width: 640px) {
    .mailerlite-modal-backdrop {
      padding: 0.75rem;
    }

    .mailerlite-modal-shell {
      border-radius: 1.1rem;
    }

    .mailerlite-modal-close {
      width: 2.25rem;
      height: 2.25rem;
      font-size: 1.45rem;
    }

    .mailerlite-modal-frame {
      height: min(88vh, 40rem);
    }

    .mailerlite-modal-mask-top {
      width: 3.75rem;
      height: 3.75rem;
    }
  }

  .home-falling-star {
    position: absolute;
    top: -20%;
    font-size: var(--size, 1.25rem);
    animation: home-fall linear infinite;
    opacity: 0.7;
    width: var(--size, 1.25rem);
    height: var(--size, 1.25rem);
  }

  .home-falling-star svg {
    width: 100%;
    height: 100%;
    display: block;
  }

  @keyframes home-fall {
    0% {
      top: -20%;
      transform: rotate(var(--rotate, 0deg));
      opacity: 0;
    }
    10% {
      opacity: 1;
    }
    100% {
      top: 105%;
      transform: rotate(calc(var(--rotate, 0deg) + 60deg));
      opacity: 0;
    }
  }

  /* Scroll-triggered animations - slide in from left */
  .scroll-reveal {
    opacity: 0;
    transform: translateX(-5rem);
    transition: opacity 800ms cubic-bezier(0.165, 0.84, 0.44, 1),
                transform 800ms cubic-bezier(0.165, 0.84, 0.44, 1);
  }

  .scroll-reveal:global(.animate-in) {
    opacity: 1;
    transform: translateX(0);
  }

  /* Character section slides from right */
  .characters-section.scroll-reveal {
    transform: translateX(5rem);
  }

  .characters-section.scroll-reveal:global(.animate-in) {
    transform: translateX(0);
  }

  @media (prefers-reduced-motion: reduce) {
    .scroll-reveal {
      opacity: 1;
      transform: none;
      transition: none;
    }
  }
</style>
