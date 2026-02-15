<script>
  // Svelte 5 runes
  import { resolve } from '$app/paths';

  // Import quiz data from separate files
  import { questions } from '$lib/data/quiz-questions';
  import { scoring } from '$lib/data/quiz-scoring';
  import { results, accentByKey } from '$lib/data/quiz-results';

  // Questions, scoring, results, and accentByKey are now imported from data files

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

  // accentByKey is now imported from quiz-results.js
  const characterImages = {
    tank: resolve("/Tank.png"),
    carla: resolve("/Carla.png"),
    crosby: resolve("/Crosby.png")
  };
  const characterImagesWebp = {
    tank: resolve("/Tank.webp"),
    carla: resolve("/Carla.webp"),
    crosby: resolve("/Crosby.webp")
  };
  const tankPattern =
    "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 200 140'%3E%3Crect width='200' height='140' fill='%23afab23'/%3E%3Cg fill='%234a715f'%3E%3Cpath d='M22 12 l3 9 9 3-9 3-3 9-3-9-9-3 9-3z'/%3E%3Cpath d='M82 26 l3 9 9 3-9 3-3 9-3-9-9-3 9-3z'/%3E%3Cpath d='M152 18 l3 9 9 3-9 3-3 9-3-9-9-3 9-3z'/%3E%3Cpath d='M48 76 l3 9 9 3-9 3-3 9-3-9-9-3 9-3z'/%3E%3Cpath d='M126 82 l3 9 9 3-9 3-3 9-3-9-9-3 9-3z'/%3E%3Cpath d='M168 112 l3 9 9 3-9 3-3 9-3-9-9-3 9-3z'/%3E%3C/g%3E%3C/svg%3E";
  const carlaPattern =
    "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 220 160'%3E%3Crect width='220' height='160' fill='%23ef6f3c'/%3E%3Cg fill='none' stroke='%23ffffff' stroke-opacity='0.95' stroke-width='1.6' stroke-linejoin='round' stroke-linecap='round'%3E%3Cpath d='M24 18 L29 30 L42 32 L32 39 L35 52 L24 45 L13 52 L16 39 L6 32 L19 30 Z'/%3E%3Cpath d='M112 24 L117 36 L129 38 L120 45 L123 58 L112 51 L101 58 L104 45 L95 38 L107 36 Z'/%3E%3Cpath d='M188 20 L193 31 L205 34 L196 41 L199 53 L188 47 L177 53 L180 41 L171 34 L183 31 Z'/%3E%3Cpath d='M56 84 L61 96 L74 98 L64 105 L67 118 L56 111 L45 118 L48 105 L38 98 L51 96 Z'/%3E%3Cpath d='M156 92 L161 103 L173 106 L164 113 L167 125 L156 119 L145 125 L148 113 L139 106 L151 103 Z'/%3E%3Cpath d='M98 132 L103 143 L115 146 L106 153 L109 165 L98 159 L87 165 L90 153 L81 146 L93 143 Z'/%3E%3C/g%3E%3C/svg%3E";
  const crosbyPattern =
    "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200'%3E%3Crect width='320' height='200' fill='%23b8cee8'/%3E%3Cg fill='%23d8dde5' opacity='0.95'%3E%3Cpolygon points='36,20 42,34 56,36 45,44 48,58 36,50 24,58 27,44 16,36 30,34'/%3E%3Cpolygon points='150,34 156,48 172,50 160,58 163,73 150,64 137,73 140,58 128,50 144,48'/%3E%3Cpolygon points='270,22 276,36 291,38 279,47 282,61 270,53 258,61 261,47 249,38 264,36'/%3E%3Cpolygon points='94,116 100,130 115,132 103,140 106,155 94,146 82,155 85,140 73,132 88,130'/%3E%3Cpolygon points='220,128 226,142 241,144 229,153 232,167 220,159 208,167 211,153 199,144 214,142'/%3E%3Cpolygon points='302,90 307,102 319,104 310,111 312,124 302,117 292,124 294,111 285,104 297,102'/%3E%3C/g%3E%3C/svg%3E";
  const comboPattern =
    "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 260 160'%3E%3Crect width='260' height='160' fill='%23f878ab'/%3E%3Cg fill='%23ffffff'%3E%3Cpath d='M18 12 L22 22 L18 32 L14 22 Z M8 22 L18 18 L28 22 L18 26 Z'/%3E%3Cpath d='M238 30 L242 40 L238 50 L234 40 Z M228 40 L238 36 L248 40 L238 44 Z'/%3E%3Cpath d='M110 124 L114 134 L110 144 L106 134 Z M100 134 L110 130 L120 134 L110 138 Z'/%3E%3C/g%3E%3Cg fill='%23d9d9dd'%3E%3Cpolygon points='64,22 67,29 74,30 69,35 70,42 64,38 58,42 59,35 54,30 61,29'/%3E%3Cpolygon points='208,14 211,21 218,22 213,27 214,34 208,30 202,34 203,27 198,22 205,21'/%3E%3Cpolygon points='42,108 45,115 52,116 47,121 48,128 42,124 36,128 37,121 32,116 39,115'/%3E%3C/g%3E%3Cg fill='%23f9d66a'%3E%3Cpath d='M148 12 L151 19 L158 22 L151 25 L148 32 L145 25 L138 22 L145 19 Z M136 22 L143 25 L146 32 L149 25 L156 22 L149 19 L146 12 L143 19 Z'/%3E%3Cpath d='M224 94 L227 101 L234 104 L227 107 L224 114 L221 107 L214 104 L221 101 Z M212 104 L219 107 L222 114 L225 107 L232 104 L225 101 L222 94 L219 101 Z'/%3E%3Cpath d='M92 68 L95 75 L102 78 L95 81 L92 88 L89 81 L82 78 L89 75 Z M80 78 L87 81 L90 88 L93 81 L100 78 L93 75 L90 68 L87 75 Z'/%3E%3C/g%3E%3C/svg%3E";
  const backgroundForResult = (key) => {
    if (key === 'tank') return `#afab23 url("${tankPattern}") center/300px repeat`;
    if (key === 'carla') return `#ef6f3c url("${carlaPattern}") center/260px repeat`;
    if (key === 'crosby') return `#b8cee8 url("${crosbyPattern}") center/420px repeat`;
    if (key === 'combo') return `#f878ab url("${comboPattern}") center/260px repeat`;
    return accentForResult(key);
  };
  const solidBackgroundForResult = (key) => {
    if (key === 'tank') return '#afab23';
    if (key === 'carla') return '#ef6f3c';
    if (key === 'crosby') return '#b8cee8';
    if (key === 'combo') return '#f878ab';
    return accentForResult(key);
  };
  const displayNameForResult = (key) => {
    if (key === 'tank') return 'TANK';
    if (key === 'carla') return 'CARLA';
    if (key === 'crosby') return 'CROSBY';
    if (key === 'combo') return 'ALL 3';
    return (results[key]?.title ?? '').toUpperCase();
  };

  const accentForResult = (key) => accentByKey[key] ?? '#c5e8ff';

  const themeFor = (index) => slideThemes[index % slideThemes.length];

  // STATE
  const shuffleIndices = (len) => {
    const arr = Array.from({ length: len }, (_, i) => i);
    for (let i = len - 1; i > 0; i -= 1) {
      const j = Math.floor(Math.random() * (i + 1));
      [arr[i], arr[j]] = [arr[j], arr[i]];
    }
    return arr;
  };

  const makeChoiceOrders = () => questions.map((q) => shuffleIndices(q.choices.length));

  let answers = $state(Array(questions.length).fill(null)); // each index is 0..3 or null
  let choiceOrders = $state(makeChoiceOrders()); // random order per question
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
  const questionImages = [
    resolve("/marshmellowPlanet.png"), // marshmallow planet
    resolve("/apples.png"), // apples
    resolve("/meteor.png"), // meteor
    resolve("/stars.png"), // stars
    resolve("/spacecat.png"), // spacecat
    resolve("/cloud.png"), // cloud
    resolve("/ufo.png"), // ufo
    resolve("/rocket.png") // rocket
  ];
  let questionBg = $state(questionColors[0] ?? '#ff7bac');
  let toast = $state(null); // { message: string, accent?: string } | null
  let toastTimer;
  let showShareMenu = $state(false);
  let slideDirection = $state('none'); // 'forward', 'back', or 'none'
  let isAnimating = $state(false);

  $effect(() => {
    questionBg = questionColors[activeSlide] ?? questionColors[0] ?? '#ff7bac';
  });

  $effect(() => {
    console.log('[Quiz debug] activeSlide:', activeSlide, 'questionBg:', questionBg, 'questionColors:', questionColors);
  });

  function selectAnswer(qIndex, choiceIndex) {
    // Input validation: ensure indices are within valid bounds
    if (!Number.isInteger(qIndex) || qIndex < 0 || qIndex >= questions.length) {
      console.error('Invalid question index:', qIndex);
      return;
    }
    if (!Number.isInteger(choiceIndex) || choiceIndex < 0 || choiceIndex >= questions[qIndex].choices.length) {
      console.error('Invalid choice index:', choiceIndex);
      return;
    }

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

    // tally scores with validation
    const t = { tank: 0, carla: 0, crosby: 0 };
    answers.forEach((choice, i) => {
      // Validate scoring data exists
      if (!scoring[i] || !scoring[i][choice]) {
        console.error('Invalid scoring data at question', i, 'choice', choice);
        return;
      }
      const add = scoring[i][choice];
      t.tank += (add.tank || 0);
      t.carla += (add.carla || 0);
      t.crosby += (add.crosby || 0);
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

    // smooth scroll to result and focus on it
    queueMicrotask(() => {
      const el = document.getElementById('resultCard');
      if (el) {
        el.scrollIntoView({ behavior: 'smooth', block: 'center' });
        // Focus on the result card for screen readers
        el.focus();
      }
    });
  }

  function resetQuiz() {
    answers = Array(questions.length).fill(null);
    choiceOrders = makeChoiceOrders();
    showCard = false;
    finalKey = null;
    tally = { tank: 0, carla: 0, crosby: 0 };
    activeSlide = 0;
    window.scrollTo({ top: 0, behavior: 'smooth' });
  }

  async function shareResult() {
    if (!showCard || !finalKey) return;

    // try Web Share API first (works on mobile)
    if (navigator.share) {
      const card = results[finalKey] ?? results.combo;
      const shareText = `${
        finalKey === 'combo' ? "You're a Cosmic Combo!" : `You're ${card.title}!`
      } — ${card.desc}
Find out your Crosby's Cosmic Adventure character!`;

      try {
        await navigator.share({
          title: "Crosby's Cosmic Adventure Quiz",
          text: shareText,
          url: window.location?.href
        });
        showToast('Shared your result!', accentByKey[finalKey] ?? currentTheme.accent);
        return;
      } catch (err) {
        // User cancelled or error occurred
        if (err.name !== 'AbortError') {
          console.log('Share failed:', err);
        }
      }
    }

    // fallback: show social share menu (desktop)
    showShareMenu = true;
  }

  function getShareUrl(platform) {
    if (!finalKey) return '';
    const card = results[finalKey] ?? results.combo;
    const shareText = `${
      finalKey === 'combo' ? "You're a Cosmic Combo!" : `You're ${card.title}!`
    } — ${card.desc}`;
    const url = window.location?.href ?? '';

    if (platform === 'email') {
      return `mailto:?subject=${encodeURIComponent("Crosby's Cosmic Adventure Quiz Result")}&body=${encodeURIComponent(shareText + '\n\n' + url)}`;
    }
    return '';
  }

  async function shareToInstagram() {
    if (!finalKey) return;
    const card = results[finalKey] ?? results.combo;
    const shareText = `${
      finalKey === 'combo' ? "You're a Cosmic Combo!" : `You're ${card.title}!`
    } — ${card.desc}
Find out your Crosby's Cosmic Adventure character! ${window.location?.href ?? ''}`.trim();

    if (navigator.share) {
      try {
        await navigator.share({
          title: "Crosby's Cosmic Adventure Quiz Result",
          text: shareText,
          url: window.location?.href
        });
        showToast('Shared!', accentByKey[finalKey] ?? currentTheme.accent);
        showShareMenu = false;
      } catch (err) {
        if (err.name !== 'AbortError') {
          await copyToClipboard();
        }
      }
    } else {
      await copyToClipboard();
    }
  }

  async function shareToTikTok() {
    if (!finalKey) return;
    const card = results[finalKey] ?? results.combo;
    const shareText = `${
      finalKey === 'combo' ? "You're a Cosmic Combo!" : `You're ${card.title}!`
    } — ${card.desc}
