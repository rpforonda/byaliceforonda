<script>
  import { resolve } from '$app/paths';
</script>

<section class="mx-auto max-w-5xl px-6 py-8 sm:py-12 md:py-16 min-h-[calc(100vh-400px)]">
  <div class="grid gap-8 sm:gap-10 items-start lg:grid-cols-[1.2fr_0.8fr]">
    <div>
      <div class="mt-4 space-y-2 text-base sm:text-lg leading-7 sm:leading-8 text-gray-700">
        <span class="font-[Borel] text-2xl sm:text-3xl leading-tight block text-[#db2108]">Hello,</span><span class="block">I'm Alice!</span>
        <span class="block">I live in Baden-Württemberg, Germany with my husband and three children.</span>
        <span class="block">
          I love to write stories inspired by my own experiences and imagination.
        </span>
        <span class="block">
          When I'm not writing, I study German and play volleyball.
        </span>
        <span class="block">
          My debut children's book, Crosby's Cosmic Adventure, was inspired by my own struggle with belonging and self-acceptance.
          I hope you are able to see yourself and connect with the stories and characters I write about.
        </span>
      </div>
      <p class="mt-6 text-base sm:text-lg leading-relaxed text-gray-700">
        All my love,<br />
        <span class="font-[Corinthia] text-4xl sm:text-5xl text-[#db2108]">Alice</span>
      </p>
    </div>
    <div class="flex justify-end lg:justify-center">
      <picture>
        <source srcset={resolve("/aboutMe.webp")} type="image/webp" />
        <img
          src={resolve("/aboutMe_opt.jpg")}
          alt="Alice Foronda"
          class="w-full max-w-[280px] sm:max-w-xs rounded-2xl shadow-lg"
        />
      </picture>
    </div>
  </div>
</section>

<style>
</style>
