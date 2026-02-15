<script>
  import { resolve } from "$app/paths";
  import { onMount } from "svelte";

  const bookImage = resolve("/CrosbyCover.PNG");
  const authorImage = resolve("/alice_logopink.jpg");
  const headerImage = resolve("/alHeader_opt.jpg");
  const characterStars = [
    {
      left: "6%",
      size: "26px",
      duration: "4s",
      delay: "0.3s",
      rotate: "-8deg",
    },
    {
      left: "14%",
      size: "34px",
      duration: "4.5s",
      delay: "0.6s",
      rotate: "12deg",
    },
    {
      left: "22%",
      size: "28px",
      duration: "3.8s",
      delay: "0.1s",
      rotate: "-6deg",
    },
    {
      left: "30%",
      size: "42px",
      duration: "4.8s",
      delay: "0.9s",
      rotate: "10deg",
    },
    {
      left: "38%",
      size: "30px",
      duration: "3.6s",
      delay: "0.4s",
      rotate: "-12deg",
    },
    {
      left: "46%",
      size: "50px",
      duration: "5s",
      delay: "0.2s",
      rotate: "8deg",
    },
    {
      left: "54%",
      size: "32px",
      duration: "3.9s",
      delay: "1s",
      rotate: "-10deg",
    },
    {
      left: "62%",
      size: "36px",
      duration: "4.2s",
      delay: "0.5s",
      rotate: "6deg",
    },
    {
      left: "70%",
      size: "44px",
      duration: "4.6s",
      delay: "0.7s",
      rotate: "-9deg",
    },
    {
      left: "78%",
      size: "28px",
      duration: "3.5s",
      delay: "1.2s",
      rotate: "11deg",
    },
    {
      left: "86%",
      size: "38px",
      duration: "4.3s",
      delay: "0.8s",
      rotate: "-7deg",
    },
    {
      left: "94%",
      size: "30px",
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
      imageWebp: resolve("/Crosby.webp"),
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

  let activeCharacter = $state(0);

  // Track scroll state for welcome and logo visibility
  let showWelcome = $state(true);

  const prevCharacter = () => {
    activeCharacter =
      (activeCharacter - 1 + characters.length) % characters.length;
  };

  const nextCharacter = () => {
    activeCharacter = (activeCharacter + 1) % characters.length;
  };

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
          rootMargin: '0px 0px -100px 0px',
        }
      );

      const elements = document.querySelectorAll('.scroll-reveal');
      elements.forEach((el) => {
        observer.observe(el);
      });
    }, 150);
  });

</script>

<section
  class="relative w-full overflow-hidden"
  style={`background-image: url('${headerImage}'); background-size: cover; background-position: center; background-repeat: no-repeat; min-height: 100vh;`}
