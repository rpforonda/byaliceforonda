# Design documentation — catchupwithalice.com

The visual language for Alice Foronda's author site. The tone is **warm, playful, and
child-friendly** — soft pastel-blue backgrounds, a confident rose-red accent, rounded shapes,
gentle motion, and lowercase navigation. This document records the design as it is actually
built, so it can be reproduced and extended consistently.

> For stack, structure, and how to run the site, see the [README](../README.md).

---

## 1. Color

Colors are applied as **inline hex values and Tailwind utilities** directly in the markup
(e.g. `bg-[#d7e6ff]`, `text-[#352f47]`, `rose-600`). There are no theme tokens yet: a
former `tailwind.config.js` palette was never read by Tailwind v4 (via `@tailwindcss/vite`)
and has been removed; its values are kept below as reference. Treat the values below as the
source of truth.

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

### Brand reference tokens (not wired up)

Kept as the intended broader palette should the project move tokens into an `@theme` block:

- **primary** — `#d2234c` · dark `#b31d41` · light `#e8365e` · red `#db2108`
- **accent** — gold `#ffd28d` · blue `#8fd3ff` · pink `#ffb4c6` · light-blue `#c7e9ff` ·
  orange `#f09a58` · green `#7fa66d` · dark-green `#25533F` · olive `#6b8a32`
- **text** — dark `#1b1b1d` · dark-blue `#0b1a2d` · navy `#052238`
- **bg** — pink-light `#ffe0ec` · pink-soft `#ffe8f0` · blue-soft `#e6f0ff`

> **Recommendation:** promote the in-use values into an `@theme` block in `app.css` so color
> is defined once and referenced as utilities. Until then,
> new work should reuse the hex values in the *Core palette* table rather than invent shades.

---

## 2. Typography

**Nunito** is the primary typeface, applied at the shell via inline
`font-family: 'Nunito', sans-serif`. Headings lean heavy and lowercase for a playful,
approachable feel (`font-black`, `lowercase`, slight letter-spacing e.g. `tracking-[0.04em]`).

- **Body & UI:** Nunito (400 / 600 / 700)
- **Secondary:** Poppins 400 (available, used sparingly)

