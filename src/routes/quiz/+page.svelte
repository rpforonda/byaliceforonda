<script>
  import Quiz from '$lib/components/Quiz.svelte';

  let showIntro = $state(true);

  function startQuiz() {
    console.log('Take the Quiz clicked');
    showIntro = false;
    console.log('showIntro now', showIntro);
  }

  const stars = [
    { left: '8%', size: '40px', duration: '9s', delay: '0s', rotate: '-10deg' },
    { left: '18%', size: '32px', duration: '7s', delay: '2s', rotate: '12deg' },
    { left: '28%', size: '48px', duration: '8.5s', delay: '1s', rotate: '-6deg' },
    { left: '38%', size: '38px', duration: '7.5s', delay: '3s', rotate: '9deg' },
    { left: '48%', size: '52px', duration: '9.5s', delay: '0.5s', rotate: '-14deg' },
    { left: '58%', size: '32px', duration: '6.8s', delay: '2.5s', rotate: '16deg' },
    { left: '68%', size: '42px', duration: '8.2s', delay: '1.2s', rotate: '-4deg' },
    { left: '78%', size: '36px', duration: '7.3s', delay: '2.8s', rotate: '7deg' },
    { left: '88%', size: '46px', duration: '9.2s', delay: '1.7s', rotate: '-12deg' },
    { left: '96%', size: '32px', duration: '6.6s', delay: '0.8s', rotate: '5deg' }
  ];
</script>

<section class="max-w-5xl px-6 py-16 text-center">
  <h1 class="text-4xl font-semibold tracking-tight text-[#1b1b1d]">Which character are you?</h1>
  <p class="mt-4 text-lg leading-relaxed text-gray-700">
    Answer a few questions to reveal your character match, then share your result
  </p>

  <div class="mt-10 flex w-full justify-center">
    {#if showIntro}
      <div
        class="relative isolate w-full max-w-5xl overflow-hidden rounded-[34px] bg-[#0c3c2f] p-8 shadow-[0_18px_48px_rgba(0,0,0,0.26)] sm:p-12"
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
          <div class="blob-card w-full max-w-3xl px-10 py-12 text-[#7fa66d] sm:px-14 sm:py-16 min-h-[340px] sm:min-h-[400px] flex items-center justify-center">
            <p class="blob-heading">
              WHICH COSMIC CHARACTER ARE YOU?
            </p>
          </div>

          <button class="quiz-pill" type="button" on:click={startQuiz}>
            TAKE THE QUIZ
          </button>
        </div>
      </div>
    {:else}
      <div class="inline-block w-full text-left">
        <Quiz />
      </div>
    {/if}
  </div>
</section>

<style>
  @import url('https://fonts.googleapis.com/css2?family=Schoolbell&display=swap');

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
    filter: drop-shadow(0 6px 14px rgba(255, 164, 196, 0.5));
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
    background: #f5d255;
    --blob-mask: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100' preserveAspectRatio='none'%3E%3Cpath fill='white' d='M26 24 C34 14 50 16 58 22 C72 16 88 20 92 34 C102 42 102 58 92 66 C86 80 74 86 60 82 C50 90 34 88 26 80 C14 82 8 70 10 60 C2 52 4 38 12 32 C12 26 18 20 26 24 Z'/%3E%3C/svg%3E");
    -webkit-mask-image: var(--blob-mask);
    mask-image: var(--blob-mask);
    -webkit-mask-size: cover;
    mask-size: cover;
    -webkit-mask-repeat: no-repeat;
    mask-repeat: no-repeat;
    box-shadow: 0 18px 36px rgba(0, 0, 0, 0.18), 0 0 0 6px rgba(12, 63, 48, 0.45);
    font-family: 'Schoolbell', 'Fredoka', system-ui, sans-serif;
    transform: rotate(-3deg);
    transform-origin: center;
  }

  .blob-heading {
    margin: 0;
    font-size: clamp(38px, 10vw, 58px);
    line-height: 1.05;
    font-weight: 600;
    letter-spacing: 0.025em;
    color: #6b8a32;
    max-width: 92%;
    margin-left: auto;
    margin-right: auto;
    text-align: center;
    text-wrap: balance;
  }

  .quiz-pill {
    position: relative;
    border: none;
    background: #f5d255;
    color: #6b8a32;
    --pill-mask: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 40' preserveAspectRatio='none'%3E%3Cpath fill='white' d='M20 2 C34 -2 62 0 78 4 C92 6 100 14 98 24 C96 34 86 40 74 38 C60 44 40 42 28 38 C16 40 6 34 4 24 C2 12 8 4 20 2 Z'/%3E%3C/svg%3E");
    -webkit-mask-image: var(--pill-mask);
    mask-image: var(--pill-mask);
    -webkit-mask-size: cover;
    mask-size: cover;
    -webkit-mask-repeat: no-repeat;
    mask-repeat: no-repeat;
    padding: 16px 34px;
    font-family: 'Schoolbell', 'Fredoka', system-ui, sans-serif;
    font-size: 18px;
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
</style>
