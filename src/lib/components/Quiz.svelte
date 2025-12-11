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

  const accentForResult = (key) => accentByKey[key] ?? '#c5e8ff';

  const themeFor = (index) => slideThemes[index % slideThemes.length];

  // STATE
  let answers = $state(Array(questions.length).fill(null)); // each index is 0..3 or null
  let showCard = $state(false);
  let finalKey = $state(null); // 'tank' | 'carla' | 'crosby' | 'combo'
  let tally = $state({ tank: 0, carla: 0, crosby: 0 });
  let activeSlide = $state(0);
  const currentTheme = $derived(() => themeFor(activeSlide));
  const isFirstSlide = $derived(() => activeSlide === 0);
  const questionColors = [
    '#ff7bac', // slide 1
    '#b8cee8', // slide 2
    '#d3b6d3', // slide 3
    '#52afce', // slide 4
    '#712c48', // slide 5
    '#ef6f3c', // slide 6
    '#96701e', // slide 7
    '#aacc96'  // slide 8
  ];
  let questionBg = questionColors[0] ?? '#ff7bac';
  let toast = $state(null); // { message: string, accent?: string } | null
  let toastTimer;

  $effect(() => {
    questionBg = questionColors[activeSlide] ?? questionColors[0] ?? '#ff7bac';
  });

  $effect(() => {
    console.log('[Quiz debug] activeSlide:', activeSlide, 'questionBg:', questionBg, 'questionColors:', questionColors);
  });

  function selectAnswer(qIndex, choiceIndex) {
    const next = [...answers];
    next[qIndex] = choiceIndex;
    answers = next;
  }

  function computeResult() {
    // check completeness
    if (answers.includes(null)) {
      showToast(
        `You're almost there! Answer all ${questions.length} to see your match.`,
        currentTheme.accent
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
        showToast('Shared your result!', accentByKey[finalKey] ?? currentTheme.accent);
        return;
      } catch {
        /* ignore if user cancels */
      }
    }

    // fallback: copy to clipboard
    try {
      await navigator.clipboard.writeText(shareText);
      showToast('Result copied — paste it anywhere to share!', accentByKey[finalKey] ?? currentTheme.accent);
    } catch {
      showToast('Could not copy to clipboard', '#f87171');
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
      showToast("Pick an answer to keep traveling!", currentTheme.accent);
      return;
    }

    if (activeSlide < questions.length - 1) {
      activeSlide += 1;
      return;
    }

    computeResult();
  }

  function showToast(message, accent = currentTheme.accent) {
    if (toastTimer) {
      clearTimeout(toastTimer);
    }
    toast = { message, accent };
    toastTimer = setTimeout(() => {
      toast = null;
    }, 2600);
  }
</script>

<svelte:window on:keydown={handleKey} />

