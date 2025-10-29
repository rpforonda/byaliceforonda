<script>
  // Svelte 5 runes
  import { resolve } from '$app/paths';

  // QUESTIONS
  const questions = [
    {
      q: "You’ve just landed on a planet made entirely of marshmallows. What’s your first move?",
      choices: [
        "Test how the gravity affects your bounce potential.",
        "Build a mini rover out of sticky bits to explore!",
        "Rally everyone for a moon-mallow picnic before takeoff.",
        "Sit quietly for a second — it’s actually kind of beautiful here."
      ]
    },
    {
      q: "What’s your go-to travel snack for a long space voyage?",
      choices: [
        "Meteorite muffins — chewy, mysterious, delicious.",
        "Cosmic trail mix — efficient and dependable.",
        "Something homemade and shared with friends.",
        "Starfruit chips — crunchy and a little unpredictable."
      ]
    },
    {
      q: "A meteor shower is heading your way! You…",
      choices: [
        "Grab your notebook — time for data collection!",
        "Calmly lead your crew to safety while keeping morale high.",
        "Hold your breath and trust your instincts — you’ve got this.",
        "Try to catch one. For science… or maybe for fun."
      ]
    },
    {
      q: "What kind of spaceship décor speaks to your soul?",
      choices: [
        "Twinkling lights and cozy corners for reading.",
        "Organized chaos — charts, mission maps, and snacks everywhere.",
        "Tools and trinkets lining every wall.",
        "Soft blankets and big windows to watch the stars drift by."
      ]
    },
    {
      q: "Which cosmic pet would you adopt?",
      choices: [
        "A floating jellycat that hums lullabies.",
        "A tiny asteroid hamster who loves adventure.",
        "A sentient toolbox that follows you around.",
        "A shy comet-fish that glows when it’s happy."
      ]
    },
    {
      q: "What’s your motto for space life?",
      choices: [
        "“Dream it, build it, launch it.”",
        "“Together, we can cross any galaxy.”",
        "“Courage starts small but shines bright.”",
        "“Curiosity never crashes.”"
      ]
    },
    {
      q: "You’re given an alien artifact that glows when you touch it. You…",
      choices: [
        "Study it under a magnifier — maybe it’s a power source!",
        "Share it with the crew so everyone can see.",
        "Feel a little nervous, then hold it tight — it feels special.",
        "Try to make it into a lamp. Obviously."
      ]
    },
    {
      q: "What’s your favorite kind of adventure?",
      choices: [
        "Quietly getting lost somewhere beautiful.",
        "Tackling a big, brave challenge with friends.",
        "Experimenting until something amazing happens.",
        "Organizing a mission where everyone has a role."
      ]
    }
  ];

  // SCORING (each question -> each choice -> weights)
  const scoring = [
    [
      { tank: 0, carla: 0, crosby: 2 },
      { tank: 1, carla: 1, crosby: 0 },
      { tank: 0, carla: 2, crosby: 1 },
      { tank: 1, carla: 0, crosby: 1 }
    ],
    [
      { tank: 1, carla: 0, crosby: 1 },
      { tank: 0, carla: 2, crosby: 0 },
      { tank: 0, carla: 1, crosby: 1 },
      { tank: 2, carla: 0, crosby: 0 }
    ],
    [
      { tank: 2, carla: 0, crosby: 0 },
      { tank: 0, carla: 2, crosby: 0 },
      { tank: 0, carla: 0, crosby: 2 },
      { tank: 1, carla: 1, crosby: 0 }
    ],
    [
      { tank: 1, carla: 0, crosby: 1 },
      { tank: 0, carla: 2, crosby: 0 },
      { tank: 2, carla: 0, crosby: 0 },
      { tank: 0, carla: 1, crosby: 1 }
    ],
    [
      { tank: 1, carla: 0, crosby: 1 },
      { tank: 0, carla: 2, crosby: 0 },
      { tank: 2, carla: 0, crosby: 0 },
      { tank: 0, carla: 0, crosby: 2 }
    ],
    [
      { tank: 2, carla: 0, crosby: 0 },
      { tank: 0, carla: 2, crosby: 0 },
      { tank: 0, carla: 0, crosby: 2 },
      { tank: 1, carla: 1, crosby: 0 }
    ],
    [
      { tank: 2, carla: 0, crosby: 0 },
      { tank: 0, carla: 2, crosby: 0 },
      { tank: 0, carla: 0, crosby: 2 },
      { tank: 1, carla: 1, crosby: 0 }
    ],
    [
      { tank: 1, carla: 0, crosby: 1 },
      { tank: 0, carla: 2, crosby: 0 },
      { tank: 2, carla: 0, crosby: 0 },
      { tank: 0, carla: 1, crosby: 1 }
    ]
  ];

  // RESULT DEFINITIONS
  const results = {
    tank: {
      title: "Tank the Turtle",
      avatar: "🐢",
      desc: "Inventive and endlessly curious — you tinker, wonder, and turn 'what if' into 'what works.' Your quiet focus sparks big discoveries."
    },
    carla: {
      title: "Carla the Capybara",
      avatar: "🪶",
      desc: "A fearless and kind leader. You keep everyone together, lift spirits, and steer the crew toward warm-hearted adventures."
    },
    crosby: {
      title: "Crosby the Crocodile",
      avatar: "💚",
      desc: "Gentle, thoughtful, and brave in your own way. You may doubt, but you shine when you step forward — and others see it too."
    },
    combo: {
      title: "Cosmic Combo",
      avatar: "✨",
      desc: "You mix Tank's curiosity, Carla's leadership, and Crosby's brave heart — a true all-star of the cosmos."
    }
  };

  // STATE
  let answers = $state(Array(questions.length).fill(null)); // each index is 0..3 or null
  let showCard = $state(false);
  let finalKey = $state(null); // 'tank' | 'carla' | 'crosby' | 'combo'
  let tally = $state({ tank: 0, carla: 0, crosby: 0 });

  // derived % progress
  const progressPct = $derived(() => {
    const answered = answers.filter((a) => a !== null).length;
    return Math.round((answered / questions.length) * 100);
  });

  function preventSubmit(event) {
    event.preventDefault();
  }

  function selectAnswer(qIndex, choiceIndex) {
    answers[qIndex] = choiceIndex;
  }

  function computeResult() {
    // check completeness
    if (answers.includes(null)) {
      alert(
        `You're almost there! Please answer all ${questions.length} questions so the stars can align ✨`
      );
      return;
    }

    // tally scores
    const t = { tank: 0, carla: 0, crosby: 0 };
    answers.forEach((choice, i) => {
      const add = scoring[i][choice];
      t.tank += add.tank;
      t.carla += add.carla;
      t.crosby += add.crosby;
    });

    // pick winner (or combo)
    const entries = Object.entries(t).sort((a, b) => b[1] - a[1]); // desc
    const topScore = entries[0][1];
    const tied = entries.filter((e) => e[1] === topScore).map((e) => e[0]);

    let key;
    if (tied.length === 1) {
      key = tied[0];
    } else {
      key = 'combo';
    }

    tally = t;
    finalKey = key;
    showCard = true;

    // smooth scroll to result
    queueMicrotask(() => {
      const el = document.getElementById('resultCard');
      if (el) el.scrollIntoView({ behavior: 'smooth', block: 'center' });
    });
  }

  function resetQuiz() {
    answers = Array(questions.length).fill(null);
    showCard = false;
    finalKey = null;
    tally = { tank: 0, carla: 0, crosby: 0 };
    window.scrollTo({ top: 0, behavior: 'smooth' });
  }

  async function shareResult() {
    if (!showCard || !finalKey) return;
    const card = results[finalKey] ?? results.combo;

    const shareText = `${
      finalKey === 'combo' ? "You're a Cosmic Combo!" : `You're ${card.title}!`
    } — ${card.desc}
