<script>
  import { resolve } from "$app/paths";
  import Seo from "#lib/Seo.svelte";
  import MailLink from "#lib/hoksi/MailLink.svelte";
</script>

<Seo
  title="Privacy | Alice Foronda"
  description="What happens to your information on catchupwithalice.com: no cookies or analytics, plus how the newsletter and Hoksi Haus banner requests handle your details."
  path="/privacy"
/>

<section class="page-shell py-12 md:py-16">
  <div class="privacy mx-auto max-w-2xl text-[#4c465f]">
    <h1 class="text-center text-4xl font-black lowercase tracking-[0.04em] text-[#352f47] md:text-5xl">
      privacy
    </h1>
    <p class="mt-2 text-center text-sm">Last updated October 26, 2026</p>

    <p class="mt-8">
      This page explains what happens to your information when you visit catchupwithalice.com,
      sign up for the newsletter or request a Hoksi Haus banner. The site is run by Alice
      Foronda.
    </p>

    <h2>browsing this site</h2>
    <ul>
      <li>The site sets no cookies and has no analytics, ads or tracking.</li>
      <li>Its fonts and images are served from the site itself, not from Google or other services.</li>
      <li>
        The site is hosted on GitHub Pages. GitHub logs every visitor's IP address for security
        purposes (see
        <a href="https://docs.github.com/en/pages/getting-started-with-github-pages/about-github-pages#data-collection" target="_blank" rel="noopener noreferrer">GitHub's
          note on GitHub Pages</a>).
      </li>
    </ul>

    <h2>crosby's cosmic dash</h2>
    <p>
      The game saves your best score in your browser's local storage. It stays on your device and
      is never sent anywhere. Sharing a score uses your device's own share menu, so it only goes
      where you choose to send it.
    </p>

    <h2>newsletter</h2>
    <p>
      On the home page, a sign-up window for Crosby's Crew opens after a few seconds or when you
      scroll down. It's provided by MailerLite, so your browser connects to MailerLite when the
      window appears. If you sign up, MailerLite stores your email address so Alice can send you
      the newsletter. Every email has an unsubscribe link. See
      <a href="https://www.mailerlite.com/legal/privacy-policy" target="_blank" rel="noopener noreferrer">MailerLite's
        privacy policy</a>.
    </p>

    <h2>hoksi haus banner requests</h2>
    <p>
      The <a href={resolve("/hoksi-haus")}>Hoksi Haus</a> order form is a Google Form, so Google
      stores your answers in the Hoksi Haus Google account under
      <a href="https://policies.google.com/privacy" target="_blank" rel="noopener noreferrer">Google's
        privacy policy</a>.
    </p>
    <ul>
      <li>
        <strong>What's collected:</strong> your name, email address, the date you need the banner,
        the occasion, the size, your design ideas and whether you need it within a week.
      </li>
      <li>
        <strong>Why:</strong> only to reply to your request, agree the design, price and date, and
        make your banner. It's never sold or used for marketing.
      </li>
      <li>
        <strong>How long it's kept:</strong> only as long as it's needed for your request and order,
        then it's deleted.
      </li>
      <li>
        <strong>Children under 13:</strong> please ask a parent or guardian to send the request for
        you.
      </li>
    </ul>

    <h2>your choices</h2>
    <p>
      You can ask to see, correct or delete the details you sent with a banner request by emailing
      Hoksi Haus at <MailLink />. To leave the newsletter, use the unsubscribe link in any email.
      If you live in the EU or UK, you can also complain to your local data protection authority.
    </p>

    <h2>other sites</h2>
    <p>
      Links to Amazon, Instagram, TikTok and Google Forms open those sites, and their own privacy
      policies apply there.
    </p>
  </div>
</section>

<style>
  .privacy h2 {
    margin-top: 2.25rem;
    font-size: 1.5rem;
    font-weight: 900;
    letter-spacing: 0.04em;
    text-transform: lowercase;
    color: #352f47;
  }

  .privacy p,
  .privacy ul {
    margin-top: 0.75rem;
    line-height: 1.65;
  }

  .privacy ul {
    list-style: disc;
    padding-left: 1.25rem;
  }

  .privacy li + li {
    margin-top: 0.5rem;
  }

  .privacy strong {
    color: #352f47;
  }

  .privacy :global(a) {
    font-weight: 600;
    color: #352f47;
    text-decoration: underline;
    text-underline-offset: 2px;
  }

  .privacy :global(a:hover) {
    color: #d2234c;
  }
</style>
