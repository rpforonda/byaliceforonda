<script>
  import Quiz from '$lib/components/Quiz.svelte';

  let showIntro = true;

  const stars = [
    { left: '8%', size: '22px', duration: '9s', delay: '0s', rotate: '-10deg' },
    { left: '18%', size: '18px', duration: '7s', delay: '2s', rotate: '12deg' },
    { left: '28%', size: '26px', duration: '8.5s', delay: '1s', rotate: '-6deg' },
    { left: '38%', size: '20px', duration: '7.5s', delay: '3s', rotate: '9deg' },
    { left: '48%', size: '28px', duration: '9.5s', delay: '0.5s', rotate: '-14deg' },
    { left: '58%', size: '18px', duration: '6.8s', delay: '2.5s', rotate: '16deg' },
    { left: '68%', size: '23px', duration: '8.2s', delay: '1.2s', rotate: '-4deg' },
    { left: '78%', size: '19px', duration: '7.3s', delay: '2.8s', rotate: '7deg' },
    { left: '88%', size: '25px', duration: '9.2s', delay: '1.7s', rotate: '-12deg' },
    { left: '96%', size: '17px', duration: '6.6s', delay: '0.8s', rotate: '5deg' }
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
        class="relative isolate w-full max-w-5xl overflow-hidden rounded-[34px] bg-[#c9f7f1] p-8 shadow-[0_18px_48px_rgba(0,0,0,0.16)] sm:p-12"
        role="dialog"
        aria-modal="true"
        aria-label="Start the cosmic character quiz"
      >
        <div class="pointer-events-none absolute inset-0 opacity-90">
          <div class="absolute inset-0 bg-[radial-gradient(circle_at_18%_18%,#ddfff6_0%,transparent_34%),radial-gradient(circle_at_84%_12%,#bff4e5_0%,transparent_36%)]"></div>
          {#each stars as star, idx}
            <span
              class="falling-star"
              style={`left:${star.left}; --size:${star.size}; animation-duration:${star.duration}; animation-delay:${star.delay}; --rotate:${star.rotate};`}
              aria-hidden="true"
            >
              ✶
            </span>
          {/each}
        </div>

        <div class="relative z-10 flex flex-col items-center gap-8 text-center">
          <div
            class="blob-card w-full max-w-2xl px-10 py-10 text-[#7fa66d] sm:px-12 sm:py-12"
            aria-hidden="true"
          >
            <p class="text-2xl font-bold leading-tight sm:text-3xl">
              WHICH COSMIC CHARACTER ARE YOU?
            </p>
          </div>

          <button
            class="rounded-full border border-[#e7d37a] bg-[#f6e48d] px-8 py-3 text-sm font-semibold uppercase tracking-[0.2em] text-[#7fa66d] shadow-[0_12px_28px_rgba(0,0,0,0.12)] transition hover:-translate-y-[2px] hover:shadow-[0_16px_34px_rgba(0,0,0,0.16)]"
            type="button"
            on:click={() => (showIntro = false)}
          >
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
  .falling-star {
    position: absolute;
    top: -12%;
    color: #ffc2d4;
    font-size: var(--size, 20px);
    animation: fall linear infinite;
    opacity: 0.85;
    text-shadow: 0 6px 14px rgba(255, 164, 196, 0.5);
  }

  @keyframes fall {
    0% {
      transform: translateY(0) rotate(var(--rotate, 0deg));
      opacity: 0;
    }
    10% {
      opacity: 1;
    }
    100% {
      transform: translateY(120%) rotate(calc(var(--rotate, 0deg) + 40deg));
      opacity: 0;
    }
  }

  .blob-card {
    background: radial-gradient(circle at 18% 24%, #ffefb2, #f4dc80 72%);
    border-radius: 48% 52% 44% 56% / 56% 46% 58% 42%;
    box-shadow: 0 18px 36px rgba(0, 0, 0, 0.12), 0 0 0 1px rgba(255, 236, 158, 0.5);
  }
</style>
