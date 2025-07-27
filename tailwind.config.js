import { defineConfig } from '@tailwindcss/vite'

export default defineConfig({
  content: ['./visualizer/src/**/*.{html,js,svelte,ts}', './visualizer/index.html'],
})