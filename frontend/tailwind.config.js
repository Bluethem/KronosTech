/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        primary: '#2b8cee',
        'background-light': '#f6f7f8',
        'background-dark': '#101922',
        'text-light': '#212529',
        'text-dark': '#EAEAEA',
        'surface-light': '#FFFFFF',
        'surface-dark': '#2a2d3d',
        'border-light': '#e8ebf3',
        'border-dark': '#34384a'
      },
      fontFamily: {
        display: ['Inter', 'sans-serif']
      },
      borderRadius: {
        DEFAULT: '0.25rem',
        lg: '0.5rem',
        xl: '0.75rem',
        full: '9999px'
      }
    }
  },
  plugins: []
};
