/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{rs,html}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      fontFamily: {
        'signature': ['Dancing Script', 'Great Vibes', 'cursive'],
        'serif': ['Georgia', 'Times New Roman', 'serif'],
      },
    },
  },
  plugins: [],
}

