/** @type {import('tailwindcss').Config} */
const defaultTheme = require('tailwindcss/defaultTheme')

module.exports = {
  darkMode: ['class', '[data-theme="dark"]'],
  content: [
    "./src/**/*.rs",
    "./index.html",
    "./input.css"
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter var', ...defaultTheme.fontFamily.sans],
      },
      colors: {
        border: "hsl(var(--border) / <alpha-value>)",
        input: "hsl(var(--input) / <alpha-value>)",
        ring: "hsl(var(--ring) / <alpha-value>)",
        background: "hsl(var(--b1) / <alpha-value>)",
        foreground: "hsl(var(--bc) / <alpha-value>)",
        primary: {
          DEFAULT: "hsl(var(--p) / <alpha-value>)",
          foreground: "hsl(var(--pc) / <alpha-value>)",
        },
        secondary: {
          DEFAULT: "hsl(var(--s) / <alpha-value>)",
          foreground: "hsl(var(--sc) / <alpha-value>)",
        },
        accent: {
          DEFAULT: "hsl(var(--a) / <alpha-value>)",
          foreground: "hsl(var(--ac) / <alpha-value>)",
        },
        destructive: {
          DEFAULT: "hsl(var(--er) / <alpha-value>)",
          foreground: "hsl(var(--erc) / <alpha-value>)",
        },
        muted: {
          DEFAULT: "hsl(var(--n) / <alpha-value>)",
          foreground: "hsl(var(--nc) / <alpha-value>)",
        },
        card: {
          DEFAULT: "hsl(var(--b1) / <alpha-value>)",
          foreground: "hsl(var(--bc) / <alpha-value>)",
        },
      },
      borderRadius: {
        lg: 'var(--rounded-box, 1rem)',
        md: 'calc(var(--rounded-box, 1rem) - 2px)',
        sm: 'calc(var(--rounded-box, 1rem) - 4px)',
      },
      minWidth: {
        '20': '5rem',
        '72': '18rem',
        '80': '20rem',
      },
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('daisyui')
  ],
  daisyui: {
    themes: [
      {
        light: {
          ...require('daisyui/src/theming/themes')['light'],
          primary: '#fbbd23',
          secondary: '#f000b8',
          accent: '#37cdbe',
          neutral: '#3d4451',
          'base-100': '#ffffff',
        },
      },
      {
        dark: {
          ...require('daisyui/src/theming/themes')['dark'],
          primary: '#fbbd23',
          secondary: '#f000b8',
          accent: '#37cdbe',
          neutral: '#2a2e37',
          'base-100': '#1f2937',
        },
      },
      'bumblebee',
    ],
    darkTheme: 'dark',
    base: true,
    styled: true,
    utils: true,
    logs: true,
  },
}
