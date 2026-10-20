import { SITE_URL } from "#lib/seo.js";

export const prerender = true;

// Public, indexable pages. /shop is a "coming soon" placeholder (noindex), so it's left out.
const pages = ["/", "/about", "/book", "/freebies", "/freebies/cosmic-dash"];

export function GET() {
  const urls = pages
    .map((path) => `  <url>\n    <loc>${SITE_URL}${path}</loc>\n  </url>`)
    .join("\n");

  const xml = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${urls}
</urlset>
`;

  return new Response(xml, { headers: { "Content-Type": "application/xml" } });
}
