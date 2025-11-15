/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        primary: '#007BFF',
        'background-light': '#F8F9FA',
        'background-dark': '#1C1E2A',
        'text-light': '#212529',
        'text-dark': '#EAEAEA',
        'surface-light': '#FFFFFF',
        'surface-dark': '#2a2d3d',
        'border-light': '#e8ebf3',
        'border-dark': '#34384a'
      },
      fontFamily: {
        display: ['Inter', 'sans-serif']
      }
    }
  },
  plugins: []
};
