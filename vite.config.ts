import adapter from '@sveltejs/adapter-static';
import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';
import tailwindcss from '@tailwindcss/vite';
import { enhancedImages } from '@sveltejs/enhanced-img';

const BASE_PATH = process.env.BASE_PATH || '';

const config: UserConfig = {
  plugins: [
    tailwindcss(),
    enhancedImages(),
    sveltekit({
      compilerOptions: { runes: true },
      adapter: adapter({ pages: 'build', assets: 'build', fallback: 'index.html' }),
      paths: { base: BASE_PATH },
      prerender: { entries: ['*'] }
    })
  ]
};

export default config;
