<script>
  import { resolve } from "$app/paths";
  import BuyButtons from '$lib/components/BuyButtons.svelte';

  const bookImage = resolve("/CrosbyCover.PNG");
  const crosbyInset = resolve("/Crosby.png");
</script>

<section
  class="w-full px-6 py-8 sm:py-12 md:py-16 text-center min-h-[calc(100vh-400px)]"
>
  <div class="mx-auto max-w-6xl">
    <div class="grid gap-6 md:grid-cols-[minmax(220px,360px)_1fr] md:gap-8 items-stretch">
      <div class="relative mr-auto ml-0 max-w-[280px] sm:max-w-sm shadow-xl mb-2 sm:mb-2 h-full">
        <img
          alt="Crosby's Cosmic Adventure book cover featuring a crocodile in space"
          src={bookImage}
          class="h-full w-full object-contain md:min-h-full"
          loading="lazy"
        />
      </div>

      <div class="book-copy mx-auto w-full max-w-2xl px-4 text-left">
        <p class="text-base sm:text-lg leading-relaxed text-gray-700">
          <img
            src={crosbyInset}
            alt="Crosby character illustration"
            class="float-right w-48 sm:w-56 h-auto ml-4 mt-10 mb-2"
            style="transform: scaleX(-1);"
            loading="lazy"
          />
          "My tail's too thick.
          <br />
          My claws too long.
          <br />
          Don't you think I'll do this all wrong?"
          <br /><br />
          Have you ever felt a little nervous about trying something new?
          <br />
          Crosby the crocodile feels the same way!
          <br />
          But with a little courage and some help from his friends, he discovers just
          how brightly he can shine.
          <br /><br />
          <span class="inline-flex items-center gap-2">
            <span class="text-yellow-500 text-sm" aria-hidden="true">★</span>
            <span>Perfect for ages 4-8</span>
            <span class="text-yellow-500 text-sm" aria-hidden="true">★</span>
          </span>
          <br />
          Join Crosby and his crew on a starry adventure full of friendship fun, and
          learning to love who you are.
        </p>
        <div class="clear-both"></div>
      </div>
    </div>
    <div class="mt-6 sm:mt-8 flex justify-center">
      <BuyButtons amazonAsin="B0XXXXXXX" amazonTag="" ingramLink="" />
    </div>
  </div>

</section>

<style>
  .book-copy {
    padding: clamp(20px, 3vw, 30px);
  }

  @media (max-width: 640px) {
    .book-copy img {
      float: none !important;
      display: block;
      margin: 0 auto 12px !important;
      width: 140px !important;
    }
  }
</style>
