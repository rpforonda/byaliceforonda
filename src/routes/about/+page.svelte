<script>
  import Seo from "#lib/Seo.svelte";
  import PageBackdrop from "#lib/PageBackdrop.svelte";
  // 1086px is all there is of this photo, so it's shown at that size at most (see `natural`),
  // and encoded at a higher quality so the build adds no artifacts of its own
  import authorPhoto from "#lib/assets/charime.jpeg?enhanced&w=1086;720&quality=85";
  import { SITE_URL, author } from "#lib/seo.js";
  // the letter's typefaces, self-hosted like the rest of the site and only loaded on this page
  import "@fontsource/lora/400.css";
  import "@fontsource/lora/400-italic.css";
  import "@fontsource/ms-madi/400.css";

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

<PageBackdrop src={authorPhoto} alt="Author Alice Foronda" natural />

<!-- relative, so it paints over the pinned backdrop; over-backdrop (app.css) starts it low so
     the photo shows first -->
<section class="over-backdrop relative flex min-h-[35vh] items-center justify-center pt-4 pb-8 sm:min-h-[45vh] sm:pt-6 sm:pb-12 md:min-h-[50vh] md:pt-8 md:pb-16">
  <div class="page-shell flex justify-center">
    <!-- Alice's hello letter, as real text so it stays sharp and is easy to edit -->
    <article class="letter" aria-labelledby="letter-hello">
      <h2 id="letter-hello" class="letter-script letter-hello">Hello.</h2>
      <p>
        I’m Alice.<br />
        I’m a second-generation Korean American author who was born and raised in Southern
        California.
      </p>
      <p>
        I have always loved words, but never had the courage to write them down until my mother’s
        sudden passing. That single experience changed my world. While the journey has been heavy,
        it has also been incredibly inspiring. This experience has provided me with the opportunity
        to take a chance and to put my words and stories out there to share them with others like
        you.
      </p>
      <p>
        I wrote <cite>Crosby’s Cosmic Adventure</cite> starting with the last page. Here, he talks
        about how he now sees his weaknesses, as strengths. He talks about his mindset
        transformation, and how that has helped him become a better version of himself. I created
        Crosby to connect with those who are sometimes unsure of themselves, who, at times, have a
        hard time believing in themselves, and for those who sometimes feel left out. I see so much
        of myself in Crosby, and he has brought me a comfort I wish I had as a child.
      </p>
      <p>
        I hope to write more stories with characters we can all see ourselves in, as a reminder
        that experiencing life and all its obstacles is a universal struggle, and that you are not
        alone.
      </p>
      <p>
        I currently live in Baden-Württemberg, Germany with my wonderful family. When I’m not
        writing stories, you can find me playing volleyball, practicing my German, or making epic
        breakfast sandwiches.
      </p>
      <p>Thank you for being here.</p>
      <p>
        Sincerely,<br />
        <span class="letter-script letter-signature">Alice J. Foronda</span>
      </p>
    </article>
  </div>
</section>

<style>
  /* the cream paper and deep red ink of the original letter artwork */
  .letter {
    width: 100%;
    max-width: 40rem;
    padding: 2rem 1.5rem 2.25rem;
    background-color: #f2f1e9;
    color: #912d2f;
    font-family: "Lora", Georgia, serif;
    font-size: 1.0625rem;
    line-height: 1.55;
    box-shadow: 0 18px 40px rgba(0, 0, 0, 0.2);
  }

  @media (min-width: 640px) {
    .letter {
      padding: 2.5rem 2.25rem 2.75rem;
      font-size: 1.125rem;
    }
  }

  /* The text grows smoothly from 1.0625rem on a phone (375px) to 1.125rem at 1100px. Browsers
     without progress() drop this and keep the two sizes above. */
  .letter {
    font-size: calc(1.0625rem + 0.0625rem * progress(100vw, 375px, 1100px));
  }

  /* one blank line between paragraphs, like the original letter (1.55em is the same at this
     line height, for browsers without lh) */
  .letter p + p {
    margin-top: 1.55em;
    margin-top: 1lh;
  }

  .letter-script {
    font-family: "Ms Madi", cursive;
    font-weight: 400;
    line-height: 1;
  }

  .letter-hello {
    margin-bottom: 1.5rem;
    text-align: center;
    font-size: 3.5rem;
  }

  .letter-signature {
    display: inline-block;
    margin-top: 0.5rem;
    font-size: 2.75rem;
  }
</style>
