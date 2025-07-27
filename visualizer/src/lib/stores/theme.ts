import { writable } from 'svelte/store'

type Theme = 'light' | 'dark'

// Check if we're in browser environment
const isBrowser = typeof window !== 'undefined'

// Get initial theme from localStorage or system preference
const getInitialTheme = (): Theme => {
  if (!isBrowser) return 'light'
  
  // Check if user has saved preference
  const saved = localStorage.getItem('theme') as Theme
  if (saved) return saved
  
  // No saved preference, use system preference
  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
}

// Apply theme to HTML element
const applyTheme = (theme: Theme) => {
  if (!isBrowser) return
  
  if (theme === 'dark') {
    document.documentElement.classList.add('dark')
  } else {
    document.documentElement.classList.remove('dark')
  }
  localStorage.setItem('theme', theme)
}

// Create the theme store
const createThemeStore = () => {
  const { subscribe, set, update } = writable<Theme>(getInitialTheme())

  return {
    subscribe,
    set: (theme: Theme) => {
      applyTheme(theme)
      set(theme)
    },
    toggle: () => {
      update(current => {
        const newTheme = current === 'light' ? 'dark' : 'light'
        applyTheme(newTheme)
        return newTheme
      })
    },
    init: () => {
      const theme = getInitialTheme()
      applyTheme(theme)
      set(theme)
    }
  }
}

export const theme = createThemeStore()