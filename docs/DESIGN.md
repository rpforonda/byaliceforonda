# Design documentation — catchupwithalice.com

The visual language for Alice Foronda's author site. The tone is **warm, playful, and
child-friendly** — soft pastel-blue backgrounds, a confident rose-red accent, rounded shapes,
gentle motion, and lowercase navigation. This document records the design as it is actually
built, so it can be reproduced and extended consistently.

> For stack, structure, and how to run the site, see the [README](../README.md).

---

## 1. Color

Colors are applied as **inline hex values and Tailwind utilities** directly in the markup
(e.g. `bg-[#d7e6ff]`, `text-[#352f47]`, `rose-600`). A token palette also exists in
`tailwind.config.js`, but **Tailwind v4 (via `@tailwindcss/vite`) does not read that config
file** unless referenced with `@config`, so those tokens are currently aspirational
reference, not live utilities. Treat the values below as the source of truth.

### Core palette (in use)

| Role                        | Value       | Where                                               |
| --------------------------- | ----------- | --------------------------------------------------- |
| **Page background (sky)**   | `#d7e6ff`   | App shell, home sections, footer top band           |
| **Footer band**             | `#c4d7f6`   | Footer lower band + logo disc                       |
| **Ink / dark purple**       | `#352f47`   | Secondary nav bar, headings                         |
| **Body text (muted)**       | `#4c465f`   | Paragraph copy                                      |
| **Primary / rose-red**      | `#d2234c`   | Focus outlines, primary brand accent                |
| **Button red**              | `rose-600`  | CTA / menu buttons (Tailwind `rose-600`/`700` hover)|
| **Characters section**      | `blue-900`  | Full-bleed dark section behind character cards      |
| **Footer base**             | `stone-800` | Wordmark + social strip background                  |
| **Surface white**           | `#ffffff`   | Header bar, menu panel, cards                       |

### Brand reference tokens (`tailwind.config.js`, not currently wired up)

Kept as the intended broader palette should the project move tokens into an `@theme` block:

- **primary** — `#d2234c` · dark `#b31d41` · light `#e8365e` · red `#db2108`
- **accent** — gold `#ffd28d` · blue `#8fd3ff` · pink `#ffb4c6` · light-blue `#c7e9ff` ·
  orange `#f09a58` · green `#7fa66d` · dark-green `#25533F` · olive `#6b8a32`
- **text** — dark `#1b1b1d` · dark-blue `#0b1a2d` · navy `#052238`
- **bg** — pink-light `#ffe0ec` · pink-soft `#ffe8f0` · blue-soft `#e6f0ff`

> **Recommendation:** promote the in-use values into an `@theme` block in `app.css` so color
> is defined once and referenced as utilities, and retire the dead config file. Until then,
> new work should reuse the hex values in the *Core palette* table rather than invent shades.

---

## 2. Typography

**Nunito** is the primary typeface, applied at the shell via inline
`font-family: 'Nunito', sans-serif`. Headings lean heavy and lowercase for a playful,
approachable feel (`font-black`, `lowercase`, slight letter-spacing e.g. `tracking-[0.04em]`).

- **Body & UI:** Nunito (400 / 600 / 700)
- **Secondary:** Poppins 400 (available, used sparingly)

Fonts are loaded from **Google Fonts** in `app.html`. The document currently requests a large
set of display families (Chewy, DynaPuff, Great Vibes, Grandstander, and others) plus Russo
One, but the rendered site primarily uses **Nunito** and **Poppins**. Three unused helper
classes (`.font-grandstander`, `.yusei-magic-regular`, `.freckle-face-regular`) remain in
`app.css`.

> **Recommendation:** self-host just Nunito + Poppins (e.g. `@fontsource/*`) and drop the
> unused families. This removes render-blocking third-party CSS and avoids sending visitor
> IPs to Google (a GDPR consideration for EU visitors), matching the site's privacy posture.

### Type treatment conventions

- Page titles: `text-4xl`/`text-5xl`, `font-black`, `lowercase`, dark purple `#352f47`.
- Navigation: `lowercase`, `font-semibold`, `tracking-[0.05em]`.
- Body copy: `text-base`/`text-lg`, muted `#4c465f`, generous max-width.

---

## 3. Layout & spacing

- **Container:** `.page-shell` (defined in `app.css`) centers content and caps width at
  **68.75rem (~1100px)** with responsive side gutters — `100% − 2rem` on mobile, `−3rem` at
  `sm`, `−4rem` at `xl`. Use it for every content section.
- **Header:** sticky/fixed on desktop (`md:fixed top-0`), static on mobile. Two stacked bars —
  a white logo bar over a dark-purple (`#352f47`) nav strip. `<main>` adds top padding
  (`md:pt-36 lg:pt-40`) to clear the fixed header.
- **Sections:** full-bleed colored bands (sky `#d7e6ff`, `blue-900`, etc.) with inner content
  constrained by `.page-shell`. Sections use `overflow-hidden` to contain decorative art.
- **Footer:** layered bands (sky → `#c4d7f6` → `stone-800`) with a circular author logo
  straddling the seam, a centered wordmark, social icons, and a copyright line.
- **Breakpoints:** standard Tailwind (`sm` 640, `md` 768, `lg` 1024, `xl` 1280). Mobile-first;
  the desktop nav (`hidden md:flex`) swaps to a button-triggered dropdown menu below `md`.

---

## 4. Components & patterns

