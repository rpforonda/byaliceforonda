# Author Site — SvelteKit 5 (runes) + Tailwind — GitHub Pages

Ready-to-import repo for an author brochure site:
- Pure client-side, static deploy (no servers)
- SvelteKit (Svelte 5 runes) + Tailwind CSS
- Book page with buy buttons (Amazon/Ingram)
- Character quiz with share-to-social
- GitHub Actions to auto-deploy to Pages
- SPA refresh fix via `404.html`

## Quick start

1. **Install deps**
   ```bash
   npm i
   ```

2. **Run locally**
   ```bash
   npm run dev
   ```

3. **Configure GitHub Pages path**
   - If this repo will be a **project page** (`https://username.github.io/<repo>`):
     - Edit `.github/workflows/deploy.yml` and set:
       ```yml
       BASE_PATH: "/<repo>"
       ```
   - If this repo will be a **user/org page** (`https://username.github.io/`):
     - Set `BASE_PATH: ""` (empty).
     - (Optional) The repo name must be `username.github.io`.

4. **Deploy**
   - Commit and push to `main`. GitHub Actions will build and publish automatically.
   - In the repo: **Settings → Pages** → Source: **GitHub Actions**.

## Where to edit

- Content pages in `src/routes/`:
  - `/` → `src/routes/+page.svelte`
  - `/about` → `src/routes/about/+page.svelte`
  - `/dedication` → `src/routes/dedication/+page.svelte`
  - `/book` → `src/routes/book/+page.svelte` (also embeds the quiz)
  - `/quiz` → `src/routes/quiz/+page.svelte`

- Components:
  - `src/lib/components/BuyButtons.svelte` — set the ASIN and optional affiliate tag; set `ingramLink`.

## Compliance notes

- Site does **not** collect personal data or payments.
- Buy buttons open Amazon/Ingram in a new tab.
- No trackers included by default.

Enjoy!
