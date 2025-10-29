<script>
  const heroImage =
    "https://images.unsplash.com/photo-1519681393784-d120267933ba?auto=format&fit=crop&w=1400&q=80";
  const bookImage =
    "https://images.unsplash.com/photo-1529651737248-dad5e287768e?auto=format&fit=crop&w=800&q=80";
  const authorImage =
    "https://images.unsplash.com/photo-1517841905240-472988babdf9?auto=format&fit=crop&w=600&q=80";
  const characterImage =
    "https://images.unsplash.com/photo-1601758003122-58cbb7070ac6?auto=format&fit=crop&w=800&q=80";

  const characterTraits = [
    { label: "Personality", value: "Kind, thoughtful, supportive" },
    { label: "Likes", value: "Stories, cake, tight spaces" },
    { label: "Dislikes", value: "Feeling left out, tight spaces" },
    { label: "Strengths", value: "Perseverance" },
  ];
</script>

<section class="relative w-full overflow-hidden bg-white">
  <div class="relative z-10 mx-auto max-w-6xl px-6 pb-20 pt-24 sm:pb-28">
    <div class="grid items-stretch gap-10 lg:grid-cols-2 lg:gap-12">
      <div
        class="relative h-80 overflow-hidden rounded-3xl sm:h-[26rem] lg:h-full lg:rounded-[3rem]"
      >
        <img
          alt="Author with child"
          src={heroImage}
          class="h-full w-full object-cover"
        />
      </div>
      <div class="relative flex flex-col justify-center">
        <img src="/alice_logopink.jpg" alt="AJK" class="rounded-full" />
        <p class="mt-4 text-3xl font-['Great_Vibes',_cursive] text-[#d2234c]">
          Welcome
        </p>
        <h1
          class="mt-2 text-4xl font-semibold tracking-tight text-[#1b1b1d] sm:text-5xl"
        >
          Stories that sparkle with courage and heart.
        </h1>
        <p class="mt-6 max-w-xl text-lg leading-relaxed text-gray-700">
          Step into a world of imagination where every page whispers
          encouragement and every character finds their voice. Alice Foronda
          crafts magical adventures that help young readers feel seen, brave,
          and brilliantly themselves.
        </p>
        <div class="mt-8 flex flex-wrap gap-4">
          <a
            class="inline-flex items-center rounded-full bg-[#d2234c] px-6 py-3 text-sm font-semibold uppercase tracking-[0.2em] text-white shadow-lg transition hover:bg-[#b31d41]"
            href="/book"
          >
            Learn More
          </a>
          <a
            class="inline-flex items-center rounded-full border border-[#d2234c] px-6 py-3 text-sm font-semibold uppercase tracking-[0.2em] text-[#d2234c] transition hover:bg-[#ffe0ec]"
            href="/quiz"
          >
            Take the Quiz
          </a>
        </div>
      </div>
    </div>
  </div>
  <div class="w-full h-32 bg-pink-100">
    <svg
      viewBox="0 0 1000 200"
      class="h-32 w-full text-white"
      preserveAspectRatio="none"
    >
      <path
        d="M0 0v4c154 96 190 17.9 300 17.9 92 0 92 78.1 200 78.1s108-78.2 200-78.2c110 0 146 78.2 300-17.8V0H0Z"
        class="fill-current"
      />
    </svg>
  </div>
</section>

