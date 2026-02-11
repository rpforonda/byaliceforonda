/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  darkMode: 'media', // Enable dark mode with prefers-color-scheme
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: '#d2234c',
          dark: '#b31d41',
          light: '#e8365e',
          red: '#db2108',
        },
        accent: {
          gold: '#ffd28d',
          blue: '#8fd3ff',
          pink: '#ffb4c6',
          'light-blue': '#c7e9ff',
          orange: '#f09a58',
          green: '#7fa66d',
          'dark-green': '#25533F',
          olive: '#6b8a32',
        },
        text: {
          dark: '#1b1b1d',
          'dark-blue': '#0b1a2d',
          'navy': '#052238',
        },
        bg: {
          'pink-light': '#ffe0ec',
          'pink-soft': '#ffe8f0',
          'blue-soft': '#e6f0ff',
        },
      },
      fontFamily: {
        sans: ['Nunito', 'system-ui', 'sans-serif'],
        cursive: ['Great Vibes', 'cursive'],
        display: ['Chewy', 'cursive'],
        playful: ['DynaPuff', 'More Sugar', 'Chewy', 'system-ui', 'sans-serif'],
        handwriting: ['Borel', 'cursive'],
        script: ['Corinthia', 'cursive'],
        heading: ['Oswald', 'sans-serif'],
      },
    },
  },
  plugins: []
};
