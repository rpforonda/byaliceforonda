<script>
  import { resolve } from '$app/paths';
</script>

<section class="mx-auto max-w-5xl px-6 py-8 sm:py-12 md:py-16 min-h-[calc(100vh-400px)] text-left">
  <div class="grid gap-8 sm:gap-10 items-stretch lg:grid-cols-[1fr_1fr]">
    <div class="order-2 lg:order-2 bg-white p-6 sm:p-8 rounded-2xl shadow-sm">
      <div class="mt-4 space-y-1 text-base sm:text-lg leading-7 sm:leading-8 text-gray-700">
        <span class="text-8xl sm:text-9xl leading-none block text-[#db2108] text-left lowercase" style="font-family: 'New Icon Script', cursive;">hello,</span>
        <div style="font-family: 'Alice Regular', serif;">
          <span class="block">I'm Alice!</span>
          <span class="block">
            I'm a second-generation Korean American, born and raised in southern California. Recently, I learned just how short life can be through my mother's passing. As painful as this season has been, it's also been clarifying, nudging me toward the dreams I'd been quietly holding.
          </span>
          <span class="block">
            I've always loved words. They've brought me comfort and joy, but also sorrow and ache. That love, along with the lessons I've gathered as a daughter, mother, wife, student, and woman, has led me here, finally taking the plunge into writing.
          </span>
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
          <span class="block">
            As I learn more, see more, and do more, I invite you into a deeper love of words too.
          </span>
        </div>
      </div>
      <p class="mt-6 text-base sm:text-lg leading-relaxed text-gray-700" style="font-family: 'Alice Regular', serif;">
        All my love,<br />
        <span class="text-4xl sm:text-5xl text-[#db2108]" style="font-family: 'Great Vibes', cursive;">Alice</span>
      </p>
    </div>
    <div class="order-1 lg:order-1 flex justify-center lg:justify-start">
      <picture>
        <source srcset={resolve("/aboutMe.webp")} type="image/webp" />
        <img
          src={resolve("/aboutMe_opt.jpg")}
          alt="Alice Foronda"
          class="w-full max-w-[640px] h-[520px] sm:h-[620px] lg:h-full object-cover rounded-2xl shadow-lg"
        />
      </picture>
    </div>
  </div>
</section>

<style>
</style>