<section class="relative w-full overflow-hidden bg-pink-100">
  <div class="relative z-10 mx-auto max-w-6xl px-6 py-20 sm:py-24">
    <div class="grid items-center gap-10 lg:grid-cols-[320px_1fr] lg:gap-16">
      <div class="relative mx-auto max-w-sm rounded-3xl bg-white p-4 shadow-xl">
        <img
          alt="Book cover"
          src={bookImage}
          class="h-full w-full rounded-2xl object-cover"
        />
      </div>
      <div>
        <p class="text-2xl font-['Great_Vibes',_cursive] text-[#d2234c]">
          Debut Book!
        </p>
        <h2 class="mt-2 text-3xl font-semibold tracking-tight text-[#1b1b1d] sm:text-4xl">
          Crosby's Cosmic Adventure
        </h2>
        <p class="mt-5 text-base leading-relaxed text-gray-700">
          Crosby the crocodile feels a little too different—but when the
          moonlight sparkles, he learns that bravery is about trying, not about
          being perfect. With joyful illustrations and a heartfelt message, this
          story reminds young dreamers that every wobble can become a wonder.
        </p>
        <a
          class="mt-8 inline-flex items-center rounded-full bg-[#d2234c] px-6 py-3 text-sm font-semibold uppercase tracking-[0.2em] text-white shadow-lg transition hover:bg-[#b31d41]"
          href="/book"
        >
          Learn More
        </a>
      </div>
    </div>
  </div>
</section>

<section class="relative w-full overflow-hidden bg-blue-900 text-white">
  <div class="relative z-10 mx-auto max-w-6xl px-6 py-24">
    <div class="mb-10 text-center">
      <p class="text-2xl font-['Great_Vibes',_cursive] text-[#ffb4c6]">
        Meet the Characters
      </p>
      <h2 class="mt-2 text-3xl font-semibold tracking-tight sm:text-4xl">
        Crosby the Crocodile
      </h2>
    </div>
    <div class="grid gap-10 lg:grid-cols-[minmax(260px,320px)_1fr] lg:gap-16">
      <div
        class="overflow-hidden rounded-full border-8 border-[#ffd28d] bg-white/10 p-6"
      >
        <img
          alt="Illustration of character"
          src={characterImage}
          class="h-full w-full rounded-full object-cover"
        />
      </div>
      <div class="space-y-6">
        <p class="text-lg leading-relaxed text-[#ffe8f0]">
          Crosby is gentle, curious, and a little shy. With friends who cheer
          him on, he learns that the very traits he once hid make him a hero in
          the hearts of others. Courage, kindness, and a dash of starlight guide
          his adventures.
        </p>
        <div class="grid gap-4 sm:grid-cols-2">
          {#each characterTraits as trait}
            <div class="rounded-2xl bg-white/10 p-4 text-sm">
              <div
                class="font-semibold uppercase tracking-[0.2em] text-[#ffd28d]"
              >
                {trait.label}
              </div>
              <div class="mt-2 text-white/90">{trait.value}</div>
            </div>
          {/each}
        </div>
        <button
          class="inline-flex items-center rounded-full border border-white/60 px-6 py-3 text-sm font-semibold uppercase tracking-[0.2em] text-white transition hover:bg-white/10"
          type="button"
        >
          Explore More Friends
        </button>
      </div>
    </div>
  </div>
  <div class="w-full h-32 bg-pink-100">
    <svg
      viewBox="0 0 1000 200"
      class="h-32 w-full text-blue-900"
      preserveAspectRatio="none"
    >
      <path
        d="M0 0v4c154 96 190 17.9 300 17.9 92 0 92 78.1 200 78.1s108-78.2 200-78.2c110 0 146 78.2 300-17.8V0H0Z"
        class="fill-current"
      />
    </svg>
  </div>
</section>

<section class="relative w-full overflow-hidden bg-pink-100">
  <div class="relative z-10 mx-auto max-w-6xl px-6 py-20 sm:py-24">
    <div
      class="grid items-center gap-10 lg:grid-cols-[1fr_minmax(220px,320px)] lg:gap-16"
    >
      <div>
        <p class="text-2xl font-['Great_Vibes',_cursive] text-[#d2234c]">
          About the Author
        </p>
        <h2 class="mt-2 text-3xl font-semibold tracking-tight text-[#1b1b1d] sm:text-4xl">
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
          class="mt-8 inline-flex items-center rounded-full bg-[#d2234c] px-6 py-3 text-sm font-semibold uppercase tracking-[0.2em] text-white shadow-lg transition hover:bg-[#b31d41]"
          href="/about"
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
