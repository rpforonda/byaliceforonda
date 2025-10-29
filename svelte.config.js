import adapter from '@sveltejs/adapter-static';

const BASE_PATH = process.env.BASE_PATH || '';

export default {
  compilerOptions: { runes: true },
  kit: {
    adapter: adapter({
      pages: 'build',
      assets: 'build',
      fallback: 'index.html'
    }),
    paths: { base: BASE_PATH },
    prerender: { entries: ['*'] }
  }
};
