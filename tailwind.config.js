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
          50: '#f9f7f4',
          100: '#f3f0ea',
          200: '#e8e1d5',
          300: '#ddd2c0',
          400: '#d2c3ab',
          500: '#c7b496',
          600: '#b39d7e',
          700: '#8f7d63',
          800: '#6b5e4a',
          900: '#4a4134',
        },
        secondary: {
          50: '#f5f3f1',
          100: '#ebe7e3',
          200: '#d7cfc7',
          300: '#c3b7ab',
          400: '#af9f8f',
          500: '#9b8773',
          600: '#826f5e',
          700: '#6a5a4c',
          800: '#52453a',
          900: '#3d332b',
        },
        accent: {
          50: '#fdfcfb',
          100: '#faf8f5',
          200: '#f5f1eb',
          300: '#efe9e0',
          400: '#e8dfd1',
          500: '#d9cdb9',
          600: '#c9b89a',
          700: '#b19e7d',
          800: '#8f7d5f',
          900: '#6d5d45',
        },
        background: '#f9f7f4',
      },
      fontFamily: {
        serif: ['Playfair Display', 'Georgia', 'Cambria', 'Times New Roman', 'serif'],
        sans: ['Lato', 'Inter', '-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'Roboto', 'sans-serif'],
      },
      animation: {
        'fade-in': 'fadeIn 0.8s ease-in-out',
        'slide-in': 'slideIn 0.5s ease-out',
        'scale-in': 'scaleIn 0.4s ease-out',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0', transform: 'translateY(30px)' },
          '100%': { opacity: '1', transform: 'translateY(0)' },
        },
        slideIn: {
          '0%': { transform: 'translateX(-100%)', opacity: '0' },
          '100%': { transform: 'translateX(0)', opacity: '1' },
        },
        scaleIn: {
          '0%': { transform: 'scale(0.92)', opacity: '0' },
          '100%': { transform: 'scale(1)', opacity: '1' },
        },
      },
    },
  },
  plugins: [],
}