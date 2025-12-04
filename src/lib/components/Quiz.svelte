<script>
  // Svelte 5 runes

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

  const slideThemes = [
    {
      name: "Tank",
      accent: "#8fd3ff",
      gradient: "linear-gradient(135deg,#061c35,#10375e)",
      pattern:
        "radial-gradient(circle at 20% 20%, rgba(143,211,255,0.24), transparent 34%), radial-gradient(circle at 80% 0%, rgba(255,211,107,0.18), transparent 30%)"
    },
    {
      name: "Carla",
      accent: "#ffb4c6",
      gradient: "linear-gradient(135deg,#2a0f2a,#3c1c3f)",
      pattern:
        "radial-gradient(circle at 12% 40%, rgba(255,180,198,0.26), transparent 32%), radial-gradient(circle at 80% 70%, rgba(255,211,107,0.16), transparent 36%)"
    },
    {
      name: "Crosby",
      accent: "#ffd28d",
      gradient: "linear-gradient(135deg,#0a2040,#11345a)",
      pattern:
        "radial-gradient(circle at 30% 10%, rgba(255,210,141,0.25), transparent 30%), radial-gradient(circle at 80% 30%, rgba(143,211,255,0.18), transparent 34%)"
    }
  ];

  const accentByKey = {
    tank: "#8fd3ff",
    carla: "#ffb4c6",
    crosby: "#ffd28d",
    combo: "#c5e8ff"
  };

  const themeFor = (index) => slideThemes[index % slideThemes.length];

  // STATE
  let answers = $state(Array(questions.length).fill(null)); // each index is 0..3 or null
  let showCard = $state(false);
  let finalKey = $state(null); // 'tank' | 'carla' | 'crosby' | 'combo'
  let tally = $state({ tank: 0, carla: 0, crosby: 0 });
  let activeSlide = $state(0);
  const currentTheme = $derived(() => themeFor(activeSlide));

  // derived % progress
  const progressPct = $derived(() =>
    Math.round(
      (answers.filter((a) => a !== null).length / questions.length) * 100
    )
  );

  function selectAnswer(qIndex, choiceIndex) {
    const next = [...answers];
    next[qIndex] = choiceIndex;
    answers = next;
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
    activeSlide = questions.length - 1;

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
    activeSlide = 0;
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

    if (showCard) return;

    if (answers[activeSlide] === null) return;

    if (activeSlide < questions.length - 1) {
      activeSlide += 1;
    } else {
      computeResult();
    }
  }

  function goToSlide(index) {
    activeSlide = Math.max(0, Math.min(index, questions.length - 1));
  }

  function prevSlide() {
    goToSlide(activeSlide - 1);
  }

  function nextSlide() {
    if (answers[activeSlide] === null) {
      alert("Pick an answer to keep traveling!");
      return;
    }

    if (activeSlide < questions.length - 1) {
      activeSlide += 1;
      return;
    }

    computeResult();
  }
</script>

<svelte:window on:keydown={handleKey} />

