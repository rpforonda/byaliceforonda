<script>
  import { resolve } from "$app/paths";
  import BuyButtons from '$lib/components/BuyButtons.svelte';

  const bookImage = resolve("/CrosbyCover.PNG");
</script>

<section class="mx-auto max-w-3xl px-6 py-8 sm:py-12 md:py-16 text-center min-h-[calc(100vh-400px)]">
  <div class="relative mx-auto max-w-[280px] sm:max-w-sm rounded-3xl bg-[dodgerblue] p-3 shadow-xl mb-6 sm:mb-8">
    <img
      alt="Crosby's Cosmic Adventure book cover featuring a crocodile in space"
      src={bookImage}
      class="h-full w-full rounded-2xl object-cover"
      loading="lazy"
    />
  </div>

  <h1 class="text-3xl sm:text-4xl font-semibold tracking-tight text-[#1b1b1d]">The Book</h1>
  <p class="mt-3 sm:mt-4 text-base sm:text-lg leading-relaxed text-gray-700 px-4">
    Back-of-book blurb, formats, and print-on-demand note.
  </p>
  <div class="mt-6 sm:mt-8 flex justify-center">
    <BuyButtons amazonAsin="B0XXXXXXX" amazonTag="" ingramLink="" />
  </div>
</section>