Find out your Crosby's Cosmic Adventure character! ${window.location?.href ?? ''}`.trim();

    if (navigator.share) {
      try {
        await navigator.share({
          title: "Crosby's Cosmic Adventure Quiz Result",
          text: shareText,
          url: window.location?.href
        });
        showToast('Shared!', accentByKey[finalKey] ?? currentTheme.accent);
        showShareMenu = false;
      } catch (err) {
        if (err.name !== 'AbortError') {
          await copyToClipboard();
        }
      }
    } else {
      await copyToClipboard();
    }
  }

  async function copyToClipboard() {
    if (!finalKey) return;
    const card = results[finalKey] ?? results.combo;
    const shareText = `${
      finalKey === 'combo' ? "You're a Cosmic Combo!" : `You're ${card.title}!`
    } — ${card.desc}
Find out your Crosby's Cosmic Adventure character! ${window.location?.href ?? ''}`.trim();

    try {
      await navigator.clipboard.writeText(shareText);
      showToast('Result copied to clipboard!', accentByKey[finalKey] ?? currentTheme.accent);
      showShareMenu = false;
    } catch {
      showToast('Could not copy to clipboard', '#f87171');
    }
  }

  function handleKey(e) {
    // Handle ESC key to close result card
    if (e.key === 'Escape' && showCard) {
      closeResult();
      return;
    }

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

  function goToSlide(index, direction = 'none') {
    if (isAnimating) return;
    const newSlide = Math.max(0, Math.min(index, questions.length - 1));
    if (newSlide === activeSlide) return;

    isAnimating = true;
    activeSlide = newSlide;

    setTimeout(() => {
      isAnimating = false;
    }, 600);
  }

  function prevSlide() {
    goToSlide(activeSlide - 1, 'back');
  }

  function nextSlide() {
    if (answers[activeSlide] === null) {
      showToast("Pick an answer to keep traveling!", currentTheme.accent);
      return;
    }

    if (activeSlide < questions.length - 1) {
      goToSlide(activeSlide + 1, 'forward');
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

  function closeResult() {
    showCard = false;
    finalKey = null;
    showShareMenu = false;
  }

  function handleClickOutside(event) {
    if (showShareMenu) {
      const shareButton = event.target.closest('.result-btn--share');
      const shareMenuEl = event.target.closest('.share-menu');
      if (!shareButton && !shareMenuEl) {
        showShareMenu = false;
      }
    }
  }
</script>

<svelte:window onkeydown={handleKey} onclick={handleClickOutside} />

<div class="relative w-full quiz-frame text-[#ffffff]">
  {#if toast}
    <div
      role="status"
      class="pointer-events-none fixed left-1/2 top-1/2 z-20 flex w-full max-w-full justify-center px-4 -translate-x-1/2 -translate-y-1/2"
      style={`color:${toast.accent ?? currentTheme.accent}`}
      aria-live="polite"
    >
      <div
        class="flex max-w-90 flex-1 items-center justify-center gap-0 rounded-2xl border px-4 py-3 text-sm shadow-[0_12px_28px_rgba(0,0,0,0.25)] backdrop-blur text-center"
        style={`background:linear-gradient(135deg, ${(toast.accent ?? currentTheme.accent)}22, #fffffff5); border-color:${(toast.accent ?? currentTheme.accent)}88; box-shadow:0 12px 28px rgba(0,0,0,0.25), 0 0 0 1px ${(toast.accent ?? currentTheme.accent)}44; color:#0b1a2d;`}
      >
        <div class="w-full">
          <div class="font-bold text-[16px] text-[#0b1a2d]">Heads up</div>
          <div class="mt-1 text-[14px] leading-relaxed text-[#1b1b1d] font-semibold">
            {toast.message}
          </div>
        </div>
      </div>
    </div>
  {/if}

  <div class="quiz-body">
    {#if showCard && finalKey}
      <div
        class="result-overlay"
        role="button"
        tabindex="0"
        onclick={closeResult}
        onkeydown={(e) => e.key === 'Enter' && closeResult()}
        aria-label="Close result card"
      >
        <div
          id="resultCard"
          role="dialog"
          aria-modal="true"
          aria-labelledby="result-title"
          tabindex="-1"
          class="result-card rounded-3xl border border-white/5 p-6 text-[#0b1a2d] shadow-[0_20px_40px_rgba(0,0,0,0.4)]"
          style={`--result-base:${solidBackgroundForResult(finalKey)}; --result-oval:#faed68; background:${backgroundForResult(finalKey)}; background-color:${solidBackgroundForResult(finalKey)}; opacity:1; backdrop-filter:none; -webkit-backdrop-filter:none; box-shadow:0 20px 40px rgba(0,0,0,0.4), 0 0 0 1px ${(accentForResult(finalKey))}55; border-color:${(accentForResult(finalKey))}55;`}
          onclick={(event) => event.stopPropagation()}
          onkeydown={(e) => e.stopPropagation()}
        >
          {#if finalKey === 'combo'}
            <div class="combo-stage" aria-label="All three characters result">
              <picture class="combo-character combo-character--tank">
                <source srcset={characterImagesWebp.tank} type="image/webp" />
                <img class="result-img combo-img" src={characterImages.tank} alt={results.tank.title} />
              </picture>
              <picture class="combo-character combo-character--crosby">
                <source srcset={characterImagesWebp.crosby} type="image/webp" />
                <img class="result-img combo-img" src={characterImages.crosby} alt={results.crosby.title} />
              </picture>
              <picture class="combo-character combo-character--carla">
                <source srcset={characterImagesWebp.carla} type="image/webp" />
                <img class="result-img combo-img" src={characterImages.carla} alt={results.carla.title} />
              </picture>
              <div class="combo-center-oval">
                <h2 id="result-title" class="result-title result-title--combo" aria-label="You are all 3">
                  <span class="result-title-youare" style="font-family:'Times New Roman', Times, serif;">
                    <span style="font-family:'Times New Roman', Times, serif;">YOU</span>
                    <span style="font-family:'Times New Roman', Times, serif;">ARE</span>
                  </span>
                  <span class="result-title-name">
                    {displayNameForResult(finalKey)}
                  </span>
                </h2>
                <div class="combo-desc">
                  {results.combo.desc}
                </div>
              </div>
            </div>
          {:else}
            <div class="result-main">
              <div class="result-figures" aria-hidden="true">
                <picture>
                  <source srcset={characterImagesWebp[finalKey]} type="image/webp" />
                  <img class="result-img single" src={characterImages[finalKey]} alt={results[finalKey].title} />
                </picture>
              </div>

              <div class="result-text">
                <h2 id="result-title" class="result-title" aria-label={`You are ${results[finalKey].title}`}>
                  <span class="result-title-youare" style="font-family:'Times New Roman', Times, serif;">
                    <span style="font-family:'Times New Roman', Times, serif;">YOU</span>
                    <span style="font-family:'Times New Roman', Times, serif;">ARE</span>
                  </span>
                  <span class="result-title-name">
                    {displayNameForResult(finalKey)}
                  </span>
                </h2>

                <div class="result-desc-shell">
                  <div class="result-desc-inner">
                    {results[finalKey].desc}
                  </div>
                </div>
              </div>
            </div>
          {/if}

          <div class="result-actions mt-4 flex flex-wrap gap-2">
            <button
              type="button"
              onclick={resetQuiz}
              class="result-btn result-btn--again rounded-xl px-4 py-2 text-[14px] font-bold"
              aria-label="Retake the character quiz"
            >
              Retake Quiz
            </button>

            <div class="relative">
              <button
                type="button"
                onclick={shareResult}
                class="result-btn result-btn--share rounded-xl px-3 py-2 text-[14px] font-semibold"
                aria-label="Share your quiz result"
              >
                Share Result
              </button>

              {#if showShareMenu}
                <div
                  class="share-menu"
                  role="menu"
                  onclick={(e) => e.stopPropagation()}
                  onkeydown={(e) => e.stopPropagation()}
                >
                  <button
                    type="button"
                    class="share-menu-close"
                    onclick={() => showShareMenu = false}
                    aria-label="Close share menu"
                  >×</button>

                  <div class="share-menu-title">Share your result</div>

                  <button
                    type="button"
                    class="share-menu-item"
                    onclick={shareToInstagram}
                  >
                    <svg viewBox="0 0 24 24" class="share-icon" aria-hidden="true">
                      <path fill="currentColor" d="M7.8 2h8.4C19.4 2 22 4.6 22 7.8v8.4a5.8 5.8 0 0 1-5.8 5.8H7.8C4.6 22 2 19.4 2 16.2V7.8A5.8 5.8 0 0 1 7.8 2m-.2 2A3.6 3.6 0 0 0 4 7.6v8.8C4 18.39 5.61 20 7.6 20h8.8a3.6 3.6 0 0 0 3.6-3.6V7.6C20 5.61 18.39 4 16.4 4H7.6m9.65 1.5a1.25 1.25 0 0 1 1.25 1.25A1.25 1.25 0 0 1 17.25 8 1.25 1.25 0 0 1 16 6.75a1.25 1.25 0 0 1 1.25-1.25M12 7a5 5 0 0 1 5 5 5 5 0 0 1-5 5 5 5 0 0 1-5-5 5 5 0 0 1 5-5m0 2a3 3 0 0 0-3 3 3 3 0 0 0 3 3 3 3 0 0 0 3-3 3 3 0 0 0-3-3z"/>
                    </svg>
                    Share to Instagram
                  </button>

                  <button
                    type="button"
                    class="share-menu-item"
                    onclick={shareToTikTok}
                  >
                    <svg viewBox="0 0 24 24" class="share-icon" aria-hidden="true">
                      <path fill="currentColor" d="M16.6 5.82s.51.5 0 0A4.278 4.278 0 0 1 15.54 3h-3.09v12.4a2.592 2.592 0 0 1-2.59 2.5c-1.42 0-2.6-1.16-2.6-2.6c0-1.72 1.66-3.01 3.37-2.48V9.66c-3.45-.46-6.47 2.22-6.47 5.64c0 3.33 2.76 5.7 5.69 5.7c3.14 0 5.69-2.55 5.69-5.7V9.01a7.35 7.35 0 0 0 4.3 1.38V7.3s-1.88.09-3.24-1.48z"/>
                    </svg>
                    Share to TikTok
                  </button>

                  <a
                    href={getShareUrl('email')}
                    class="share-menu-item"
                    onclick={() => showShareMenu = false}
                  >
                    <svg viewBox="0 0 24 24" class="share-icon" aria-hidden="true">
                      <path fill="currentColor" d="M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-8 5-8-5V6l8 5 8-5v2z"/>
                    </svg>
                    Share via Email
                  </a>

                  <button
                    type="button"
                    class="share-menu-item"
                    onclick={copyToClipboard}
                  >
                    <svg viewBox="0 0 24 24" class="share-icon" aria-hidden="true">
                      <path fill="currentColor" d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/>
                    </svg>
                    Copy to Clipboard
                  </button>
                </div>
              {/if}
            </div>
          </div>
        </div>
      </div>
    {/if}

      {#key activeSlide}
        <div class="slide-two-col" aria-live="polite">
          <div
            class={`question-panel ${isFirstSlide ? 'first-slide-card' : ''}`}
            style={`--question-bg:${questionBg}; background:${questionBg}; background-color:${questionBg};`}
          >
            <div class="question-content">
            <!-- Progress indicator -->
            <div class="question-progress" aria-label={`Question ${activeSlide + 1} of ${questions.length}`}>
              <span class="sr-only">Question {activeSlide + 1} of {questions.length}</span>
              <span aria-hidden="true" class="progress-text">
                {activeSlide + 1} / {questions.length}
              </span>
            </div>

            <h2 class={`question-text font-semibold leading-snug ${isFirstSlide ? 'first-slide-text first-slide-question' : ''}`}>
              {questions[activeSlide].q}
            </h2>
            <img
              class="question-illustration"
              src={questionImages[activeSlide]}
              alt="Decorative illustration for question {activeSlide + 1}"
              loading="lazy"
            />
          </div>
        </div>

        <div class="answer-panel">
          <div class="grid gap-3">
            {#each choiceOrders[activeSlide] as originalIndex, ci}
              {@const choiceText = questions[activeSlide].choices[originalIndex]}
              <button
                type="button"
                class={`flex w-full items-center gap-4 rounded-full border-2 px-5 py-3 text-left font-semibold shadow-[0_12px_32px_rgba(0,0,0,0.35)] transition duration-200 answer-card answer-border ${answers[activeSlide] === originalIndex ? 'selected' : ''}`}
                aria-pressed={answers[activeSlide] === originalIndex}
                onclick={() => selectAnswer(activeSlide, originalIndex)}
                style={`border-color:${currentTheme.accent};`}
              >
              <span
                class="answer-bullet flex items-center justify-center rounded-full border-2"
                style={`border-color:${currentTheme.accent}; background:${answers[activeSlide] === originalIndex ? currentTheme.accent : 'transparent'}`}
                aria-hidden="true"
              >
                {answers[activeSlide] === originalIndex ? '✓' : ''}
              </span>
              <span class="leading-snug answer-text">{choiceText}</span>
              </button>
            {/each}
          </div>

          <div class="nav-actions">
            {#if activeSlide === questions.length - 1}
            <button
              type="button"
              class="nav-cloud"
              onclick={resetQuiz}
            >
              Reset Journey
            </button>
          {:else}
            <button
              type="button"
              class="nav-cloud"
              onclick={prevSlide}
              disabled={activeSlide === 0}
            >
              ← Back
            </button>
          {/if}
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
      {/key}
    <div class="slide-footer">
      <div class="opacity-90">
        Made with stardust • Crosby’s Cosmic Adventure
      </div>
      <div class="text-[13px] leading-snug text-[#9fb6d6]">
        Interactive quiz • no tracking
      </div>
    </div>
  </div>
</div>

<style>
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
    font-size: clamp(24px, 6vw, 52px);
    line-height: 1.1;
  }

  @media (max-width: 640px) {
    .first-slide-question {
      font-size: clamp(20px, 5.5vw, 32px);
    }
  }

  .quiz-frame {
    width: clamp(320px, 70vw, 1200px);
    margin: 0 auto;
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .quiz-body {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 24px;
    min-height: clamp(520px, 80vh, 760px);
    height: auto;
  }

  @media (min-width: 641px) {
    .quiz-body {
      height: clamp(520px, 80vh, 760px);
    }
  }

  .slide-two-col {
    display: grid;
    grid-template-columns: 1.2fr 0.8fr;
    gap: 0;
    border-radius: 18px;
    overflow: hidden;
    flex: 1;
    min-height: 0;
    align-items: stretch;
    animation: slideIn 600ms cubic-bezier(0.165, 0.84, 0.44, 1);
  }

  @keyframes slideIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes slideInBack {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .slide-forward {
    animation: slideIn 600ms cubic-bezier(0.165, 0.84, 0.44, 1);
  }

  .slide-back {
    animation: slideInBack 600ms cubic-bezier(0.165, 0.84, 0.44, 1);
  }

  @media (prefers-reduced-motion: reduce) {
    .slide-two-col,
    .slide-forward,
    .slide-back {
      animation: none !important;
    }
  }

  .question-panel {
    position: relative;
    background: var(--question-bg, #ff7bac) !important;
    padding: clamp(16px, 3vw, 26px);
    display: flex;
    align-items: center;
    justify-content: center;
    color: #ffffff;
    border-radius: 18px 0 0 18px;
    text-transform: uppercase;
    font-family: 'DynaPuff', 'More Sugar', 'Chewy', system-ui, sans-serif;
    height: 100%;
  }

  .question-text {
    text-align: center;
    margin: 0 auto;
    font-size: clamp(18px, 4vw, 28px);
    line-height: 1.3;
  }

  .question-progress {
    position: absolute;
    top: 12px;
    left: 12px;
    background: rgba(255, 255, 255, 0.95);
    color: #1b1b1d;
    padding: 6px 12px;
    border-radius: 16px;
    font-size: 13px;
    font-weight: 600;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
    z-index: 10;
  }

  .progress-text {
    font-family: 'Nunito', sans-serif;
    letter-spacing: 0.02em;
  }

  .question-content {
    display: grid;
    gap: clamp(12px, 2vw, 16px);
    justify-items: center;
    align-items: center;
    text-align: center;
    width: 100%;
    max-width: 100%;
  }

  .question-illustration {
    width: clamp(160px, 32vw, 220px);
    height: clamp(120px, 24vw, 180px);
    object-fit: cover;
  }

  .answer-panel {
    background: #edc257;
    padding: 20px 18px;
    border-radius: 0 18px 18px 0;
    position: relative;
    display: grid;
    grid-template-rows: 1fr auto;
    gap: 12px;
  }

  @media (max-width: 768px) {
    .slide-two-col {
      grid-template-columns: 1fr;
      min-height: unset;
      height: auto;
      gap: 0;
    }

    .question-panel {
      border-radius: 18px 18px 0 0;
      padding: 18px 16px;
      min-height: 240px;
    }

    .question-progress {
      top: 10px;
      left: 10px;
    }

    .answer-panel {
      border-radius: 0 0 18px 18px;
      padding: 16px 14px 18px;
      grid-template-rows: auto auto;
    }
  }

  
  .answer-card {
    background: #edc257 !important;
    border-radius: 999px !important;
    border-width: 3px;
    align-items: center;
  }

  .answer-text {
    font-family: 'Oswald', 'Chewy', system-ui, sans-serif;
    text-transform: uppercase;
    color: #ffffff;
    font-size: clamp(18px, 3.4vw, 24px);
    line-height: 1.3;
  }

  .answer-border {
    border: 3px solid #ffffff;
  }

  .answer-bullet {
    width: 18px;
    height: 18px;
    min-width: 18px;
    min-height: 18px;
    border-radius: 999px !important;
    flex-shrink: 0;
  }

  .selected.answer-border {
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.35), 0 0 0 6px rgba(5, 34, 56, 0.2);
  }

  .slide-footer {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    gap: 6px;
    margin-top: auto;
    text-align: left;
    font-size: 13px;
    color: #9fb6d6;
    padding: 8px 4px 0;
  }

  @media (min-width: 640px) {
    .slide-footer {
      flex-direction: row;
      align-items: center;
      text-align: left;
    }
  }

  .result-overlay {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 16px;
    background: rgba(6, 12, 26, 0.55);
    backdrop-filter: blur(6px);
    border-radius: 18px;
    z-index: 15;
  }

  .result-card {
    width: 80%;
    height: 80%;
    max-width: 960px;
    max-height: 640px;
    display: flex;
    flex-direction: column;
  }

  .result-main {
    display: grid;
    grid-template-columns: 1fr 2fr;
    gap: 16px;
    flex: 1;
    min-height: 0;
    align-items: start;
  }

  .result-figures {
    position: relative;
    display: grid;
    grid-auto-flow: column;
    align-items: center;
    justify-content: center;
    gap: clamp(10px, 1.2vw, 14px);
    padding: clamp(16px, 2vw, 20px);
    min-height: 100%;
    overflow: visible;
    z-index: 0;
  }

  .result-figures::before {
    content: '';
    position: absolute;
    inset: -8% -12%;
    background: var(--result-oval, #faed68);
    border-radius: 50%;
    --result-oval-mask: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100' preserveAspectRatio='none'%3E%3Cpath fill='white' d='M12 55 C10 36 24 18 45 14 C66 8 88 20 91 40 C95 58 87 78 66 86 C45 93 20 82 12 55 Z'/%3E%3C/svg%3E");
    -webkit-mask-image: var(--result-oval-mask);
    mask-image: var(--result-oval-mask);
    -webkit-mask-size: 100% 100%;
    mask-size: 100% 100%;
    -webkit-mask-repeat: no-repeat;
    mask-repeat: no-repeat;
    z-index: -1;
    box-shadow: 0 12px 26px rgba(0, 0, 0, 0.18);
  }

  .result-text {
    text-align: center;
    min-width: 0;
    color: #0b1a2d;
  }

  .result-img {
    width: clamp(180px, 22vw, 280px);
    height: clamp(180px, 22vw, 280px);
    object-fit: contain;
    display: block;
  }

  .result-img.single {
    width: clamp(290px, 36vw, 460px);
    height: clamp(290px, 36vw, 460px);
  }

  .result-title {
    margin: 0;
    display: flex;
    align-items: stretch;
    justify-content: center;
    width: 100%;
    column-gap: clamp(4px, 0.8vw, 8px);
    color: #0b1a2d;
    font-weight: 700;
  }

  .result-title-youare {
    display: grid;
    grid-template-rows: 1fr 1fr;
    align-items: center;
    justify-items: center;
    font-size: clamp(28px, 3.4vw, 50px);
    line-height: 1;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    font-family: 'Times New Roman', Times, serif !important;
  }

  .result-title-name {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 100%;
    font-size: clamp(56px, 7vw, 104px);
    line-height: 1;
    text-transform: uppercase;
    font-family: 'Arial Black', Arial, sans-serif;
  }

  .result-desc-shell {
    margin-top: 4px;
    width: 100%;
    max-width: 100%;
    align-self: stretch;
    padding: 30px 32px;
    border-radius: 24px;
    background: var(--result-oval, #faed68);
    --result-rect-mask: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100' preserveAspectRatio='none'%3E%3Cpath fill='white' d='M10 20 C12 12 20 8 30 10 C46 7 64 7 80 10 C90 8 96 14 96 24 C98 40 98 60 96 76 C95 88 88 94 78 92 C60 95 40 95 22 92 C12 94 5 88 6 76 C4 60 4 40 6 24 C6 22 8 21 10 20 Z'/%3E%3C/svg%3E");
    -webkit-mask-image: var(--result-rect-mask);
    mask-image: var(--result-rect-mask);
    -webkit-mask-size: 104% 104%;
    mask-size: 104% 104%;
    -webkit-mask-position: center;
    mask-position: center;
    -webkit-mask-repeat: no-repeat;
    mask-repeat: no-repeat;
  }

  .result-desc-inner {
    width: min(90%, 700px);
    margin: 0 auto;
    font-size: clamp(16px, 1.7vw, 24px);
    line-height: 1.24;
    color: #0b1a2d;
    background: var(--result-base, #ffffff);
    border-radius: 14px;
    min-height: clamp(120px, 16vh, 180px);
    padding: clamp(26px, 3.6vw, 40px);
    -webkit-mask-image: var(--result-rect-mask);
    mask-image: var(--result-rect-mask);
    -webkit-mask-size: 104% 104%;
    mask-size: 104% 104%;
    -webkit-mask-position: center;
    mask-position: center;
    -webkit-mask-repeat: no-repeat;
    mask-repeat: no-repeat;
    font-family: 'More Sugar', 'DynaPuff', 'Chewy', system-ui, sans-serif;
  }

  .result-actions {
    margin-top: 14px;
    justify-content: center;
  }

  .result-btn {
    border: none;
    box-shadow: 0 10px 22px rgba(0, 0, 0, 0.2);
    transition: transform 140ms ease, box-shadow 160ms ease, filter 140ms ease;
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
  }

  .result-btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 14px 26px rgba(0, 0, 0, 0.24);
    filter: brightness(1.05);
  }

  .result-btn:active {
    transform: translateY(0);
    box-shadow: 0 8px 18px rgba(0, 0, 0, 0.2);
  }

  .result-btn:focus-visible {
    outline: 3px solid rgba(255, 255, 255, 0.9);
    outline-offset: 2px;
  }

  .result-btn--again {
    background: rgba(250, 237, 104, 0.85);
    color: #0b1a2d;
  }

  .result-btn--share {
    background: rgba(255, 111, 145, 0.9);
    color: #ffffff;
  }

  .combo-stage {
    position: relative;
    flex: 1;
    min-height: 0;
  }

  .combo-center-oval {
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    width: 96%;
    height: calc(100% - 44px);
    background: #faed68;
    color: #0b1a2d;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: clamp(10px, 1.2vw, 16px);
    padding: clamp(36px, 4vw, 56px);
    text-align: center;
    border-radius: 50%;
    --combo-oval-mask: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 200 140' preserveAspectRatio='none'%3E%3Cpath fill='white' d='M12 70 C12 34 44 12 100 12 C156 12 188 34 188 70 C188 106 156 128 100 128 C44 128 12 106 12 70 Z'/%3E%3C/svg%3E");
    -webkit-mask-image: var(--combo-oval-mask);
    mask-image: var(--combo-oval-mask);
    -webkit-mask-size: 100% 100%;
    mask-size: 100% 100%;
    -webkit-mask-repeat: no-repeat;
    mask-repeat: no-repeat;
    box-shadow: 0 14px 30px rgba(0, 0, 0, 0.2);
    z-index: 1;
  }

  .result-title--combo {
    margin: 0;
  }

  .combo-desc {
    max-width: min(62%, 520px);
    font-size: clamp(22px, 2.4vw, 36px);
    line-height: 1.2;
    color: #0b1a2d;
    font-family: 'More Sugar', 'DynaPuff', 'Chewy', system-ui, sans-serif;
  }

  .combo-character {
    position: absolute;
    z-index: 4;
  }

  .combo-character--tank {
    left: -2%;
    top: auto;
    bottom: -2%;
    transform: none;
  }

  .combo-character--crosby {
    left: 2%;
    top: -2%;
    transform: none;
  }

  .combo-character--carla {
    right: -2%;
    top: auto;
    bottom: -2%;
    transform: none;
  }

  .combo-img {
    width: clamp(160px, 18vw, 260px);
    height: clamp(160px, 18vw, 260px);
  }

  @media (max-width: 900px) {
    .result-card {
      width: 94%;
      height: auto;
      max-height: 88vh;
      padding: 20px 16px;
      overflow-y: auto;
    }

    .result-main {
      grid-template-columns: 1fr;
      gap: 18px;
    }

    .combo-center-oval {
      width: 98%;
      height: calc(100% - 40px);
      top: 52%;
      padding: clamp(24px, 5vw, 36px);
    }

    .combo-desc {
      max-width: min(82%, 440px);
      font-size: clamp(16px, 4vw, 24px);
      line-height: 1.3;
    }

    .combo-character--tank {
      left: -4%;
      top: auto;
      bottom: 0%;
      transform: none;
    }

    .combo-character--crosby {
      top: -4%;
      left: 4%;
      transform: none;
    }

    .combo-character--carla {
      right: -4%;
      top: auto;
      bottom: 0%;
      transform: none;
    }

    .combo-img {
      width: clamp(110px, 25vw, 180px);
      height: clamp(110px, 25vw, 180px);
    }

    .result-title {
      gap: 6px;
      flex-wrap: wrap;
    }

    .result-title-youare {
      grid-template-columns: repeat(2, auto);
      grid-template-rows: 1fr;
      column-gap: 6px;
      justify-content: center;
      font-size: clamp(24px, 5vw, 38px);
    }

    .result-title-name {
      min-height: auto;
      font-size: clamp(36px, 9vw, 64px);
      width: 100%;
    }

    .result-desc-shell {
      width: 100%;
      padding: clamp(18px, 4vw, 24px);
      margin-top: 8px;
    }

    .result-desc-inner {
      font-size: clamp(15px, 3.5vw, 20px);
      line-height: 1.4;
    }

    .result-figures {
      min-height: 180px;
      padding: 12px;
    }

    .result-img.single {
      width: clamp(200px, 48vw, 320px);
      height: clamp(200px, 48vw, 320px);
    }

    .result-actions {
      flex-direction: row;
      gap: 8px;
      margin-top: 12px;
    }

    .result-btn {
      font-size: clamp(12px, 3vw, 14px);
      padding: 10px 16px;
    }
  }

  .nav-actions {
    display: flex;
    gap: 12px;
    justify-content: flex-end;
    align-items: flex-end;
    margin-top: 0;
  }

  .nav-cloud {
    position: relative;
    padding: 14px 22px;
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

  @media (max-width: 640px) {
    .quiz-body {
      min-height: auto;
      height: auto;
      gap: 16px;
    }

    .question-panel {
      min-height: 280px;
      padding: 16px 12px;
    }

    .question-progress {
      top: 8px;
      left: 8px;
      padding: 4px 10px;
      font-size: 11px;
    }

    .question-text {
      font-size: clamp(15px, 4.2vw, 20px);
      line-height: 1.2;
    }

    .question-illustration {
      width: clamp(120px, 40vw, 180px);
      height: clamp(90px, 30vw, 140px);
    }

    .answer-panel {
      padding: 14px 12px 16px;
      gap: 10px;
    }

    .answer-card {
      padding: 12px 16px !important;
    }

  .answer-text {
    font-size: clamp(14px, 3.2vw, 18px);
    line-height: 1.4;
  }

  @media (max-width: 640px) {
    .answer-text {
      font-size: clamp(13px, 3.8vw, 16px);
    }
  }

    .nav-actions {
      flex-direction: column;
      align-items: stretch;
      gap: 8px;
      margin-top: 8px;
    }

    .nav-cloud {
      width: 100%;
      text-align: center;
      padding: 12px 18px;
      font-size: 14px;
    }

    .slide-footer {
      padding: 12px 4px 4px;
      font-size: 11px;
    }
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

  @media (max-width: 480px) {
    .result-card {
      width: 96%;
      max-height: 90vh;
      padding: 16px 12px;
    }

    .result-title-youare {
      font-size: clamp(20px, 5.5vw, 32px);
      column-gap: 4px;
    }

    .result-title-name {
      font-size: clamp(32px, 10vw, 56px);
    }

    .result-desc-shell {
      padding: 16px;
      margin-top: 6px;
    }

    .result-desc-inner {
      font-size: clamp(14px, 3.8vw, 18px);
    }

    .combo-desc {
      font-size: clamp(14px, 4.2vw, 20px);
      max-width: 85%;
    }

    .combo-img {
      width: clamp(90px, 24vw, 150px);
      height: clamp(90px, 24vw, 150px);
    }

    .result-img.single {
      width: clamp(170px, 52vw, 280px);
      height: clamp(170px, 52vw, 280px);
    }

    .result-btn {
      font-size: 12px;
      padding: 8px 14px;
    }
  }

  .share-menu {
    position: absolute;
    bottom: calc(100% + 8px);
    left: 50%;
    transform: translateX(-50%);
    background: rgba(255, 255, 255, 0.98);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    border-radius: 16px;
    padding: 16px;
    box-shadow: 0 12px 36px rgba(0, 0, 0, 0.25), 0 0 0 1px rgba(0, 0, 0, 0.1);
    min-width: 240px;
    z-index: 100;
    animation: shareMenuFadeIn 200ms ease;
  }

  @keyframes shareMenuFadeIn {
    from {
      opacity: 0;
      transform: translateX(-50%) translateY(4px);
    }
    to {
      opacity: 1;
      transform: translateX(-50%) translateY(0);
    }
  }

  .share-menu-close {
    position: absolute;
    top: 8px;
    right: 8px;
    width: 24px;
    height: 24px;
    border: none;
    background: rgba(0, 0, 0, 0.05);
    border-radius: 50%;
    color: #0b1a2d;
    font-size: 18px;
    line-height: 1;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 140ms ease;
  }

  .share-menu-close:hover {
    background: rgba(0, 0, 0, 0.1);
  }

  .share-menu-title {
    font-size: 14px;
    font-weight: 700;
    color: #0b1a2d;
    margin-bottom: 12px;
    padding-right: 24px;
  }

  .share-menu-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 12px;
    border-radius: 8px;
    background: transparent;
    border: none;
    color: #0b1a2d;
    font-size: 14px;
    font-weight: 600;
    text-decoration: none;
    cursor: pointer;
    transition: background 140ms ease, transform 100ms ease;
    width: 100%;
    text-align: left;
  }

  .share-menu-item:hover {
    background: rgba(0, 0, 0, 0.05);
    transform: translateX(2px);
  }

  .share-icon {
    width: 20px;
    height: 20px;
    flex-shrink: 0;
  }

  @media (max-width: 640px) {
    .share-menu {
      position: fixed;
      bottom: 16px;
      left: 50%;
      transform: translateX(-50%);
      min-width: calc(100vw - 32px);
      max-width: 400px;
    }

    @keyframes shareMenuFadeIn {
      from {
        opacity: 0;
        transform: translateX(-50%) translateY(20px);
      }
      to {
        opacity: 1;
        transform: translateX(-50%) translateY(0);
      }
    }
  }
</style>
