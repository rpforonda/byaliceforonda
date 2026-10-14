# catchupwithalice.com

Official site for children's book author **Alice Foronda** (`byaliceforonda`) — a small,
fully static marketing site built with SvelteKit and deployed to GitHub Pages at
[catchupwithalice.com](https://catchupwithalice.com).

There is no backend: no server code, no database, no API, no runtime request handling. Every
route gets its own HTML file at build time, and everything else (page content, menus,
animations, the newsletter popup) is rendered in the browser. Purchases and sign-ups happen on
third-party sites (Shopify, MailerLite), so the site itself collects no personal data.

> Looking for the visual language — colors, type, components, motion? See
> [`docs/DESIGN.md`](docs/DESIGN.md).

## Stack

| Concern        | Choice                                                            |
| -------------- | ----------------------------------------------------------------- |
| Framework      | [SvelteKit 3](https://svelte.dev/docs/kit) with **Svelte 5 runes** |
| Rendering      | `@sveltejs/adapter-static` — every route prerendered to full HTML at build time, then hydrated |
| Styling        | [Tailwind CSS v4](https://tailwindcss.com) via `@tailwindcss/vite`|
| Images         | [`@sveltejs/enhanced-img`](https://svelte.dev/docs/kit/images) — AVIF/WebP + `srcset` at build time |
| Build          | [Vite 8](https://vite.dev)                                        |
| Hosting        | GitHub Pages (custom domain via `static/CNAME`)                   |
| CI/CD          | GitHub Actions (`.github/workflows/deploy.yml`)                   |

`+layout.js` sets `prerender = true`, `ssr = true`, `csr = true`: each page is rendered to
complete HTML at build time (content, headings, images, `<title>`, meta tags), then hydrated
in the browser. There's still no server — `ssr` only means "render at build time" here. Keep
browser-only code (`window`, `document`, `localStorage`) inside `onMount` / `$effect` /
event handlers, or the build will fail.

SvelteKit 3 has no `svelte.config.js`: the SvelteKit options (adapter, `paths`, `prerender`,
compiler options) are passed to the `sveltekit()` plugin in `vite.config.ts`. Imports from
`src/lib` use the `#lib/...` subpath import defined in `package.json` (SvelteKit 3 removed
`$lib`).

## Getting started

```bash
npm install
npm run dev        # local dev server (Vite)
npm run build      # static build → build/
npm run preview    # serve the production build locally
```

The adapter writes an SPA fallback to `build/404.html`, so GitHub Pages serves it for unknown
paths and the client router takes over.

Requires **Node 22.17 or newer** (SvelteKit 3's minimum); CI builds with Node 24.

If your system has libvips installed (e.g. Arch Linux), `sharp` (used by enhanced-img) tries
to compile against it and `npm install` fails with *"Please add node-addon-api to your
dependencies"*. Use its prebuilt binary instead:

```bash
SHARP_IGNORE_GLOBAL_LIBVIPS=1 npm ci
```

## Project structure

```
vite.config.ts          # Vite plugins + SvelteKit options (adapter-static, paths, prerender)
src/
  app.html              # document shell; loads Nunito + Poppins, injects SvelteKit head/body
  app.css               # Tailwind import + base layer (focus styles, motion, .page-shell)
  lib/
    CosmicDash.svelte   # "Crosby's Cosmic Dash" — canvas endless-runner game
    Seo.svelte          # per-page <head>: title, description, canonical, Open Graph, JSON-LD
    seo.js              # site URL, share image, author + book structured data
    assets/             # all images the pages use (processed at build time)
      game/             # Cosmic Dash character sprites
  routes/
    +layout.svelte      # header, nav, footer, mobile menu, View Transitions
    +layout.js          # prerender / csr / ssr flags
    +page.svelte        # home: hero, book promo, newsletter popup, characters
    about/+page.svelte  # author bio (image panels)
    book/+page.svelte   # book info + buy buttons → Shopify
    freebies/+page.svelte  # coloring-page PDFs + Cosmic Dash game
    shop/+page.svelte   # "coming soon" placeholder (not in nav, noindex)
    sitemap.xml/+server.js  # prerendered sitemap of the public pages
static/                 # served as-is: coloring-page PDFs, favicons, og-image.jpg, robots.txt, CNAME, .nojekyll
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
- **Freebies:** three printable coloring-page PDFs (Crosby, Tank, Carla) served from `static/`
  and linked with `asset()`, plus **Crosby's Cosmic Dash**, a browser game (see below).
- **Social:** Instagram `@byaliceforonda`, TikTok `@catchupwithalice` (footer).

### Crosby's Cosmic Dash

A self-contained `<canvas>` endless-runner (Chrome-dino style) on the freebies page,
implemented in `src/lib/CosmicDash.svelte`. Pick a character (Crosby, Carla, or Tank), run
across a moon surface in deep space, jump moon rocks and space crystals, duck (or jump)
shooting comets, and collect stars. The nebula colors change every 500 points (Purple Nebula →
Teal Galaxy → Rosy Starfield → Golden Comet Belt).

- **Characters & skills:** each friend has a passive perk and a special move charged by
  collecting stars. Crosby, the main hero, is deliberately the strongest.

  | Character | Passive | Special (stars to charge) |
  | --------- | ------- | ------------------------- |
  | Crosby | Starlight Double Jump — jump again mid-air | Brave Heart (6) — 5s invincible: smash obstacles, star magnet, double score |
  | Carla | Navigator's Map — off-screen obstacle warnings + star magnet | Calm Compass (9) — slows the world for 4s |
  | Tank | Spring Gadget — higher held jump | Rocket Pack (9) — fly safely over everything for 3s |

- **Sprites:** colored versions of the pencil character sketches, painted in the book-cover
  palette, in `src/lib/assets/game/` (imported, so Vite fingerprints them).
- **Controls:** Space / ↑ / W to jump (hold for a higher jump), ↓ / S to duck, X / E / Shift
  for the special move, P / Esc to pause. On touch: tap to jump, swipe down to duck, and the
  ★ button under the canvas for the special.
- **Tech:** Canvas 2D + `requestAnimationFrame`; DPR-aware sizing via `ResizeObserver`;
  auto-pauses on tab-hide (`visibilitychange`) and on blur.
- **Persistence:** high score only, in `localStorage` (`catchupwithalice-cosmic-dash-hi`),
  guarded by try/catch. No network, no personal data.
- **Accessibility:** `role="application"` with descriptive label, keyboard-operable, an
  `aria-live` region announcing game-over, and a visible focus outline.

## Images

Every image a page shows lives in `src/lib/assets/` and is rendered with `<enhanced:img>`,
which generates AVIF + WebP (plus a PNG/JPEG fallback) at build time and adds `width`/`height`
to prevent layout shift:

```svelte
<enhanced:img src="#lib/assets/cosmic.jpeg" alt="…" width="1280" class="…" />
```

Pick the sizing that matches how the image is displayed:

- **Source is at least 2× its displayed size:** set `width` to 2× the largest CSS width (e.g.
  `width="160"` for an 80px logo). You get a 1× and a 2× version.
- **Image spans the layout responsively:** set `sizes` (e.g.
  `sizes="(min-width: 1280px) 1100px, 100vw"`). You get a range of widths for the browser to
  choose from.
- **Source is smaller than 2× its display size:** use one width at the source size:
  `src="#lib/assets/joincrosby.png?w=463"` **and** `sizes="463px"`. Without `sizes`, the
  browser assumes `100vw` and the image renders larger than intended.

Mark the largest above-the-fold image `fetchpriority="high"` and leave out `loading="lazy"`
on anything visible without scrolling. Assets used from script (like the game sprites) are
imported normally (`import sprite from "#lib/assets/game/crosby.webp"`) so Vite fingerprints
them and a missing file fails the build.

`<enhanced:img>` wraps each `<img>` in a `<picture>`. `app.css` sets `picture { display:
contents }` and hides its `<source>` children so layouts behave as if the `<img>` were
unwrapped.

Only put files in `static/` when they need a stable, unhashed URL (PDFs, favicons, CNAME).
GitHub Pages can't redirect, so renaming or removing anything there breaks existing links.

## Deployment

Pushing to `main` triggers `.github/workflows/deploy.yml`, which runs `npm ci && npm run
build`, uploads `build/` as a Pages artifact, and deploys it. The workflow uses the official
GitHub Pages OIDC flow; the `pages: write` / `id-token: write` permissions are scoped to the
deploy job.

Pull requests into `main` run the same build **without deploying** (the build job checks the
PR merged with the current `main`), so a broken build shows up on the PR instead of in
production.

### GitHub Pages constraints

Because the host serves static files only, keep these in mind:

- **No custom HTTP headers** — no `Cache-Control`, no CSP or redirects at the server. Use
  `<meta>` / `<link rel="preload">` in `app.html` instead, and hashed filenames for cache-busting.
- **Custom domain** — `static/CNAME` (`catchupwithalice.com`) and `static/.nojekyll` must stay
  in `static/`. Leave `BASE_PATH` empty (custom domain serves from root).
- **Routing** — `/about` is served from `about.html`; unknown paths fall through to `404.html`.

### SEO

- Every page renders a `<Seo>` component (`src/lib/Seo.svelte`) with its own title,
  description and canonical URL, plus Open Graph / Twitter tags pointing at
  `static/og-image.jpg` (1200×630, from the book's starry-night spread). Change site-wide
  values (URL, share image, author/book details) in `src/lib/seo.js`.
- Structured data (JSON-LD): `WebSite` + `Person` + `Book` on the home page, `Book` on `/book`,
  `ProfilePage` on `/about`.
- `/about` and `/book` keep their text in artwork, so each image has a screen-reader-only
  transcript (linked with `aria-describedby`) that crawlers can read. Update the transcript
  if the artwork changes.
- `sitemap.xml` is generated at build time and referenced from `static/robots.txt`. Add new
  public pages to the list in `src/routes/sitemap.xml/+server.js`.
- `/shop` is `noindex` until it has real content.

## Security & privacy

This is a static site with a narrow attack surface. No user input reaches the DOM (no forms,
no `fetch`, no reading of URL params), no `{@html}`, and all external links carry
`rel="noopener noreferrer"`. The only embedded third party is the MailerLite signup iframe,
whose `src` is hardcoded. No analytics or trackers are bundled. The sole hardening gap is the
absence of a Content-Security-Policy `<meta>` tag (GitHub Pages can't set CSP headers).

## License

© Alice Foronda. All rights reserved. Site content and artwork are not licensed for reuse.
