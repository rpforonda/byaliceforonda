<script>
  import Quiz from '$lib/components/Quiz.svelte';

  let showIntro = $state(true);

  function startQuiz() {
    console.log('Take the Quiz clicked');
    showIntro = false;
    console.log('showIntro now', showIntro);
  }

  const stars = [
    { left: '8%', size: '32px', duration: '9s', delay: '0s', rotate: '-10deg' },
    { left: '18%', size: '26px', duration: '7s', delay: '2s', rotate: '12deg' },
    { left: '28%', size: '38px', duration: '8.5s', delay: '1s', rotate: '-6deg' },
    { left: '38%', size: '30px', duration: '7.5s', delay: '3s', rotate: '9deg' },
    { left: '48%', size: '40px', duration: '9.5s', delay: '0.5s', rotate: '-14deg' },
    { left: '58%', size: '26px', duration: '6.8s', delay: '2.5s', rotate: '16deg' },
    { left: '68%', size: '34px', duration: '8.2s', delay: '1.2s', rotate: '-4deg' },
    { left: '78%', size: '28px', duration: '7.3s', delay: '2.8s', rotate: '7deg' },
    { left: '88%', size: '36px', duration: '9.2s', delay: '1.7s', rotate: '-12deg' },
    { left: '96%', size: '24px', duration: '6.6s', delay: '0.8s', rotate: '5deg' }
  ];
</script>

<section class="mx-auto max-w-5xl px-6 py-16 text-center">
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
          <div class="blob-card w-full max-w-3xl px-12 py-12 text-[#7fa66d] sm:px-16 sm:py-16 min-h-[300px] sm:min-h-[360px]">
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
  @import url('https://fonts.googleapis.com/css2?family=Baloo+2:wght@600;700&display=swap');

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
    --blob-mask: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100' preserveAspectRatio='none'%3E%3Cpath fill='white' d='M26 10 C32 4 44 4 50 10 C56 4 68 4 74 10 C84 8 92 16 92 26 C98 30 100 40 96 48 C100 56 98 66 92 70 C92 80 84 88 74 86 C68 92 56 92 50 86 C44 92 32 92 26 86 C16 88 8 80 8 70 C2 66 0 56 4 48 C0 40 2 30 8 26 C8 16 16 8 26 10 Z'/%3E%3C/svg%3E");
    -webkit-mask-image: var(--blob-mask);
    mask-image: var(--blob-mask);
    -webkit-mask-size: cover;
    mask-size: cover;
    -webkit-mask-repeat: no-repeat;
    mask-repeat: no-repeat;
    box-shadow: 0 18px 36px rgba(0, 0, 0, 0.18), 0 0 0 6px rgba(12, 63, 48, 0.45);
    font-family: 'Baloo 2', 'Fredoka', system-ui, sans-serif;
  }

  .blob-heading {
    margin: 0;
    font-size: clamp(35px, 5vh, 80px);
    line-height: 1.08;
    font-weight: 700;
    letter-spacing: 0.02em;
    color: #6b8a32;
    max-width: 90%;
    margin-left: auto;
    margin-right: auto;
    text-align: center;
  }

  .quiz-pill {
    border: 3px solid #0c3f30;
    background: #f5d255;
    color: #6b8a32;
    border-radius: 999px;
    padding: 14px 32px;
    font-family: 'Baloo 2', 'Fredoka', system-ui, sans-serif;
    font-size: 17px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.04em;
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