Fonts are **self-hosted via `@fontsource`** — Nunito (400/600/700) and Poppins (400) are
imported in `+layout.svelte` and bundled from `node_modules` at build time (served as subset
`.woff2` from the site's own origin). There are **no third-party font requests**, so no
visitor IPs are sent to Google and the site stays cookieless — matching its privacy posture.
The character names on the home page ask for `Inter`, which is not bundled, so they fall back
to the system sans.

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
| **Newsletter popup** | "Join Crosby's Crew" modal on the home page — backdrop, close button, and a MailerLite `<iframe>` (fixed `src`); auto-opens after 5s or at 75% scroll, once per home-page load (dismissal isn't remembered across visits); closable. |
| **Buy button**       | Single "buy on amazon" button opening the Amazon listing in a new tab with `rel="noopener noreferrer"` (the Shopify store is closed). |
| **Freebie tiles**    | Coloring-page thumbnails linking to printable PDFs in a new tab.         |
| **Image panels**     | About page composes bio content as full-width illustrated images.        |
| **Cosmic Dash game** | Canvas endless-runner on the freebies page with a character picker (see §4a). |

### 4a. Crosby's Cosmic Dash (game)

A `<canvas>` endless-runner (engine and in-game palette in `cosmic-dash/src/`, UI in `src/lib/CosmicDash.svelte`) themed to the book's characters —
the brand's most interactive surface. It is designed to feel like a storybook toy rather than
an arcade game.

- **Framing:** sits in a rounded, white-bordered "game-frame" over the sky-blue `#d7e6ff`
  background with a soft drop shadow — the same rounded, pastel language as the rest of the site.
- **Where it lives:** its own page, `/freebies/cosmic-dash`. `/freebies` promotes it above
  the coloring pages with a **game card**: the white-bordered rounded frame around a night-sky
  gradient (`#120f33 → #2b2466 → #4f3a8c`) with a few gold `★` / `✦` sparkles, a small
  "FREE GAME" kicker in `#ffd28d`, the title in the cover's yellow `#f3d36f` with a maroon
  `#7c2b28` offset shadow, a one-line pitch, a rose `#d2234c` "play now ▶" pill, and the three
  sprites standing on a strip of the lavender moon ground. The whole card is one link.
- **Everything in the canvas:** there's no picker, skill card or help text around the game.
  The **intro screen is the character select**, drawn on the canvas over the live scene in a
  translucent `rgba(18,15,51,0.72)` panel: the yellow/maroon title; three friend cards (sprite
  on a small glow, name, 5 gold power stars) with the chosen one filled and ringed in its
  accent (Crosby `#7dbd55`, Carla `#d68a4c`, Tank `#e8a24a`) and the others dimmed; the chosen
  friend's name in gold with their role (and "main hero" for Crosby); gold "Passive · …" and
  "Special · … (n ★)" labels with the descriptions wrapped beside them; and the controls in
  `#ffd28d` along the bottom. Hints use touch wording ("tap a friend…") or key names depending
  on the last input. The crash screen adds an outlined "◀ choose a friend" pill.
- **Special button:** a ★ button tinted in the chosen friend's accent sits in a dark `#241d4a`
  strip under the canvas with the star count, and pulses gold when charged.
- **Characters:** colored sprites (`src/lib/assets/game/`) made by painting the pencil sketches
  in the book-cover palette with the pencil kept on top (multiply), so they read like the
  book's watercolor art.
- **Art direction:** deep space drawn procedurally on the canvas — a `#120f33 → #4f3a8c` sky,
  twinkling parallax star layers, gold storybook stars, nebulae, ringed/blue planets, shooting
  stars, a lavender cratered moon surface, moon rocks, glowing teal/pink space crystals, and
  star-faced comets. Every 500 points the nebulae shift to a new zone palette with a
  "✦ entering the … ✦" banner. Titles keep the cover's yellow `#f3d36f` with maroon `#7c2b28`
  drop shadow; canvas text uses **Nunito** (redrawn once `document.fonts.ready`).
- **Motion:** continuous `requestAnimationFrame` loop while playing; a short particle "burst"
  on game-over. Note this canvas animation does **not** currently consult
  `prefers-reduced-motion` (acceptable because play is user-initiated and pausable, but worth a
  future toggle).
- **Share score:** the game-over screen shows the final score and a rose "share my score"
  button. It draws a 1080×1080 score-card **image** on an off-screen canvas (space gradient +
  the chosen character sprite + the score in the cover's gold/maroon, footer URL) and shares
  the image plus a caption via the Web Share API — an image is what Instagram/Facebook actually
  post, since they ignore pre-filled text. Fallbacks: text-only share, or saving the card on
  desktop. The card and text are built entirely client-side from the run's own numbers — **no
  name, identifier, or server call**, same-origin sprites so the canvas isn't tainted — so
  sharing adds no privacy surface.
- **Focus/affordances:** a 3px `#d2234c` focus-visible outline on the frame matches the
  site-wide focus treatment; the site's global image-hover zoom is disabled on the promo
  card's sprites so they don't jostle.

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
  keeps them responsive, and the `width`/`height` that `<enhanced:img>` emits reserve space
  before they load. The non-editable body hides the text caret while preserving it on
  real form fields.

---

## 7. Imagery & voice

- **Art style:** soft, rounded, storybook character illustrations (Crosby, Tank, Carla) and
  decorative "blob" shapes; pastel sky-blue is the connective background throughout.
- **Assets:** source images live in `src/lib/assets/` and are rendered with `<enhanced:img>`,
  which serves AVIF/WebP at the displayed size. See [Images in the README](../README.md#images)
  for the sizing conventions. Printable freebies are PDFs in `static/`. The favicon is a 32px
  PNG plus a 180px `apple-touch-icon` cut from the round author logo.
- **Voice:** friendly, lowercase, kid-oriented ("join crosby's crew", "shop coming soon"),
  warm and encouraging in character bios.

---

## 8. Known design/implementation gaps

Documented so they aren't mistaken for intent:

1. **No color tokens** — colors are hardcoded hex values; move the core palette into
   `@theme` in `app.css`.
2. **No CSP** — add a `<meta http-equiv="Content-Security-Policy">` in `app.html` (GitHub
   Pages can't set the header) allowing self and `assets.mailerlite.com` (fonts are now
   self-hosted, so no font origin is needed). The game's WebAssembly also needs
   `'wasm-unsafe-eval'` in `script-src`, or the game won't start.
3. **Text in artwork** — the about letter and book description are images with hidden text
   transcripts; real HTML text styled to match would be better for SEO and accessibility.
4. **Game & reduced motion** — Cosmic Dash's canvas loop ignores `prefers-reduced-motion`;
   consider a reduced-motion or "pause animations" affordance for motion-sensitive players.
5. **Game text is drawn, not HTML** — the intro (character descriptions, controls) is painted on
   the canvas, so it can't be selected, zoomed as text, or read by screen readers directly. The
   component compensates with a visually hidden controls description and `aria-live`
   announcements of each character choice; keep those in sync when the intro text changes.