| Component            | Behavior                                                                 |
| -------------------- | ------------------------------------------------------------------------ |
| **Header + nav**     | Logo links home; desktop shows inline lowercase nav; mobile shows a rose "Menu" button opening a dropdown. |
| **Mobile menu**      | Toggled on click; closes on outside click or `Escape` (returns focus to the button); `aria-expanded` / `aria-controls` wired up. |
| **Buttons / CTAs**   | Rounded, `rose-600` fill, white text, `rose-700` hover; used for menu and shop CTAs. |
| **Character cards**  | Carla, Crosby, Tank in the `blue-900` characters section, each with art + a short bio. |
| **Newsletter popup** | "Join Crosby's Crew" modal on the home page — backdrop, close button, and a MailerLite `<iframe>` (fixed `src`); auto-opens once per visit, closable. |
| **Buy buttons**      | Open `catchupwithalice.myshopify.com` in a new tab with `rel="noopener noreferrer"`. |
| **Freebie tiles**    | Coloring-page thumbnails linking to printable PDFs in a new tab.         |
| **Image panels**     | About page composes bio content as full-width illustrated images.        |
| **Cosmic Dash game** | Canvas endless-runner on the freebies page with a character picker (see §4a). |

### 4a. Crosby's Cosmic Dash (game)

A `<canvas>` endless-runner (`src/lib/CosmicDash.svelte`) themed to the book's characters —
the brand's most interactive surface. It is designed to feel like a storybook toy rather than
an arcade game.

- **Framing:** sits in a rounded, white-bordered "game-frame" over the sky-blue `#d7e6ff`
  background with a soft drop shadow — the same rounded, pastel language as the rest of the site.
- **Character picker:** pill buttons for Crosby / Carla / Tank, each tinted with its own accent
  (Crosby `#8fc27b`, Carla `#ef6f3c`, Tank per-character), the selected pill ringed in that accent;
  disabled while a run is in progress.
- **Art direction:** everything is drawn procedurally on the canvas in the site palette — moon
  rocks, starberry bushes, comets, collectible stars, parallax clouds/doodles, and a slow
  day↔night tint cycle. Canvas text uses **Nunito** (redrawn once `document.fonts.ready`).
- **Motion:** continuous `requestAnimationFrame` loop while playing; a short particle "burst"
  on game-over. Note this canvas animation does **not** currently consult
  `prefers-reduced-motion` (acceptable because play is user-initiated and pausable, but worth a
  future toggle).
- **Focus/affordances:** a 3px `#d2234c` focus-visible outline on the frame matches the
  site-wide focus treatment; the site's global image-hover zoom is explicitly disabled on the
  character portraits so they don't jostle.

---

## 5. Motion

Motion is deliberate and soft, always gated for accessibility.

- **Page transitions:** View Transitions API (`onNavigate` + `document.startViewTransition`)
  cross-fades routes over 0.5s.
- **Scroll reveal:** elements with `.scroll-reveal` fade/slide in via an `IntersectionObserver`
  that adds `.animate-in`; the characters section slides in from the right.
- **Decorative:** animated "falling stars" in the characters section, driven by per-star CSS
  custom properties (`--size`, `--rotate`, randomized duration/delay).
- **Hover:** images scale to `1.05` and links/buttons ease color/transform changes, using a
  slow `cubic-bezier(0.165, 0.84, 0.44, 1)` (600–800ms) for a gentle feel. The wordmark link
  opts out of the hover scale.

**All of the above respects `prefers-reduced-motion: reduce`**, which collapses animation and
transition durations to ~0 and disables smooth scroll.

---

## 6. Accessibility

Baked in throughout:

- **Skip link** — "Skip to main content" appears on focus and jumps to `#main-content`.
- **Focus-visible** — 2px `#d2234c` outline with offset on all interactive elements; form
  controls get a matching high-contrast ring.
- **Keyboard** — mobile menu is fully operable, closes on `Escape`, and restores focus.
- **ARIA** — labeled nav toggle, social links, close buttons, and the newsletter iframe.
- **Reduced motion** — honored globally (see §5).
- **Images** — all carry `alt` text; `img { max-width:100%; height:auto; display:block }`
  keeps them responsive. The non-editable body hides the text caret while preserving it on
  real form fields.

---

## 7. Imagery & voice

- **Art style:** soft, rounded, storybook character illustrations (Crosby, Tank, Carla) and
  decorative "blob" shapes; pastel sky-blue is the connective background throughout.
- **Assets:** stored in `static/` (PNG/WebP/JPEG) and referenced via `resolve("/name.png")`;
  printable freebies are PDFs. Several source images are large — see the optimization notes
  below.
- **Voice:** friendly, lowercase, kid-oriented ("join crosby's crew", "shop coming soon"),
  warm and encouraging in character bios.

---

## 8. Known design/implementation gaps

Documented so they aren't mistaken for intent:

1. **Dead color tokens** — `tailwind.config.js` isn't consumed by Tailwind v4; move tokens to
   `@theme` in `app.css` or remove the file.
2. **Font bloat** — ~16 Google Font families requested, ~2 used; self-host Nunito + Poppins.
3. **Unused CSS classes** — `.font-grandstander`, `.yusei-magic-regular`,
   `.freckle-face-regular` can be removed.
4. **No CSP** — add a `<meta http-equiv="Content-Security-Policy">` in `app.html` (GitHub
   Pages can't set the header) allowing self, Google Fonts, and `assets.mailerlite.com`.
5. **Image weight / LCP** — hero and coloring-page images are large and some are lazy-loaded;
   build-time optimization (`@sveltejs/enhanced-img`, right-sized thumbnails, eager LCP image
   with `fetchpriority="high"`) would cut page weight substantially.
6. **Social meta** — `og:image` resolves to `undefined` (commented-out `site.image`); add an
   absolute 1200×630 image, per-page descriptions, and a canonical link.
7. **Game & reduced motion** — Cosmic Dash's canvas loop ignores `prefers-reduced-motion`;
   consider a reduced-motion or "pause animations" affordance for motion-sensitive players.
