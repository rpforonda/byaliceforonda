# catchupwithalice.com

Official site for children's book author **Alice Foronda** (`byaliceforonda`) — a small,
fully static marketing site built with SvelteKit and deployed to GitHub Pages at
[catchupwithalice.com](https://catchupwithalice.com).

There is no backend: no server code, no database, no API, no runtime request handling. Every
page is prerendered to a plain HTML file at build time, and all dynamic behavior (menus,
animations, the newsletter popup) runs in the browser. Purchases and sign-ups happen on
third-party sites (Shopify, MailerLite), so the site itself collects no personal data.

> Looking for the visual language — colors, type, components, motion? See
> [`docs/DESIGN.md`](docs/DESIGN.md).

## Stack

| Concern        | Choice                                                            |
| -------------- | ----------------------------------------------------------------- |
| Framework      | [SvelteKit 2](https://kit.svelte.dev) with **Svelte 5 runes**     |
| Rendering      | `@sveltejs/adapter-static` — prerendered, client-side hydrated    |
| Styling        | [Tailwind CSS v4](https://tailwindcss.com) via `@tailwindcss/vite`|
| Build          | [Vite 6](https://vitejs.dev)                                      |
| Hosting        | GitHub Pages (custom domain via `static/CNAME`)                   |
| CI/CD          | GitHub Actions (`.github/workflows/deploy.yml`)                   |

`+layout.js` sets `ssr = false`, `csr = true`, `prerender = true`: pages are generated as
static HTML at build time and take over on the client after load.

## Getting started

```bash
npm install
npm run dev        # local dev server (Vite)
npm run build      # static build → build/
npm run preview    # serve the production build locally
```

`npm run build` also runs a `postbuild` step that copies `build/index.html` to
`build/404.html` so client-side routing survives a hard refresh on GitHub Pages.

Requires Node 20 (matching CI). A `.mise.toml` is present if you use [mise](https://mise.jdx.dev).

## Project structure

```
src/
  app.html              # document shell; loads Google Fonts, injects SvelteKit head/body
  app.css               # Tailwind import + base layer (focus styles, motion, .page-shell)
  routes/
    +layout.svelte      # header, nav, footer, mobile menu, View Transitions
    +layout.js          # prerender / csr / ssr flags
    +page.svelte        # home: hero, book promo, newsletter popup, characters
    about/+page.svelte  # author bio (image panels)
    book/+page.svelte   # book info + buy buttons → Shopify
    freebies/+page.svelte  # downloadable coloring-page PDFs
    shop/+page.svelte   # "coming soon" placeholder (not in nav)
static/                 # images, PDFs, robots.txt, CNAME, .nojekyll
.github/workflows/deploy.yml
```

Nav exposes **home / about / book / freebies**. `/shop` exists but is an unlinked
placeholder.

## Content & integrations

- **Characters:** Carla, Crosby, and Tank — featured on the home page.
- **Book purchases:** buttons on `/book` and the home page open the Shopify store
  (`catchupwithalice.myshopify.com`) in a new tab.
- **Newsletter:** "Join Crosby's Crew" — a MailerLite embedded form shown in a popup
  modal on the home page (`assets.mailerlite.com`, form id `QQryIS`).
- **Freebies:** three printable coloring-page PDFs (Crosby, Tank, Carla) served from `static/`.
- **Social:** Instagram `@byaliceforonda`, TikTok `@catchupwithalice` (footer).

## Deployment

Pushing to `main` triggers `.github/workflows/deploy.yml`, which runs `npm ci && npm run
build`, uploads `build/` as a Pages artifact, and deploys it. The workflow uses the official
GitHub Pages OIDC flow with least-privilege permissions (`pages: write`, `id-token: write`).

### GitHub Pages constraints

Because the host serves static files only, keep these in mind:

- **No custom HTTP headers** — no `Cache-Control`, no CSP or redirects at the server. Use
  `<meta>` / `<link rel="preload">` in `app.html` instead, and hashed filenames for cache-busting.
- **Custom domain** — `static/CNAME` (`catchupwithalice.com`) and `static/.nojekyll` must stay
  in `static/`. Leave `BASE_PATH` empty (custom domain serves from root).
- **Routing** — `/about` is served from `about.html`; unknown paths fall through to `404.html`.

## Security & privacy

This is a static site with a narrow attack surface. No user input reaches the DOM (no forms,
no `fetch`, no reading of URL params), no `{@html}`, and all external links carry
`rel="noopener noreferrer"`. The only embedded third party is the MailerLite signup iframe,
whose `src` is hardcoded. No analytics or trackers are bundled. The sole hardening gap is the
absence of a Content-Security-Policy `<meta>` tag (GitHub Pages can't set CSP headers).

## License

© Alice Foronda. All rights reserved. Site content and artwork are not licensed for reuse.
