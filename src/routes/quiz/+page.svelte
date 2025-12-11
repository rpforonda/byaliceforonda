<script>
  import Quiz from '$lib/components/Quiz.svelte';

  let showIntro = $state(true);
  let showSplash = $state(false);

  function startQuiz() {
    console.log('Take the Quiz clicked');

    showIntro = false;
    showSplash = true;
    console.log('showIntro now', showIntro, 'showSplash now', showSplash);
  }

  function goToQuiz() {
    showSplash = false;
    console.log('Splash done, showing quiz');
  }

  const stars = [
    { left: '4%', size: '40px', duration: '4.2s', delay: '0s', rotate: '-8deg' },
    { left: '10%', size: '34px', duration: '3.8s', delay: '0.4s', rotate: '6deg' },
    { left: '16%', size: '48px', duration: '4.6s', delay: '0.2s', rotate: '-10deg' },
    { left: '22%', size: '30px', duration: '3.5s', delay: '0.8s', rotate: '14deg' },
    { left: '28%', size: '44px', duration: '4.1s', delay: '0.1s', rotate: '-4deg' },
    { left: '34%', size: '36px', duration: '3.6s', delay: '1s', rotate: '9deg' },
    { left: '40%', size: '52px', duration: '4.8s', delay: '0.3s', rotate: '-12deg' },
    { left: '48%', size: '32px', duration: '3.4s', delay: '1.2s', rotate: '11deg' },
    { left: '56%', size: '46px', duration: '4.4s', delay: '0.5s', rotate: '-6deg' },
    { left: '62%', size: '38px', duration: '3.7s', delay: '1.4s', rotate: '7deg' },
    { left: '68%', size: '42px', duration: '4s', delay: '0.7s', rotate: '-9deg' },
    { left: '74%', size: '34px', duration: '3.3s', delay: '1.1s', rotate: '5deg' },
    { left: '80%', size: '50px', duration: '4.5s', delay: '0.6s', rotate: '-11deg' },
    { left: '86%', size: '36px', duration: '3.6s', delay: '1.5s', rotate: '8deg' },
    { left: '92%', size: '44px', duration: '4.2s', delay: '0.9s', rotate: '-7deg' },
    { left: '96%', size: '30px', duration: '3.2s', delay: '1.8s', rotate: '6deg' },
    { left: '12%', size: '26px', duration: '3s', delay: '1.6s', rotate: '-5deg' },
    { left: '52%', size: '28px', duration: '3.1s', delay: '1.9s', rotate: '4deg' }
  ];
</script>