<div class="relative w-full quiz-frame text-[#ffffff]">
  {#if toast}
    <div
      role="status"
      class="pointer-events-none fixed left-1/2 top-1/2 z-20 flex w-full max-w-full justify-center px-4 -translate-x-1/2 -translate-y-1/2"
      style={`color:${toast.accent ?? currentTheme.accent}`}
      aria-live="polite"
    >
      <div
        class="flex max-w-[360px] flex-1 items-center justify-center gap-0 rounded-2xl border px-4 py-3 text-sm shadow-[0_12px_28px_rgba(0,0,0,0.25)] backdrop-blur text-center"
        style={`background:linear-gradient(135deg, ${(toast.accent ?? currentTheme.accent)}22, #fffffff0); border-color:${(toast.accent ?? currentTheme.accent)}66; box-shadow:0 12px 28px rgba(0,0,0,0.25), 0 0 0 1px ${(toast.accent ?? currentTheme.accent)}33; color:#0b1a2d;`}
      >
        <div class="w-full">
          <div class="font-semibold text-[16px] text-[#0b1a2d]">Heads up</div>
          <div class="mt-1 text-[13px] leading-relaxed text-[#0f2237]">
            {toast.message}
          </div>
        </div>
      </div>
    </div>
  {/if}

  <div class="space-y-6">
    <div class="flex flex-wrap items-center justify-end gap-3 text-sm text-[#ffffff]">
      <button
        type="button"
        onclick={resetQuiz}
        class="rounded-full border border-white px-4 py-2 text-xs font-semibold uppercase tracking-[0.2em] text-[#ff7bac]
               transition hover:border-white hover:-translate-y-[2px] hover:shadow-[0_10px_24px_rgba(6,18,36,0.25)]"
      >
        Reset Journey
      </button>
    </div>

    <div class="slide-two-col" aria-live="polite">
      <div
        class={`question-panel ${isFirstSlide ? 'first-slide-card' : ''}`}
        style={`--question-bg:${questionBg}; background:${questionBg}; background-color:${questionBg};`}
      >
        <h2 class={`question-text text-2xl font-semibold leading-snug ${isFirstSlide ? 'first-slide-text first-slide-question' : ''}`}>
        {questions[activeSlide].q}
      </h2>
    </div>

    <div class="answer-panel">
      <div class="grid gap-3">
        {#each questions[activeSlide].choices as choiceText, ci}
          <button
            type="button"
            class={`flex w-full items-start gap-3 rounded-2xl border-2 px-4 py-3 text-left text-[14px] font-semibold shadow-[0_12px_32px_rgba(0,0,0,0.35)] transition duration-200 answer-card answer-border ${answers[activeSlide] === ci ? 'selected' : ''}`}
            aria-pressed={answers[activeSlide] === ci}
            onclick={() => selectAnswer(activeSlide, ci)}
            style={`border-color:${currentTheme.accent};`}
          >
            <span
              class="mt-[2px] flex h-5 w-5 items-center justify-center rounded-full border-2"
              style={`border-color:${currentTheme.accent}; background:${answers[activeSlide] === ci ? currentTheme.accent : 'transparent'}`}
              aria-hidden="true"
            >
              {answers[activeSlide] === ci ? '✓' : ''}
            </span>
            <span class="leading-snug answer-text">{choiceText}</span>
          </button>
        {/each}
      </div>

      <div class="nav-actions">
        <button
          type="button"
          class="nav-cloud"
          onclick={prevSlide}
          disabled={activeSlide === 0}
        >
          ← Back
        </button>
        <button
          type="button"
          class="nav-cloud"
          onclick={nextSlide}
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
        class="rounded-[24px] border border-white/5 p-6 text-[#0b1a2d] shadow-[0_20px_40px_rgba(0,0,0,0.4)]"
        style={`background:${accentForResult(finalKey)}; box-shadow:0 20px 40px rgba(0,0,0,0.4), 0 0 0 1px ${(accentForResult(finalKey))}55; border-color:${(accentForResult(finalKey))}55;`}
      >
        <div class="flex flex-col items-start gap-4 sm:flex-row sm:items-start">
          <div
            class="flex h-[90px] w-[90px] items-center justify-center rounded-2xl border-2 text-[24px] font-extrabold text-[#ffffff]"
            style={`border-color:${accentForResult(finalKey)}; background:${accentForResult(finalKey)}cc`}
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
            class="rounded-xl border px-4 py-2 text-[14px] font-bold shadow-[0_10px_22px_rgba(0,0,0,0.2)] transition"
            style={`background:${accentForResult(finalKey)}dd; color:#0b1a2d; border-color:${accentForResult(finalKey)};`}
          >
            Take Again
          </button>

          <button
            type="button"
            onclick={shareResult}
            class="rounded-xl border px-3 py-2 text-[14px] font-semibold transition"
            style={`border-color:${accentForResult(finalKey)}; color:#0b1a2d; background:${accentForResult(finalKey)}2b;`}
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
</div>

<style>
  @import url('https://fonts.googleapis.com/css2?family=More+Sugar&display=swap');
  @import url('https://fonts.googleapis.com/css2?family=DynaPuff:wght@500;600&display=swap');
  @import url('https://fonts.googleapis.com/css2?family=Macabro+Danger&family=Oswald:wght@500;600&display=swap');

  .first-slide-card {
    background: var(--question-bg, #ff7bac);
    border-radius: 18px;
    padding: 16px;
    box-shadow: 0 12px 28px rgba(0, 0, 0, 0.25);
  }

  .first-slide-text {
    font-family: 'DynaPuff', 'More Sugar', 'Chewy', system-ui, sans-serif;
  }

  .first-slide-question {
    text-transform: uppercase;
    font-size: clamp(32px, 6vw, 52px);
    line-height: 1.1;
  }

  .quiz-frame {
    width: clamp(320px, 70vw, 1200px);
    margin: 0 auto;
  }

  .slide-two-col {
    display: grid;
    grid-template-columns: 1.2fr 0.8fr;
    gap: 0;
    border-radius: 18px;
    overflow: hidden;
    min-height: 400px;
  }

  .question-panel {
    background: var(--question-bg, #ff7bac) !important;
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 14px;
    color: #ffffff;
    border-radius: 18px 0 0 18px;
    text-transform: uppercase;
    font-family: 'DynaPuff', 'More Sugar', 'Chewy', system-ui, sans-serif;
    height: 100%;
  }

  .question-text {
    text-align: center;
    margin: 0 auto;
  }

  .answer-panel {
    background: #edc257;
    padding: 20px;
    border-radius: 0 18px 18px 0;
    position: relative;
  }

  @media (max-width: 768px) {
    .slide-two-col {
      grid-template-columns: 1fr;
      min-height: unset;
    }

    .question-panel {
      border-radius: 18px 18px 0 0;
    }

    .answer-panel {
      border-radius: 0 0 18px 18px;
    }
  }

  
  .answer-card {
    background: #edc257 !important;
  }

  .answer-text {
    font-family: 'Oswald', 'Chewy', system-ui, sans-serif;
    text-transform: uppercase;
    color: #ffffff;
    font-size: 20px;
    line-height: 1.35;
  }

  .answer-border {
    border: 3px solid #ffffff;
  }

  .selected.answer-border {
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.35), 0 0 0 6px rgba(5, 34, 56, 0.2);
  }

  .nav-actions {
    display: flex;
    gap: 12px;
    justify-content: flex-end;
    margin-top: 18px;
  }

  .nav-cloud {
    position: relative;
    padding: 16px 26px;
    background: #fff;
    color: #ff7bac;
    border: 3px solid #ffffff;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    font-size: 16px;
    cursor: pointer;
    border-radius: 999px;
    box-shadow: 0 10px 22px rgba(0, 0, 0, 0.18);
    -webkit-mask-image: var(--cloud-mask);
    mask-image: var(--cloud-mask);
    -webkit-mask-size: cover;
    mask-size: cover;
    -webkit-mask-repeat: no-repeat;
    mask-repeat: no-repeat;
    box-shadow: 0 10px 22px rgba(0, 0, 0, 0.18);
    transition: transform 120ms ease, box-shadow 140ms ease;
  }

  .nav-cloud:hover {
    transform: translateY(-2px);
    box-shadow: 0 14px 26px rgba(0, 0, 0, 0.2);
  }

  .nav-cloud:disabled {
    opacity: 0.55;
    cursor: not-allowed;
    box-shadow: none;
  }
</style>