<div
  class="w-full max-w-5xl space-y-6 rounded-[28px] border border-[rgba(255,255,255,0.08)]
         bg-[linear-gradient(135deg,#0d2343,#10345e)] p-6 sm:p-8 shadow-[0_25px_60px_rgba(4,12,28,0.55)] text-[#e6f0ff]"
>
  <div class="flex flex-wrap items-center justify-between gap-3 text-sm text-[#d0e4ff]">
    <div class="flex w-full flex-col gap-2 sm:w-auto sm:flex-row sm:items-center sm:gap-3">
      <div class="text-xs uppercase tracking-[0.2em]">
        Answered {answers.filter((a) => a !== null).length}/{questions.length} ({progressPct}%)
      </div>
      <div class="h-2 w-full overflow-hidden rounded-full bg-white/10 sm:w-48" aria-hidden="true">
        <div
          class="h-full rounded-full bg-[linear-gradient(90deg,#8fd3ff,#ffd36b)] transition-all duration-300 ease-out"
          style={`width:${progressPct}%`}
        ></div>
      </div>
    </div>
    <button
      type="button"
      onclick={resetQuiz}
      class="rounded-full border border-white/20 px-4 py-2 text-xs font-semibold uppercase tracking-[0.2em] text-[#cfe5ff]
             transition hover:border-white/40 hover:-translate-y-[2px] hover:shadow-[0_10px_24px_rgba(6,18,36,0.4)]"
    >
      Reset Journey
    </button>
  </div>

  <div
    class="relative overflow-hidden rounded-[26px] border border-white/5 bg-[#0f2a4d] p-5 sm:p-7"
    aria-live="polite"
  >
    <div class="absolute inset-0" style={`background:${currentTheme.gradient}`}></div>
    <div class="absolute inset-0 opacity-80" style={`background-image:${currentTheme.pattern}`}></div>
    <div
      class="absolute -left-24 top-8 h-40 w-40 rounded-full blur-3xl"
      style={`background: ${currentTheme.accent}33`}
      aria-hidden="true"
    ></div>
    <div
      class="absolute bottom-[-80px] right-[-60px] h-48 w-48 rounded-full blur-3xl"
      style="background: rgba(255,255,255,0.08)"
      aria-hidden="true"
    ></div>

    <div class="relative grid items-start gap-6 lg:grid-cols-[1fr_320px]">
      <div class="space-y-3">
        <div
          class="inline-flex items-center gap-2 rounded-full border border-white/30 bg-white/10 px-3 py-2 text-[11px] font-semibold uppercase tracking-[0.2em] text-white/90 shadow-[0_8px_22px_rgba(0,0,0,0.25)]"
          style={`border-color:${currentTheme.accent}`}
        >
          Q{activeSlide + 1}/{questions.length}
        </div>
        <h2 class="text-2xl font-semibold leading-snug text-white sm:text-[26px]">
          {questions[activeSlide].q}
        </h2>
        <p class="text-sm leading-relaxed text-white/80">
          Pick the reply that feels like you, then glide to the next question.
        </p>
      </div>

      <div class="grid gap-3">
        {#each questions[activeSlide].choices as choiceText, ci}
          {#if answers[activeSlide] === ci}
            <button
              type="button"
              class="flex w-full items-start gap-3 rounded-2xl border-2 px-4 py-3 text-left text-[14px] font-semibold text-white shadow-[0_12px_32px_rgba(0,0,0,0.35)]
                     transition duration-200"
              style={`border-color:${currentTheme.accent}; background:rgba(255,255,255,0.08); box-shadow:0 12px 32px rgba(0,0,0,0.35), 0 0 0 6px ${currentTheme.accent}22;`}
              aria-pressed="true"
              onclick={() => selectAnswer(activeSlide, ci)}
            >
              <span
                class="mt-[2px] flex h-5 w-5 items-center justify-center rounded-full border-2"
                style={`border-color:${currentTheme.accent}; background:${currentTheme.accent}`}
                aria-hidden="true"
              >
                ✓
              </span>
              <span class="leading-snug">{choiceText}</span>
            </button>
          {:else}
            <button
              type="button"
              class="flex w-full items-start gap-3 rounded-2xl border border-white/20 bg-white/5 px-4 py-3 text-left text-[14px] text-white/90
                     transition duration-150 hover:-translate-y-[2px] hover:border-white/40 hover:bg-white/10"
              aria-pressed="false"
              onclick={() => selectAnswer(activeSlide, ci)}
            >
              <span
                class="mt-[2px] h-5 w-5 rounded-full border-2 border-white/30"
                aria-hidden="true"
              ></span>
              <span class="leading-snug">{choiceText}</span>
            </button>
          {/if}
        {/each}
      </div>
    </div>

    <div class="relative mt-6 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
      <div class="flex flex-wrap items-center gap-2" aria-label="Question selector dots">
        {#each questions as _, idx}
          <button
            type="button"
            class={`h-3.5 rounded-full transition ${idx === activeSlide ? 'w-12 bg-white shadow-lg' : 'w-3 bg-white/40 hover:bg-white/70'}`}
            onclick={() => goToSlide(idx)}
            aria-label={`Go to question ${idx + 1}`}
            aria-current={idx === activeSlide}
          ></button>
        {/each}
      </div>
      <div class="flex flex-wrap gap-2">
        <button
          type="button"
          onclick={prevSlide}
          class="rounded-xl border border-white/25 px-4 py-2 text-sm font-semibold text-white/80 transition hover:border-white/50 hover:bg-white/10 disabled:cursor-not-allowed disabled:opacity-50"
          disabled={activeSlide === 0}
        >
          ← Back
        </button>
        <button
          type="button"
          onclick={nextSlide}
          class="rounded-xl border-0 bg-[linear-gradient(180deg,#8fd3ff,#4db7ff)] px-5 py-2 text-sm font-semibold text-[#052238] shadow-[0_14px_32px_rgba(0,0,0,0.35)] transition hover:-translate-y-[2px]"
        >
          {activeSlide === questions.length - 1 ? "See My Character" : "Next Question →"}
        </button>
      </div>
    </div>
  </div>

  {#if showCard && finalKey}
    <div
      id="resultCard"
      role="region"
      aria-live="polite"
      class="rounded-[24px] border border-white/5 bg-[linear-gradient(135deg,rgba(255,255,255,0.06),rgba(6,20,40,0.92))] p-6 text-[#e6f0ff] shadow-[0_20px_40px_rgba(0,0,0,0.4)]"
      style={`box-shadow:0 20px 40px rgba(0,0,0,0.4), 0 0 0 1px ${accentByKey[finalKey] ?? '#c5e8ff'}33;`}
    >
      <div class="flex flex-col items-start gap-4 sm:flex-row sm:items-start">
        <div
          class="flex h-[90px] w-[90px] items-center justify-center rounded-2xl border-2 text-[24px] font-extrabold text-[#052238]"
          style={`border-color:${accentByKey[finalKey] ?? '#c5e8ff'}; background:${(accentByKey[finalKey] ?? '#c5e8ff')}cc`}
          aria-hidden="true"
        >
          {finalKey === 'combo' ? results.combo.avatar : results[finalKey].avatar}
        </div>

        <div class="text-left">
          <p class="text-xs uppercase tracking-[0.24em] text-white/70">
            Cosmic Crew Match
          </p>
          <h2 class="m-0 text-[22px] font-semibold leading-snug text-white">
            {finalKey === 'combo'
              ? "You're a Cosmic Combo!"
              : `You're ${results[finalKey].title}!`}
          </h2>

          <div class="mt-2 text-[14px] leading-relaxed text-[#d1e3ff]">
            {finalKey === 'combo'
              ? results.combo.desc
              : results[finalKey].desc}
          </div>
        </div>
      </div>

      <div class="mt-4 flex flex-wrap gap-2">
        <button
          type="button"
          onclick={resetQuiz}
          class="rounded-xl border-0 bg-[linear-gradient(180deg,#8fd3ff,#4db7ff)] px-4 py-2 text-[14px] font-bold
                 text-[#052238] shadow-[0_10px_22px_rgba(77,183,255,0.18)]"
        >
          Take Again
        </button>

        <button
          type="button"
          onclick={shareResult}
          class="rounded-xl border border-[rgba(255,255,255,0.15)] bg-transparent px-3 py-2 text-[14px] font-semibold text-[#d1e3ff] transition hover:border-white/40"
        >
          Share
        </button>
      </div>
    </div>
  {/if}

  <footer
    class="flex flex-col items-start justify-between gap-3 text-[13px] text-[#9fb6d6] sm:flex-row sm:items-center"
  >
    <div class="opacity-90">
      Made with stardust • Crosby’s Cosmic Adventure
    </div>
    <div class="text-[13px] leading-snug text-[#9fb6d6]">
      Interactive quiz • no tracking
    </div>
  </footer>
</div>