Find out your Crosby's Cosmic Adventure character!`;

    // try Web Share API first
    if (navigator.share) {
      try {
        await navigator.share({ text: shareText });
        return;
      } catch {
        /* ignore if user cancels */
      }
    }

    // fallback: copy to clipboard
    try {
      await navigator.clipboard.writeText(shareText);
      alert('Result copied to clipboard — paste it anywhere to share!');
    } catch {
      alert('Could not copy to clipboard');
    }
  }

  function handleKey(e) {
    if (e.key !== 'Enter') return;

    e.preventDefault();

    // focus first unanswered
    const idx = answers.findIndex((a) => a === null);
    if (idx !== -1) {
      const first = document.querySelector(`input[name="q${idx}"]`);
      if (first) first.focus();
      return;
    }

    // all answered, compute
    computeResult();
  }
</script>

<svelte:window on:keydown={handleKey} />

  <!-- card container -->
  <div
    class="w-full max-w-[920px] rounded-xl border border-[rgba(255,255,255,0.03)]
           bg-blue-900
           shadow-[0_6px_30px_rgba(5,10,20,0.6),inset_0_1px_0_rgba(255,255,255,0.02)]
           p-6 sm:p-7"
  >
    <!-- header -->
    <header class="mb-5 flex items-start gap-4">
      <div
        class="flex h-[72px] w-[72px] -rotate-6 items-center justify-center rounded-xl
               bg-[linear-gradient(135deg,#8fd3ff,#ffd36b)]
               font-bold text-[#052238] shadow-[0_6px_18px_rgba(141,211,255,0.06)]"
        aria-hidden="true"
      >
        ★
      </div>

      <div>
        <h1
          id="quiz-title"
          class="m-0 text-[20px] font-semibold leading-snug tracking-[0.01em]"
        >
          Which <em>Crosby’s Cosmic Adventure</em> Character Are You?
        </h1>

        <p class="mt-[6px] text-sm leading-relaxed text-[#9fb6d6]">
          A playful, cosmic quiz — pick the answers that feel most you and
          discover your crew match.
        </p>

        <p class="mt-2 text-[13px] leading-relaxed text-[#9fb6d6]">
          8 short and silly questions • results are delightfully unpredictable
        </p>
      </div>
    </header>

    <!-- quiz body -->
    <form
      class="mt-4"
      onsubmit={preventSubmit}
      aria-live="polite"
    >
      {#each questions as qobj, qi}
        <fieldset
          class="mb-3 grid grid-cols-[1fr_auto] items-start gap-2 rounded-lg border border-[rgba(255,255,255,0.02)]
                 bg-[linear-gradient(180deg,rgba(255,255,255,0.04),transparent)]
                 p-4 text-[14px] text-[#dff3ff]"
          id={"q" + (qi + 1)}
        >
          <legend class="col-span-2 mb-1 p-0 text-[15px] font-semibold leading-snug text-[#e6f0ff]">
            {qi + 1}. {qobj.q}
          </legend>

          <div class="col-span-2 text-[13px] text-[#9fb6d6]">Pick one</div>

          <div class="col-span-2 mt-2 flex flex-wrap gap-2">
            {#each qobj.choices as choiceText, ci}
              <label
                class="flex cursor-pointer select-none items-start gap-2 rounded-md border border-[rgba(255,255,255,0.02)]
                       bg-[rgba(255,255,255,0.02)] px-3 py-2 text-left text-[14px] text-[#dff3ff]
                       transition-all duration-150 ease-out hover:-translate-y-[3px]
                       hover:shadow-[0_6px_18px_rgba(3,10,20,0.5)]"
              >
                <input
                  class="mt-[2px] h-[18px] w-[18px] shrink-0 accent-[#8fd3ff]"
                  type="radio"
                  name={"q" + qi}
                  value={ci}
                  aria-label={choiceText}
                  checked={answers[qi] === ci}
                  onchange={() => selectAnswer(qi, ci)}
                />

                <span class="leading-snug">{choiceText}</span>
              </label>
            {/each}
          </div>
        </fieldset>
      {/each}
    </form>

    <!-- controls -->
    <div class="mt-4 flex flex-col gap-3 sm:flex-row sm:items-center">
      <!-- progress bar -->
      <div
        class="h-3 flex-1 overflow-hidden rounded-full bg-[rgba(255,255,255,0.03)]"
        aria-hidden="true"
      >
        <div
          class="h-full rounded-full bg-[linear-gradient(90deg,#8fd3ff,#ffd36b)] transition-all duration-300 ease-out"
          style={`width:${progressPct}%`}
        ></div>
      </div>

      <div class="flex flex-wrap items-center gap-2">
        <button
          type="button"
          onclick={computeResult}
          class="rounded-lg border-0 bg-[linear-gradient(180deg,#8fd3ff,#4db7ff)] px-4 py-2 text-[14px] font-bold
                 text-[#052238] shadow-[0_6px_18px_rgba(77,183,255,0.12)]"
        >
          See My Character
        </button>

        <button
          type="button"
          onclick={resetQuiz}
          title="Reset quiz"
          class="rounded-lg border border-[rgba(255,255,255,0.04)] bg-transparent px-3 py-2 text-[14px] font-semibold text-[#9fb6d6]"
        >
          Reset
        </button>
      </div>
    </div>

    <!-- RESULT CARD -->
    {#if showCard && finalKey}
      <div
        id="resultCard"
        role="region"
        aria-live="polite"
        class="mt-5 rounded-xl border border-[rgba(255,255,255,0.03)]
               bg-[linear-gradient(180deg,rgba(255,255,255,0.02),transparent)]
               p-5 text-[#e6f0ff]"
      >
        <div
          class="flex flex-col items-start gap-4 sm:flex-row sm:items-start"
        >
          <!-- avatar -->
          <div
            class="flex h-[84px] w-[84px] items-center justify-center rounded-xl border border-[rgba(255,255,255,0.02)]
                   bg-[linear-gradient(135deg,rgba(255,255,255,0.03),transparent)]
                   text-[20px] font-extrabold text-[#ffd36b] sm:h-[110px] sm:w-[110px] sm:text-[26px]"
            aria-hidden="true"
          >
            {finalKey === 'combo' ? results.combo.avatar : results[finalKey].avatar}
          </div>

          <div class="text-left">
            <h2 class="m-0 text-[20px] font-semibold leading-snug text-[#e6f0ff]">
              {finalKey === 'combo'
                ? "You're a Cosmic Combo!"
                : `You're ${results[finalKey].title}!`}
            </h2>

            <div class="mt-2 text-[14px] leading-relaxed text-[#9fb6d6]">
              {finalKey === 'combo'
                ? results.combo.desc
                : results[finalKey].desc}
            </div>
          </div>
        </div>

        <!-- score stats -->
        <div class="mt-4 flex flex-wrap gap-2 text-[13px] text-[#9fb6d6]">
          <div class="rounded-md bg-[rgba(255,255,255,0.02)] px-3 py-2">
            Tank: {tally.tank}
          </div>
          <div class="rounded-md bg-[rgba(255,255,255,0.02)] px-3 py-2">
            Carla: {tally.carla}
          </div>
          <div class="rounded-md bg-[rgba(255,255,255,0.02)] px-3 py-2">
            Crosby: {tally.crosby}
          </div>
        </div>

        <!-- actions under result -->
        <div class="mt-4 flex flex-wrap gap-2">
          <button
            type="button"
            onclick={resetQuiz}
            class="rounded-lg border-0 bg-[linear-gradient(180deg,#8fd3ff,#4db7ff)] px-4 py-2 text-[14px] font-bold
                   text-[#052238] shadow-[0_6px_18px_rgba(77,183,255,0.12)]"
          >
            Take Again
          </button>

          <button
            type="button"
            onclick={shareResult}
            class="rounded-lg border border-[rgba(255,255,255,0.04)] bg-transparent px-3 py-2 text-[14px] font-semibold text-[#9fb6d6]"
          >
            Share
          </button>
        </div>
      </div>
    {/if}

    <!-- footer -->
    <footer
      class="mt-6 flex flex-col items-start justify-between gap-3 text-[13px] text-[#9fb6d6] sm:flex-row sm:items-center"
    >
      <div class="opacity-90">
        Made with stardust • Crosby’s Cosmic Adventure
      </div>
      <div class="text-[13px] leading-snug text-[#9fb6d6]">
        Interactive quiz • no tracking
      </div>
    </footer>
  </div>