<section class="px-6 py-16 text-center">
  <div class="quiz-chewy mt-10 flex w-full justify-center">
    <div class="quiz-shell w-full">
      {#if showSplash}
        <div
          class="quiz-stage quiz-stage--splash relative isolate overflow-hidden rounded-[34px] bg-[#25533F] shadow-[0_18px_48px_rgba(0,0,0,0.26)]"
          role="presentation"
          aria-hidden="true"
        >
          <div class="splash-row flex h-full flex-col overflow-hidden sm:flex-row">
            <div class="splash-col bg-[#6b8a32]">
              <div class="splash-blob text-splash-green">
                TANK
              </div>
              <div class="splash-figure">
                <svg viewBox="0 0 140 180" aria-hidden="true" focusable="false">
                  <defs>
                    <linearGradient id="tankShell" x1="0%" y1="0%" x2="100%" y2="100%">
                      <stop offset="0%" stop-color="#2f4f16" />
                      <stop offset="100%" stop-color="#3b6a24" />
                    </linearGradient>
                  </defs>
                  <circle cx="70" cy="46" r="28" fill="#70c36b" />
                  <ellipse cx="70" cy="110" rx="42" ry="52" fill="url(#tankShell)" />
                  <ellipse cx="70" cy="114" rx="26" ry="32" fill="#79d37a" opacity="0.9" />
                  <rect x="44" y="78" width="12" height="56" rx="6" fill="#70c36b" />
                  <rect x="84" y="78" width="12" height="56" rx="6" fill="#70c36b" />
                  <rect x="56" y="148" width="12" height="20" rx="6" fill="#4c8f32" />
                  <rect x="72" y="148" width="12" height="20" rx="6" fill="#4c8f32" />
                  <path d="M52 44 Q70 24 88 44" stroke="#4c8f32" stroke-width="6" fill="none" stroke-linecap="round" />
                  <circle cx="58" cy="44" r="4" fill="#0c0c0c" />
                  <circle cx="82" cy="44" r="4" fill="#0c0c0c" />
                </svg>
              </div>
            </div>
            <div class="splash-col bg-[#c7e9ff]">
              <div class="splash-blob text-splash-blue">
                CROSBY
              </div>
              <div class="splash-figure">
                <svg viewBox="0 0 140 180" aria-hidden="true" focusable="false">
                  <defs>
                    <linearGradient id="crocBody" x1="0%" y1="0%" x2="100%" y2="100%">
                      <stop offset="0%" stop-color="#66c4b8" />
                      <stop offset="100%" stop-color="#4aa5a0" />
                    </linearGradient>
                  </defs>
                  <path d="M46 52 Q70 18 96 46 L92 72 Q88 84 70 86 Q52 84 48 72 Z" fill="url(#crocBody)" />
                  <circle cx="64" cy="50" r="5" fill="#0c0c0c" />
                  <circle cx="84" cy="50" r="5" fill="#0c0c0c" />
                  <path d="M60 62 Q70 68 80 62" stroke="#0c0c0c" stroke-width="4" fill="none" stroke-linecap="round" />
                  <rect x="52" y="78" width="12" height="58" rx="6" fill="url(#crocBody)" />
                  <rect x="76" y="78" width="12" height="58" rx="6" fill="url(#crocBody)" />
                  <ellipse cx="70" cy="122" rx="36" ry="46" fill="#6ed1c7" />
                  <rect x="60" y="148" width="12" height="22" rx="6" fill="#3c8d84" />
                  <rect x="78" y="148" width="12" height="22" rx="6" fill="#3c8d84" />
                </svg>
              </div>
            </div>
            <div class="splash-col bg-[#f09a58]">
              <div class="splash-blob text-splash-orange">
                CARLA
              </div>
              <div class="splash-figure">
                <svg viewBox="0 0 140 180" aria-hidden="true" focusable="false">
                  <defs>
                    <linearGradient id="capyBody" x1="0%" y1="0%" x2="100%" y2="100%">
                      <stop offset="0%" stop-color="#c47a3b" />
                      <stop offset="100%" stop-color="#a55a26" />
                    </linearGradient>
                  </defs>
                  <ellipse cx="70" cy="50" rx="30" ry="26" fill="url(#capyBody)" />
                  <circle cx="60" cy="48" r="5" fill="#0c0c0c" />
                  <circle cx="82" cy="48" r="5" fill="#0c0c0c" />
                  <path d="M58 60 Q70 68 82 60" stroke="#0c0c0c" stroke-width="4" fill="none" stroke-linecap="round" />
                  <ellipse cx="70" cy="116" rx="40" ry="50" fill="#d48b52" />
                  <rect x="50" y="82" width="12" height="60" rx="6" fill="url(#capyBody)" />
                  <rect x="78" y="82" width="12" height="60" rx="6" fill="url(#capyBody)" />
                  <rect x="58" y="150" width="12" height="22" rx="6" fill="#7c3f1d" />
                  <rect x="76" y="150" width="12" height="22" rx="6" fill="#7c3f1d" />
                </svg>
              </div>
            </div>
          </div>
          <div class="quiz-splash-actions">
            <button class="quiz-pill" type="button" onclick={goToQuiz}>
              NEXT
            </button>
          </div>
        </div>
      {:else if showIntro}
        <div
          class="quiz-stage quiz-stage--intro relative isolate overflow-hidden rounded-[34px] bg-[#25533F] p-8 shadow-[0_18px_48px_rgba(0,0,0,0.26)] sm:p-12"
          role="dialog"
          aria-modal="true"
          aria-label="Start the cosmic character quiz"
        >
          <div class="pointer-events-none absolute inset-0 opacity-90">
            <div class="absolute inset-0 bg-[radial-gradient(circle_at_18%_18%,#104836_0%,transparent_34%),radial-gradient(circle_at_84%_12%,#0f4e3a_0%,transparent_36%)]"></div>
            {#each stars as star}
              <span
                class="falling-star"
                style={`left:${star.left}; --size:${star.size}; animation-duration:${star.duration}; animation-delay:${star.delay}; --rotate:${star.rotate};`}
                aria-hidden="true"
              >
                <svg viewBox="0 0 100 100" aria-hidden="true" focusable="false">
                  <polygon
                    points="50,6 62,38 95,38 68,57 78,91 50,72 22,91 32,57 5,38 38,38"
                    fill="#ffc5d9"
                    stroke="#ffc5d9"
                    stroke-width="6"
                    stroke-linejoin="round"
                    stroke-linecap="round"
                  />
                </svg>
              </span>
            {/each}
          </div>

          <div class="relative z-10 flex flex-col items-center gap-8 text-center">
            <div class="blob-card w-full max-w-5xl px-16 py-18 text-[#7fa66d] sm:px-20 sm:py-22 min-h-[560px] sm:min-h-[660px] flex items-center justify-center">
              <p class="blob-heading">
                WHICH<br />
                <span class="cosmic-row">
                  <span class="whisker whisker-left" aria-hidden="true">
                    <span></span>
                    <span></span>
                    <span></span>
                  </span>
                  <span class="cosmic-word">COSMIC</span>
                  <span class="whisker whisker-right" aria-hidden="true">
                    <span></span>
                    <span></span>
                    <span></span>
                  </span>
                </span>
                <br />
                CHARACTER<br />
                ARE YOU?
              </p>
            </div>

            <button class="quiz-pill" type="button" onclick={startQuiz}>
              TAKE THE QUIZ
            </button>
          </div>
        </div>
      {:else}
        <div class="quiz-stage quiz-stage--quiz">
          <Quiz />
        </div>
      {/if}
    </div>
  </div>
</section>

<style>
  @import url('https://fonts.googleapis.com/css2?family=Chewy&display=swap');

  .quiz-shell {
    width: clamp(320px, 70vw, 1200px);
    margin: 0 auto;
  }

  .quiz-shell > * {
    width: 100%;
  }

  .quiz-stage {
    min-height: clamp(520px, 80vh, 760px);
    height: clamp(520px, 80vh, 760px);
    width: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  .quiz-stage--quiz {
    align-items: stretch;
    justify-content: flex-start;
  }

  .quiz-splash-actions {
    position: absolute;
    bottom: clamp(16px, 4vw, 28px);
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    justify-content: center;
  }

  .splash-row {
    flex: 1;
    min-height: 100%;
  }

  .falling-star {
    position: absolute;
    top: -20%;
    font-size: var(--size, 20px);
    animation: fall linear infinite;
    opacity: 0.88;
    width: var(--size, 20px);
    height: var(--size, 20px);
  }

  .falling-star svg {
    width: 100%;
    height: 100%;
    display: block;
    filter: none;
  }

  .falling-star polygon {
    fill: #d3b6d3;
    stroke: #d3b6d3;
  }

  @keyframes fall {
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

  .blob-card {
    position: relative;
    background: #edc257;
    --blob-mask: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100' preserveAspectRatio='none'%3E%3Cpath fill='white' d='M26 24 C34 14 50 16 58 22 C72 16 88 20 92 34 C102 42 102 58 92 66 C86 80 74 86 60 82 C50 90 34 88 26 80 C14 82 8 70 10 60 C2 52 4 38 12 32 C12 26 18 20 26 24 Z'/%3E%3C/svg%3E");
    -webkit-mask-image: var(--blob-mask);
    mask-image: var(--blob-mask);
    -webkit-mask-size: cover;
    mask-size: cover;
    -webkit-mask-repeat: no-repeat;
    mask-repeat: no-repeat;
    box-shadow: 0 18px 36px rgba(0, 0, 0, 0.18), 0 0 0 6px rgba(12, 63, 48, 0.45);
    font-family: 'Chewy', 'Fredoka', system-ui, sans-serif;
    transform-origin: center;
    padding: clamp(18px, 4vw, 42px) clamp(20px, 6vw, 54px);
    min-height: clamp(360px, 74vw, 580px);
  }

  .blob-heading {
    margin: 0;
    font-size: clamp(30px, 8vw, 62px);
    line-height: 1.05;
    font-weight: 600;
    letter-spacing: clamp(0.015em, 0.6vw, 0.03em);
    color: #6b8a32;
    max-width: 92%;
    margin-left: auto;
    margin-right: auto;
    text-align: center;
    text-wrap: balance;
    position: relative;
    z-index: 1;
  }

  .cosmic-row {
    display: inline-flex;
    align-items: center;
    gap: clamp(4px, 1.8vw, 12px);
    vertical-align: middle;
  }

  .whisker {
    position: relative;
    display: inline-flex;
    flex-direction: column;
    align-items: center;
    gap: clamp(6px, 1.6vw, 12px);
    margin: 0 clamp(8px, 3vw, 22px);
    vertical-align: middle;
    pointer-events: none;
    z-index: 2;
    transform: translateY(-6px);
  }

  .whisker span {
    display: block;
    width: clamp(28px, 16vw, 52px);
    height: clamp(6px, 2.2vw, 10px);
    background: #ef6f3c;
    border-radius: 999px;
    transform-origin: center;
    transform: rotate(var(--base-rot, 0deg));
    animation: whisker-wave 2.6s ease-in-out infinite;
  }

  .whisker-left span:nth-child(1) { --base-rot: 18deg; animation-delay: 0s; }
  .whisker-left span:nth-child(2) { --base-rot: 6deg; animation-delay: 0.12s; }
  .whisker-left span:nth-child(3) { --base-rot: -10deg; animation-delay: 0.22s; }

  .whisker-right span:nth-child(1) { --base-rot: -18deg; animation-delay: 0.05s; }
  .whisker-right span:nth-child(2) { --base-rot: -6deg; animation-delay: 0.16s; }
  .whisker-right span:nth-child(3) { --base-rot: 10deg; animation-delay: 0.26s; }

  @keyframes whisker-wave {
    0% { transform: rotate(calc(var(--base-rot, 0deg) - 4deg)); }
    50% { transform: rotate(calc(var(--base-rot, 0deg) + 4deg)) translateY(-2px); }
    100% { transform: rotate(calc(var(--base-rot, 0deg) - 4deg)); }
  }

  @media (max-width: 480px) {
    .blob-card {
      min-height: 320px;
      padding: 16px 18px;
    }

    .blob-heading {
      font-size: clamp(26px, 9vw, 46px);
      letter-spacing: 0.01em;
    }

    .cosmic-row {
      gap: 4px;
    }

    .whisker {
      gap: 4px;
      margin: 0 10px;
    }

    .whisker span {
      width: clamp(16px, 14vw, 32px);
      height: clamp(4px, 2vw, 6px);
    }
  }

  .quiz-pill {
    position: relative;
    border: none;
    background: #f5d255;
    color: #25533F;
    --pill-mask: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 40' preserveAspectRatio='none'%3E%3Cpath fill='white' d='M20 2 C34 -2 62 0 78 4 C92 6 100 14 98 24 C96 34 86 40 74 38 C60 44 40 42 28 38 C16 40 6 34 4 24 C2 12 8 4 20 2 Z'/%3E%3C/svg%3E");
    -webkit-mask-image: var(--pill-mask);
    mask-image: var(--pill-mask);
    -webkit-mask-size: cover;
    mask-size: cover;
    -webkit-mask-repeat: no-repeat;
    mask-repeat: no-repeat;
    padding: 16px 34px;
    font-family: 'Chewy', 'Fredoka', system-ui, sans-serif;
    font-size: 24px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    box-shadow:
      0 16px 36px rgba(0, 0, 0, 0.22),
      0 0 0 4px rgba(12, 63, 48, 0.35);
    transition: transform 120ms ease, box-shadow 140ms ease;
  }

  .quiz-pill:hover {
    transform: translateY(-2px);
    box-shadow:
      0 20px 40px rgba(0, 0, 0, 0.24),
      0 0 0 4px rgba(12, 63, 48, 0.35);
  }

  .quiz-pill:active {
    transform: translateY(0);
    box-shadow:
      0 12px 26px rgba(0, 0, 0, 0.2),
      0 0 0 4px rgba(12, 63, 48, 0.35);
  }

  :global(.quiz-chewy),
  :global(.quiz-chewy *) {
    font-family: 'Chewy', 'Fredoka', system-ui, sans-serif;
  }

  .splash-blob {
    position: absolute;
    top: 10px;
    left: 50%;
    transform: translateX(-50%);
    padding: clamp(22px, 5vw, 32px) clamp(34px, 9vw, 56px);
    height: clamp(32%, 16vw, 46%);
    min-height: 150px;
    min-width: 220px;
    display: grid;
    place-items: center;
    border: 3px solid rgba(140, 110, 20, 0.85);
    background: #f5d255;
    --splash-mask: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 160 120' preserveAspectRatio='none'%3E%3Cpath fill='white' d='M20 60 L36 16 L64 48 L84 10 L104 48 L132 16 L140 60 L132 94 L104 78 L80 110 L60 78 L32 96 Z'/%3E%3C/svg%3E");
    -webkit-mask-image: var(--splash-mask);
    mask-image: var(--splash-mask);
    -webkit-mask-size: cover;
    mask-size: cover;
    -webkit-mask-repeat: no-repeat;
    mask-repeat: no-repeat;
    font-family: 'Chewy', 'Fredoka', system-ui, sans-serif;
    font-size: clamp(28px, 7vw, 52px);
    letter-spacing: 0.12em;
    text-transform: uppercase;
    box-shadow:
      0 14px 28px rgba(0, 0, 0, 0.28),
      0 0 0 4px rgba(140, 110, 20, 0.4);
    text-align: center;
  }

  .splash-col {
    position: relative;
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    padding: clamp(48px, 6vw, 84px) clamp(18px, 4vw, 24px);
    gap: clamp(18px, 4vw, 26px);
  }

  .splash-figure {
    max-width: 180px;
    width: 72%;
    margin-top: auto;
    padding-bottom: clamp(18px, 3vw, 32px);
  }

  .splash-figure svg {
    width: 100%;
    height: auto;
    display: block;
  }

  .text-splash-green {
    color: #2f4f16;
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.28);
  }
  .text-splash-blue {
    color: #2f7fd9;
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.24);
  }
  .text-splash-orange {
    color: #a0400c;
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
  }
</style>