>
  <picture style="display: none;">
    <source srcset={resolve("/alHeader.webp")} type="image/webp" />
  </picture>

  <div class="relative z-10 mx-auto max-w-6xl px-6 transition-all duration-700 ease-out" style={showWelcome ? 'padding-top: 5rem; padding-bottom: 8rem;' : 'padding-top: 2rem; padding-bottom: 3rem;'}>
    <div class="flex flex-col lg:flex-row items-center justify-center gap-8 lg:gap-12 transition-all duration-700 ease-out">
      <!-- Logo -->
      <div class={`logo-container relative inline-flex items-center justify-center shrink-0 transition-all duration-700 ease-out ${showWelcome ? '' : 'logo-shrunk'}`}>
        <picture>
          <source srcset={resolve("/alice_logopink.webp")} type="image/webp" />
          <img
            src={resolve("/alice_logopink.jpg")}
            alt="Alice Foronda smiling, author of Crosby's Cosmic Adventure"
            class="h-40 w-40 rounded-full shadow-lg sm:h-48 sm:w-48 lg:h-56 lg:w-56 transition-shadow duration-700 ease-out"
            loading="lazy"
            style={showWelcome ? 'box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);' : 'box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);'}
          />
        </picture>
        <svg
          class="pointer-events-none absolute inset-[-15%] h-[130%] w-[130%] text-[#d2234c] transition-opacity duration-700 ease-out"
          viewBox="0 0 200 200"
          aria-hidden="true"
          style={showWelcome ? 'opacity: 1;' : 'opacity: 0.6;'}
        >
          <defs>
            <path
              id="logoCirclePath"
              d="M100,100 m -86,0 a 86,86 0 1,1 172,0 a 86,86 0 1,1 -172,0"
            />
          </defs>
          <text
            fill="currentColor"
            font-size="clamp(10px, 2.2vw, 14px)"
            font-weight="700"
            letter-spacing="clamp(1.5px, 0.6vw, 3.5px)"
            font-family="Poppins, 'Helvetica Neue', Arial, sans-serif"
          >
            <textPath
              href="#logoCirclePath"
              startOffset="50%"
              text-anchor="middle"
              textLength="540"
              lengthAdjust="spacingAndGlyphs"
            >
              • SNACK GIVER • CHILDREN'S BOOK AUTHOR • LOVER • MOM • KIMCHI
              STEALER
            </textPath>
          </text>
        </svg>
      </div>

      <!-- Welcome Section -->
      <div
        class="welcome-content text-center lg:text-left max-w-xl transition-all duration-700 ease-out origin-left rounded-3xl p-8 border border-white/10"
        style={showWelcome
          ? 'opacity: 1; transform: scale(1) translateX(0); max-height: 1000px; pointer-events: auto; background: rgba(255, 255, 255, 0.12); backdrop-filter: blur(20px); -webkit-backdrop-filter: blur(20px);'
          : 'opacity: 0; transform: scale(0.9) translateX(-20px); max-height: 0; pointer-events: none; overflow: hidden; background: rgba(255, 255, 255, 0); backdrop-filter: blur(0px); -webkit-backdrop-filter: blur(0px);'}
      >
        <p class="text-5xl sm:text-6xl font-['Great_Vibes',cursive] text-[#d2234c] mb-2">
          Welcome
        </p>
        <h1 class="text-3xl sm:text-4xl font-semibold tracking-tight text-gray-900 mb-4">
          Stories that sparkle with courage and heart.
        </h1>
        <p class="text-base sm:text-lg leading-relaxed text-gray-800 mb-6">
          Step into a world of imagination where every page whispers
          encouragement and every character finds their voice. Alice Foronda
          crafts magical adventures that help young readers feel seen, brave,
          and brilliantly themselves.
        </p>
        <div class="flex flex-wrap gap-3 justify-center lg:justify-start">
          <a
            class="inline-flex items-center rounded-full bg-[#d2234c] px-5 py-2.5 text-sm font-semibold uppercase tracking-[0.2em] text-white shadow-lg hover:bg-[#b01d3c] hover:shadow-2xl"
            href={resolve("/book")}
            style="transition: all 800ms cubic-bezier(0.165, 0.84, 0.44, 1);"
            onmouseenter={(e) => { e.currentTarget.style.transform = 'translateY(-4px) scale(1.05)'; }}
            onmouseleave={(e) => { e.currentTarget.style.transform = 'translateY(0) scale(1)'; }}
          >
            Learn More
          </a>
          <a
            class="inline-flex items-center rounded-full border-2 border-[#d2234c] bg-white/5 px-5 py-2.5 text-sm font-semibold uppercase tracking-[0.2em] text-[#d2234c] hover:bg-[#d2234c] hover:text-white backdrop-blur-sm shadow-md hover:shadow-xl"
            href={resolve("/quiz")}
            style="transition: all 800ms cubic-bezier(0.165, 0.84, 0.44, 1);"
            onmouseenter={(e) => { e.currentTarget.style.transform = 'translateY(-4px) scale(1.05)'; }}
            onmouseleave={(e) => { e.currentTarget.style.transform = 'translateY(0) scale(1)'; }}
          >
            Take the Quiz
          </a>
        </div>
      </div>
    </div>
  </div>
</section>

<section
  class="relative w-full overflow-hidden bg-pink-100 flex items-center justify-center scroll-reveal"
  style="min-height: 100vh;"
>
  <div class="relative z-10 mx-auto max-w-6xl px-6 py-20 sm:py-24 w-full">
    <div class="grid items-center gap-10 lg:grid-cols-[320px_1fr] lg:gap-16">
      <div
        class="relative mx-auto max-w-sm rounded-3xl bg-[dodgerblue] p-3 shadow-xl hover:shadow-2xl"
        style="transition: all 1000ms cubic-bezier(0.165, 0.84, 0.44, 1);"
        onmouseenter={(e) => { e.currentTarget.style.transform = 'translateY(-8px) scale(1.02)'; }}
        onmouseleave={(e) => { e.currentTarget.style.transform = 'translateY(0) scale(1)'; }}
      >
        <img
          alt="Crosby's Cosmic Adventure book cover featuring a crocodile in space"
          src={bookImage}
          class="h-full w-full rounded-2xl object-cover"
          loading="lazy"
        />
      </div>
      <div>
        <p class="text-2xl font-['Great_Vibes',_cursive] text-[#d2234c]">
          Debut Book!
        </p>
        <h2
          class="mt-2 text-3xl font-semibold tracking-tight text-[#1b1b1d] sm:text-4xl"
        >
          Crosby's Cosmic Adventure
        </h2>
        <p class="mt-5 text-base leading-relaxed text-gray-700">
          Crosby the crocodile feels a little too different—but when the
          moonlight sparkles, he learns that bravery is about trying, not about
          being perfect. With joyful illustrations and a heartfelt message, this
          story reminds young dreamers that every wobble can become a wonder.
        </p>
        <a
          class="mt-8 inline-flex items-center rounded-full bg-[#d2234c] px-6 py-3 text-sm font-semibold uppercase tracking-[0.2em] text-white shadow-lg hover:bg-[#b01d3c] hover:shadow-2xl"
          href={resolve("/book")}
          style="transition: all 800ms cubic-bezier(0.165, 0.84, 0.44, 1);"
          onmouseenter={(e) => { e.currentTarget.style.transform = 'translateY(-4px) scale(1.05)'; }}
          onmouseleave={(e) => { e.currentTarget.style.transform = 'translateY(0) scale(1)'; }}
        >
          Learn More
        </a>
      </div>
    </div>
  </div>
