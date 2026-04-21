<script>
  import { resolve } from '$app/paths';

  const charimeImage = resolve('/charime.jpeg');
  const letterMeImage = resolve('/lettermetwo.png');
</script>

<section class="page-shell flex min-h-[35vh] items-center justify-center py-8 sm:min-h-[45vh] sm:py-12 md:min-h-[50vh] md:py-16">
  <img
    src={charimeImage}
    alt="Chari Me"
    class="w-full max-w-4xl h-auto object-contain"
    loading="lazy"
  />
</section>

<section class="page-shell flex min-h-[35vh] items-center justify-center pt-4 pb-8 sm:min-h-[45vh] sm:pt-6 sm:pb-12 md:min-h-[50vh] md:pt-8 md:pb-16">
  <img
    src={letterMeImage}
    alt="Letter Me"
    class="w-full max-w-4xl h-auto object-contain"
    loading="lazy"
  />
</section>

<style>
</style>
