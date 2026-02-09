/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.rs",
  ],
  theme: {
    extend: {
      colors: {
        primary: {
          50: '#fef5f5',
          100: '#fde8e8',
          200: '#fbd5d5',
          300: '#f8b5b5',
          400: '#f4c2c2',
          500: '#e89b9b',
          600: '#d97575',
          700: '#c25555',
          800: '#a13f3f',
          900: '#853838',
        },
        secondary: {
          50: '#f3f7f3',
          100: '#e4ede4',
          200: '#c9dcc9',
          300: '#a8c5a8',
          400: '#82a882',
          500: '#638e63',
          600: '#4d714d',
          700: '#3f5c3f',
          800: '#354b35',
          900: '#2d3e2d',
        },
        accent: {
          50: '#fdfbf3',
          100: '#faf4e0',
          200: '#f5e7bc',
          300: '#efd88d',
          400: '#d4af37',
          500: '#c9a332',
          600: '#b38a2a',
          700: '#956d25',
          800: '#7a5723',
          900: '#664920',
        },
        background: '#fffaf5',
      },
      fontFamily: {
        serif: ['Georgia', 'Times New Roman', 'serif'],
        sans: ['-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'Roboto', 'sans-serif'],
      },
      animation: {
        'fade-in': 'fadeIn 0.5s ease-in',
        'slide-in': 'slideIn 0.3s ease-out',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0', transform: 'translateY(10px)' },
          '100%': { opacity: '1', transform: 'translateY(0)' },
        },
        slideIn: {
          '0%': { transform: 'translateX(-100%)' },
          '100%': { transform: 'translateX(0)' },
        },
      },
    },
  },
  plugins: [],
}