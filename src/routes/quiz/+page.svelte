<script>
  import { resolve } from '$app/paths';
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

  const starfield = [
    { left: '6%', top: '12%', size: '68px', rotate: '-6deg', opacity: 0.9 },
    { left: '14%', top: '32%', size: '44px', rotate: '8deg', opacity: 0.8 },
    { left: '9%', top: '68%', size: '56px', rotate: '-12deg', opacity: 0.92 },
    { left: '18%', top: '82%', size: '84px', rotate: '4deg', opacity: 0.86 },
    { left: '26%', top: '18%', size: '60px', rotate: '-9deg', opacity: 0.88 },
    { left: '28%', top: '54%', size: '104px', rotate: '12deg', opacity: 0.94 },
    { left: '34%', top: '8%', size: '52px', rotate: '-4deg', opacity: 0.83 },
    { left: '40%', top: '36%', size: '92px', rotate: '10deg', opacity: 0.9 },
    { left: '48%', top: '68%', size: '64px', rotate: '-11deg', opacity: 0.82 },
    { left: '56%', top: '22%', size: '76px', rotate: '6deg', opacity: 0.88 },
    { left: '62%', top: '6%', size: '88px', rotate: '-7deg', opacity: 0.93 },
    { left: '68%', top: '46%', size: '48px', rotate: '9deg', opacity: 0.8 },
    { left: '74%', top: '74%', size: '72px', rotate: '-5deg', opacity: 0.9 },
    { left: '82%', top: '18%', size: '96px', rotate: '7deg', opacity: 0.87 },
    { left: '86%', top: '58%', size: '56px', rotate: '-8deg', opacity: 0.84 },
    { left: '92%', top: '10%', size: '80px', rotate: '11deg', opacity: 0.9 },
    { left: '88%', top: '86%', size: '60px', rotate: '-6deg', opacity: 0.82 },
    { left: '52%', top: '88%', size: '52px', rotate: '5deg', opacity: 0.78 },
    { left: '4%', top: '44%', size: '40px', rotate: '6deg', opacity: 0.76 },
    { left: '20%', top: '6%', size: '48px', rotate: '-10deg', opacity: 0.8 },
    { left: '32%', top: '72%', size: '44px', rotate: '12deg', opacity: 0.78 },
    { left: '46%', top: '8%', size: '60px', rotate: '-7deg', opacity: 0.85 },
    { left: '58%', top: '54%', size: '68px', rotate: '9deg', opacity: 0.88 },
    { left: '64%', top: '78%', size: '52px', rotate: '-9deg', opacity: 0.8 },
    { left: '70%', top: '30%', size: '80px', rotate: '5deg', opacity: 0.9 },
    { left: '78%', top: '50%', size: '44px', rotate: '-11deg', opacity: 0.76 },
    { left: '84%', top: '92%', size: '64px', rotate: '8deg', opacity: 0.86 },
    { left: '94%', top: '40%', size: '48px', rotate: '-5deg', opacity: 0.8 },
    { left: '60%', top: '94%', size: '56px', rotate: '7deg', opacity: 0.82 },
    { left: '10%', top: '92%', size: '48px', rotate: '-7deg', opacity: 0.8 },
    { left: '36%', top: '92%', size: '60px', rotate: '11deg', opacity: 0.84 },
    { left: '12%', top: '46%', size: '36px', rotate: '-4deg', opacity: 0.75 },
    { left: '2%', top: '26%', size: '40px', rotate: '10deg', opacity: 0.78 },
    { left: '8%', top: '8%', size: '36px', rotate: '-8deg', opacity: 0.74 },
    { left: '18%', top: '60%', size: '52px', rotate: '6deg', opacity: 0.82 },
    { left: '24%', top: '40%', size: '44px', rotate: '-10deg', opacity: 0.77 },
    { left: '30%', top: '30%', size: '56px', rotate: '12deg', opacity: 0.86 },
    { left: '44%', top: '46%', size: '48px', rotate: '-6deg', opacity: 0.8 },
    { left: '50%', top: '14%', size: '36px', rotate: '8deg', opacity: 0.74 },
    { left: '58%', top: '10%', size: '44px', rotate: '-12deg', opacity: 0.79 },
    { left: '66%', top: '64%', size: '60px', rotate: '4deg', opacity: 0.85 },
    { left: '72%', top: '12%', size: '52px', rotate: '-9deg', opacity: 0.82 },
    { left: '76%', top: '88%', size: '40px', rotate: '7deg', opacity: 0.78 },
    { left: '88%', top: '34%', size: '36px', rotate: '-7deg', opacity: 0.73 },
    { left: '96%', top: '72%', size: '48px', rotate: '5deg', opacity: 0.8 },
    { left: '42%', top: '78%', size: '44px', rotate: '-11deg', opacity: 0.79 }
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
            <div class="splash-col bg-[#afab23]">
              <div class="splash-blob text-splash-green">
                TANK
              </div>
              <div class="splash-figure">
                <picture>
                  <source srcset={resolve("/Tank.webp")} type="image/webp" />
                  <img src={resolve("/Tank.png")} alt="Tank" class="splash-img" />
                </picture>
              </div>
            </div>
            <div class="splash-col bg-[#b8cee8]">
              <div class="splash-blob text-splash-blue">
                CROSBY
              </div>
              <div class="splash-figure">
                <picture>
                  <source srcset={resolve("/Crosby.webp")} type="image/webp" />
                  <img src={resolve("/Crosby.png")} alt="Crosby" class="splash-img" />
                </picture>
              </div>
            </div>
            <div class="splash-col bg-[#ef6f3c]">
              <div class="splash-blob text-splash-orange">
                CARLA
              </div>
              <div class="splash-figure">
                <picture>
                  <source srcset={resolve("/Carla.webp")} type="image/webp" />
                  <img src={resolve("/Carla.png")} alt="Carla" class="splash-img" />
                </picture>
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
            <div
              class="absolute inset-0 bg-[radial-gradient(circle_at_18%_18%,#104836_0%,transparent_34%),radial-gradient(circle_at_84%_12%,#0f4e3a_0%,transparent_36%)]"
              aria-hidden="true"
            ></div>
            <div class="starfield" aria-hidden="true">
              {#each starfield as star}
                <span
                  class="starfield-star"
                  style={`left:${star.left}; top:${star.top}; width:calc(${star.size} * 0.9); height:calc(${star.size} * 0.9); transform: rotate(${star.rotate}); opacity:${star.opacity ?? 0.88};`}
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
          </div>

          <div class="relative z-10 flex flex-col items-center text-center">
            <div class="blob-card w-full max-w-3xl px-8 py-18 text-[#7fa66d] sm:px-12 sm:py-22 min-h-[560px] sm:min-h-[660px] flex items-center justify-center">
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

            <button class="quiz-pill intro-pill" type="button" onclick={startQuiz}>
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

  .starfield {
    position: absolute;
    inset: 0;
    pointer-events: none;
  }

  .starfield-star {
    position: absolute;
    display: block;
  }

  .starfield-star svg {
    width: 100%;
    height: 100%;
    display: block;
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
    font-family: 'More Sugar', 'DynaPuff', 'Chewy', system-ui, sans-serif;
    transform-origin: center;
    padding: clamp(18px, 4vw, 42px) clamp(24px, 6vw, 48px);
    min-height: clamp(360px, 74vw, 580px);
  }

  .blob-heading {
    margin: 0;
    font-size: clamp(36px, 10.5vw, 80px);
    line-height: 1.05;
    font-weight: 600;
    letter-spacing: clamp(0.018em, 0.85vw, 0.032em);
    color: #6b8a32;
    max-width: 92%;
    margin-left: auto;
    margin-right: auto;
    text-align: center;
    text-wrap: balance;
    position: relative;
    z-index: 1;
    font-family: 'More Sugar', 'DynaPuff', 'Chewy', system-ui, sans-serif;
  }

  .cosmic-row {
    display: inline-flex;
    align-items: center;
    gap: clamp(2px, 1vw, 8px);
    vertical-align: middle;
  }

  .whisker {
    position: relative;
    display: inline-flex;
    flex-direction: column;
    align-items: center;
    gap: clamp(8px, 1.8vw, 14px);
    margin: 0 clamp(3px, 1.2vw, 8px);
    vertical-align: middle;
    pointer-events: none;
    z-index: 2;
    transform: translateY(-6px);
  }

  .whisker span {
    display: block;
    width: clamp(36px, 18vw, 64px);
    height: clamp(4px, 1.5vw, 6px);
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
      gap: 2px;
    }

    .whisker {
      gap: 8px;
      margin: 0 4px;
    }

    .whisker span {
      width: clamp(24px, 16vw, 40px);
      height: clamp(3px, 1.5vw, 5px);
    }
  }

  .quiz-pill {
    position: relative;
    border: none;
    background: #f5d255;
    color: #25533F;
    --pill-mask: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 120 44' preserveAspectRatio='none'%3E%3Cpath fill='white' d='M8 7 C12 3 22 2 34 4 C48 1 72 1 86 4 C98 2 108 3 112 8 C117 12 118 19 116 24 C116 31 113 36 107 39 C101 42 92 42 84 40 C68 43 52 43 36 40 C27 43 18 42 12 38 C6 35 4 29 5 23 C3 18 4 12 8 7 Z'/%3E%3C/svg%3E");
    -webkit-mask-image: var(--pill-mask);
    mask-image: var(--pill-mask);
    -webkit-mask-size: 100% 100%;
    mask-size: 100% 100%;
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

  .intro-pill {
    margin-top: -20px;
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

  @media (max-width: 480px) {
    .intro-pill {
      margin-top: -12px;
    }
  }

  :global(.quiz-chewy),
  :global(.quiz-chewy *) {
    font-family: 'Chewy', 'Fredoka', system-ui, sans-serif;
  }

.splash-blob {
  position: absolute;
  top: -4rem;
  left: 50%;
  transform: translateX(-50%);
  padding: clamp(48px, 12vw, 80px) clamp(72px, 16vw, 120px);
  height: clamp(55%, 30vw, 75%);
  min-height: 325px;
    min-width: 575px;
    display: grid;
    place-items: center;
    background: #f5d255;
    --splash-mask: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 200 180'%3E%3Cpath fill='white' d='M100 56 L107.8 70.7 L125 60.6 L121.2 75.9 L143.3 73 L129 84.8 L150 90 L129 95.2 L143.3 107 L121.2 104.1 L125 119.4 L107.8 109.3 L100 124 L92.2 109.3 L75 119.4 L78.8 104.1 L56.7 107 L71 95.2 L50 90 L71 84.8 L56.7 73 L78.8 75.9 L75 60.6 L92.2 70.7 Z'/%3E%3C/svg%3E");
    -webkit-mask-image: var(--splash-mask);
    mask-image: var(--splash-mask);
    -webkit-mask-size: contain;
    mask-size: contain;
    -webkit-mask-repeat: no-repeat;
    mask-repeat: no-repeat;
    -webkit-mask-position: center;
    mask-position: center;
    font-family: 'Chewy', 'Fredoka', system-ui, sans-serif;
    font-size: clamp(26px, 6.5vw, 50px);
  letter-spacing: 0.04em;
  text-transform: uppercase;
  box-shadow: 0 14px 28px rgba(0, 0, 0, 0.2);
  text-align: center;
  line-height: 1.1;
}

@media (max-width: 640px) {
  .quiz-stage--splash {
    overflow: visible;
    height: auto;
    min-height: unset;
  }

  .splash-row {
    gap: 28px;
  }

  .splash-col {
    padding-top: 80px;
  }

  .splash-blob {
    top: -2rem;
    transform: translateX(-50%) scale(1.0);
    -webkit-mask-size: contain;
    mask-size: contain;
    min-width: 0;
    min-height: 280px;
    padding: 36px 54px;
    font-size: clamp(22px, 5.5vw, 32px);
    height: auto;
  }

  .splash-figure {
    margin-top: 10px;
    max-width: 160px;
  }
}

  .splash-col {
    position: relative;
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    padding: clamp(20px, 3vw, 36px) clamp(8px, 2vw, 12px);
    gap: clamp(12px, 2vw, 18px);
  }

  .splash-figure {
    max-width: 320px;
    width: 95%;
    margin-top: auto;
    padding-bottom: clamp(90px, 10vw, 104px);
  }

  .splash-img {
    width: 100%;
    height: clamp(240px, 32vw, 320px);
    display: block;
    object-fit: contain;
  }

  .text-splash-green {
    color: #afab23;
    text-shadow: 0 2px 6px rgba(0, 0, 0, 0.24);
  }
  .text-splash-blue {
    color: #b8cee8;
    text-shadow: 0 2px 6px rgba(0, 0, 0, 0.28);
  }
  .text-splash-orange {
    color: #ef6f3c;
    text-shadow: 0 2px 6px rgba(0, 0, 0, 0.32);
  }
</style>
