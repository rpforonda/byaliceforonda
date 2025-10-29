<script>
  import BuyButtons from '$lib/components/BuyButtons.svelte';
  import Quiz from '$lib/components/Quiz.svelte';
</script>

<section class="mx-auto max-w-3xl px-6 py-16 text-center">
  <h1 class="text-4xl font-semibold tracking-tight text-[#1b1b1d]">The Book</h1>
  <p class="mt-4 text-lg leading-relaxed text-gray-700">
    Back-of-book blurb, formats, and print-on-demand note.
  </p>
  <div class="mt-8 flex justify-center">
    <BuyButtons amazonAsin="B0XXXXXXX" amazonTag="" ingramLink="" />
  </div>
</section>
