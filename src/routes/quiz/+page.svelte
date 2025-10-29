<script>
  import Quiz from '$lib/components/Quiz.svelte';
</script>

<section class="mx-auto max-w-3xl px-6 py-16 text-center">
  <h1 class="text-4xl font-semibold tracking-tight text-[#1b1b1d]">Which character are you?</h1>
  <p class="mt-4 text-lg leading-relaxed text-gray-700">
    Answer a few questions to reveal your character match, then share your result
  </p>
  <div class="mt-8 inline-block text-left">
    <Quiz />
  </div>
</section>
