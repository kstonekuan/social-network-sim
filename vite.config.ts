import { svelte } from '@sveltejs/vite-plugin-svelte'
import tailwindcss from '@tailwindcss/vite'
import { defineConfig } from 'vite'

// https://vite.dev/config/
export default defineConfig({
  root: 'visualizer',
  plugins: [svelte(), tailwindcss()],
  build: {
    minify: 'esbuild',
    sourcemap: true,
    outDir: '../dist',
    emptyOutDir: true,
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ['axios', 'date-fns'],
          svelte: ['svelte']
        }
      }
    }
  }
})
