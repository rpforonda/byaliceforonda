<script>
  import Seo from "#lib/Seo.svelte";
  import { SITE_URL, author } from "#lib/seo.js";

  const jsonLd = {
    "@context": "https://schema.org",
    "@type": "ProfilePage",
    url: `${SITE_URL}/about`,
    mainEntity: author,
  };
</script>

<Seo
  title="About Alice Foronda — Children's Book Author"
  description="Alice J. Foronda is a Korean American children's author from Southern California, now living in Germany, and the creator of Crosby the crocodile."
  path="/about"
  {jsonLd}
/>

<h1 class="sr-only">About Alice Foronda</h1>

<section class="page-shell flex min-h-[35vh] items-center justify-center py-8 sm:min-h-[45vh] sm:py-12 md:min-h-[50vh] md:py-16">
  <enhanced:img
    src="#lib/assets/charime.jpeg"
    alt="Author Alice Foronda"
    sizes="(min-width: 960px) 896px, 100vw"
    class="w-full max-w-4xl h-auto object-contain"
    fetchpriority="high"
  />
</section>

<section class="page-shell flex min-h-[35vh] items-center justify-center pt-4 pb-8 sm:min-h-[45vh] sm:pt-6 sm:pb-12 md:min-h-[50vh] md:pt-8 md:pb-16">
  <enhanced:img
    src="#lib/assets/lettermetwo.png?w=616"
    sizes="616px"
    alt="A hello letter from Alice"
    aria-describedby="alice-letter"
    class="w-full max-w-4xl h-auto object-contain"
    loading="lazy"
  />
  <!-- The letter is artwork; this is its text for screen readers and search engines. -->
  <div id="alice-letter" class="sr-only">
    <h2>Hello.</h2>
    <p>
      I'm Alice. I'm a second-generation Korean American author who was born and raised in
      Southern California.
    </p>
    <p>
      I have always loved words, but never had the courage to write them down until my mother's
      sudden passing. That single experience changed my world. While the journey has been heavy,
      it has also been incredibly inspiring. This experience has provided me with the opportunity
      to take a chance and to put my words and stories out there to share them with others like
      you.
    </p>
    <p>
      I wrote Crosby's Cosmic Adventure starting with the last page. Here, he talks about how he
      now sees his weaknesses, as strengths. He talks about his mindset transformation, and how
      that has helped him become a better version of himself. I created Crosby to connect with
      those who are sometimes unsure of themselves, who, at times, have a hard time believing in
      themselves, and for those who sometimes feel left out. I see so much of myself in Crosby,
      and he has brought me a comfort I wish I had as a child.
    </p>
    <p>
      I hope to write more stories with characters we can all see ourselves in, as a reminder
      that experiencing life and all its obstacles is a universal struggle, and that you are not
      alone.
    </p>
    <p>
      I currently live in Baden-Württemberg, Germany with my wonderful family. When I'm not
      writing stories, you can find me playing volleyball, practicing my German, or making epic
      breakfast sandwiches.
    </p>
    <p>Thank you for being here.</p>
    <p>Sincerely, Alice J. Foronda</p>
  </div>
</section>
