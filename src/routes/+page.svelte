<script>
  import { resolve } from "$app/paths";
  import { onMount } from "svelte";

  const crosbyFrameImage = resolve("/Crosbyframe.png");
  const greenBlobImage = resolve("/greenblob.png");
  const heroPopupImage = resolve("/cosmic.jpeg");
  const authorImage = resolve("/alice_logopink.jpg");
  const meetCharactersImage = resolve("/mtc.png");
  const joinCrosbyImage = resolve("/joincrosby.png");
  const creamBlobImage = resolve("/creamblob.png");
  const crosbyCoverHomeImage = resolve("/crosbycoverhome.png");
  const crosbyGreenImage = resolve("/crosbygreen.png");
  const carlaPinkImage = resolve("/carlapink.png");
  const tankYellowImage = resolve("/tankyellow.png");
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
    // Small delay to ensure layout is complete
    setTimeout(() => {
      const observer = new IntersectionObserver(
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
  });

</script>

<svelte:head>
  <!-- MailerLite Universal -->
  <script>
    (function(w,d,e,u,f,l,n){w[f]=w[f]||function(){(w[f].q=w[f].q||[])
    .push(arguments);},l=d.createElement(e),l.async=1,l.src=u,
    n=d.getElementsByTagName(e)[0],n.parentNode.insertBefore(l,n);})
    (window,document,'script','https://assets.mailerlite.com/js/universal.js','ml');
    ml('account', '2261437');
  </script>
  <!-- End MailerLite Universal -->
</svelte:head>

<section
  class="relative w-full overflow-hidden bg-[#d7e6ff]"
>
  <div class="page-shell relative z-10 transition-all duration-700 ease-out" style={showWelcome ? 'padding-top: 4rem; padding-bottom: 2.5rem;' : 'padding-top: 1.5rem; padding-bottom: 1.5rem;'}>
    <div class="flex flex-col lg:flex-row items-start justify-start gap-8 lg:gap-8 transition-all duration-700 ease-out">
      <div class="w-full max-w-[21rem] lg:max-w-[26rem] shrink-0">
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
                class="freckle-face-regular max-w-[34ch] text-center text-black text-[clamp(0.8rem,1.25vw,1.05rem)] leading-[1.35]"
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

  .green-blob-heading {
    margin: 0 0 0.75rem;
    font-size: clamp(1.45rem, 3.1vw, 2.6rem);
    line-height: 1.08;
    font-weight: 700;
    color: #2d471a;
  }

  .green-blob-copy {
    margin: 0;
    max-width: 34ch;
    font-size: clamp(0.95rem, 1.4vw, 1.2rem);
    line-height: 1.45;
    color: rgba(33, 53, 20, 0.88);
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

  .green-blob-panel {
    display: flex;
    align-items: center;
    justify-content: center;
    width: min(100%, 21.25rem);
    min-height: clamp(6.25rem, 16vw, 11.25rem);
    border: 2px dashed rgba(43, 71, 23, 0.5);
    border-radius: 1.125rem;
    background: rgba(255, 255, 255, 0.14);
    padding: 1rem;
    text-align: center;
    font-size: clamp(0.9rem, 1.2vw, 1rem);
    font-weight: 600;
    color: rgba(40, 63, 22, 0.82);
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

  .character-card-frame {
    width: min(100%, 13.75rem);
    aspect-ratio: 1 / 1;
  }

  .character-card-frame picture {
    display: block;
    width: 100%;
    height: 100%;
  }

  .character-card-image {
    width: 100%;
    height: 100%;
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