</section>

<section
  class="characters-section relative w-full overflow-hidden bg-blue-900 text-white scroll-reveal"
  style="min-height: 100vh;"
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
  <div class="relative z-10 mx-auto flex h-full max-w-6xl flex-col px-6 py-24">
    <div class="mb-10 text-center">
      <p
        class="text-3xl font-['Great_Vibes',_cursive] text-[#ffb4c6] sm:text-4xl"
      >
        Meet the Characters
      </p>
      <h2
        class="mt-2 text-3xl font-semibold tracking-tight sm:text-4xl whitespace-nowrap text-center mx-auto"
      >
        {characters[activeCharacter].name}
      </h2>
      <p class="mt-2 text-base text-[#b8d4ff]">
        {characters[activeCharacter].role}
      </p>
    </div>
    <div
      class="grid gap-10 lg:grid-cols-[minmax(320px,420px)_1fr] lg:gap-16 flex-1"
      aria-live="polite"
    >
      <div
        class="character-frame overflow-hidden rounded-full border-8 bg-white/10 p-2 shadow-[0_15px_40px_rgba(0,0,0,0.25)] mx-auto flex items-center justify-center"
        style={`border-color:${characters[activeCharacter].accent}`}
      >
        <picture>
          <source srcset={characters[activeCharacter].imageWebp} type="image/webp" />
          <img
            alt={`Illustration of ${characters[activeCharacter].name}`}
            src={characters[activeCharacter].image}
            class={`rounded-full object-contain character-img ${activeCharacter === 0 ? 'character-img-crosby' : ''} ${activeCharacter === 2 ? 'character-img-tank' : ''}`}
            loading="lazy"
          />
        </picture>
      </div>

      <style>
        .logo-container {
          will-change: transform, opacity;
          transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
          transform: scale(1) translateX(0);
          opacity: 1;
        }

        .logo-shrunk {
          transform: scale(0) translateX(0);
          opacity: 0;
        }

        @media (min-width: 1024px) {
          .logo-shrunk {
            transform: scale(0.75) translateX(-50%);
            opacity: 1;
          }
        }

        .welcome-content {
          will-change: opacity, transform, max-height;
          transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
          box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
        }

        @media (prefers-reduced-motion: reduce) {
          .logo-container,
          .welcome-content {
            transition: none !important;
          }
        }

        @media (max-width: 1023px) {
          .welcome-content {
            padding: 1.5rem !important;
          }

          .logo-container {
            /* On mobile/tablet, don't translate as much */
          }
        }

        @media (max-width: 640px) {
          .welcome-content {
            padding: 1.25rem !important;
          }
        }

        .character-frame {
          width: 345px;
          height: 345px;
        }

        .character-img {
          width: 345px;
          height: 345px;
        }

        .character-img-crosby {
          width: 320px;
          height: 320px;
        }

        .character-img-tank {
          width: 310px;
          height: 310px;
        }

        @media (min-width: 640px) {
          .character-frame {
            width: 385px;
            height: 385px;
          }

          .character-img {
            width: 385px;
            height: 385px;
          }

          .character-img-crosby {
            width: 360px;
            height: 360px;
          }

          .character-img-tank {
            width: 345px;
            height: 345px;
          }
        }

        @media (min-width: 1024px) {
          .character-frame {
            width: 460px;
            height: 460px;
          }

          .character-img {
            width: 460px;
            height: 460px;
          }

          .character-img-crosby {
            width: 430px;
            height: 430px;
          }

          .character-img-tank {
            width: 410px;
            height: 410px;
          }
        }
      </style>
      <div class="space-y-6">
        <p
          class="text-lg leading-relaxed text-[#ffe8f0] transition-all duration-300"
        >
          {characters[activeCharacter].summary}
        </p>
        <div class="grid gap-4 sm:grid-cols-2">
          {#each characters[activeCharacter].traits as trait}
            <div class="rounded-2xl bg-white/10 p-4 text-sm backdrop-blur-sm">
              <div
                class="font-semibold uppercase tracking-[0.2em]"
                style={`color:${characters[activeCharacter].accent}`}
              >
                {trait.label}
              </div>
              <div class="mt-2 text-white/90">{trait.value}</div>
            </div>
          {/each}
        </div>
        <div
          class="flex flex-col items-center gap-3 sm:flex-row sm:flex-wrap sm:items-center"
        >
          <div class="flex items-center gap-2">
            <button
              class="inline-flex items-center justify-center rounded-full border border-white/60 px-4 py-2 text-[13px] font-semibold uppercase tracking-[0.2em] text-white hover:bg-white/10 hover:shadow-lg sm:px-5 sm:py-3 sm:text-sm whitespace-nowrap"
              type="button"
              onclick={prevCharacter}
              aria-label="Previous character"
              style="transition: all 600ms cubic-bezier(0.165, 0.84, 0.44, 1);"
              onmouseenter={(e) => { e.currentTarget.style.transform = 'translateY(-3px) scale(1.03)'; }}
              onmouseleave={(e) => { e.currentTarget.style.transform = 'translateY(0) scale(1)'; }}
            >
              ← <span class="hidden sm:inline ml-1">Meet Previous</span>
            </button>
            <button
              class="inline-flex items-center justify-center rounded-full bg-white px-4 py-2 text-[13px] font-semibold uppercase tracking-[0.2em] text-[#052238] shadow-lg hover:shadow-2xl sm:px-5 sm:py-3 sm:text-sm whitespace-nowrap"
              type="button"
              onclick={nextCharacter}
              aria-label="Next character"
              style="transition: all 600ms cubic-bezier(0.165, 0.84, 0.44, 1);"
              onmouseenter={(e) => { e.currentTarget.style.transform = 'translateY(-4px) scale(1.05)'; }}
              onmouseleave={(e) => { e.currentTarget.style.transform = 'translateY(0) scale(1)'; }}
            >
              <span class="hidden sm:inline">Next Up</span>
              <span class="sm:ml-1">→</span>
            </button>
          </div>
          <div
            class="flex flex-wrap justify-center gap-2 sm:justify-start"
            role="group"
            aria-label="Choose a character to view"
          >
            {#each characters as character, index}
              <button
                class={`rounded-full px-3 py-2 text-xs font-semibold uppercase tracking-[0.2em] transition ${
                  index === activeCharacter
                    ? "bg-white text-[#052238] shadow-lg"
                    : "bg-white/10 text-white/90 hover:bg-white/20"
                }`}
                type="button"
                onclick={() => (activeCharacter = index)}
                aria-pressed={index === activeCharacter}
                aria-label={`View ${character.name}`}
              >
                {character.name.split(" ")[0]}
              </button>
            {/each}
          </div>
        </div>
      </div>
    </div>
  </div>
</section>

<!-- About the Author section - commented out
<section class="relative w-full overflow-hidden bg-pink-100 flex items-center justify-center" style="min-height: calc(100vh - 200px);">
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
  .home-falling-star {
    position: absolute;
    top: -20%;
    font-size: var(--size, 20px);
    animation: home-fall linear infinite;
    opacity: 0.7;
    width: var(--size, 20px);
    height: var(--size, 20px);
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
    transform: translateX(-80px);
    transition: opacity 800ms cubic-bezier(0.165, 0.84, 0.44, 1),
                transform 800ms cubic-bezier(0.165, 0.84, 0.44, 1);
  }

  .scroll-reveal.animate-in {
    opacity: 1;
    transform: translateX(0);
  }

  .scroll-reveal.animate-in .scroll-reveal-child {
    opacity: 1;
    transform: translateX(0);
  }

  .scroll-reveal-child {
    opacity: 0;
    transform: translateX(-40px);
    transition: opacity 600ms cubic-bezier(0.165, 0.84, 0.44, 1),
                transform 600ms cubic-bezier(0.165, 0.84, 0.44, 1);
  }

  /* Staggered delays for child elements */
  .scroll-reveal-child:nth-child(1) {
    transition-delay: 150ms;
  }

  .scroll-reveal-child:nth-child(2) {
    transition-delay: 250ms;
  }

  .scroll-reveal-child:nth-child(3) {
    transition-delay: 350ms;
  }

  /* Character section slides from right */
  .characters-section.scroll-reveal {
    transform: translateX(80px);
  }

  .characters-section.scroll-reveal.animate-in {
    transform: translateX(0);
  }

  @media (prefers-reduced-motion: reduce) {
    .scroll-reveal,
    .scroll-reveal-child {
      opacity: 1;
      transform: none;
      transition: none;
    }
  }
</style>
