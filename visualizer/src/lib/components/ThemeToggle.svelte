<script lang="ts">
  import { theme } from '../stores/theme'
  
  function toggleTheme() {
    theme.toggle()
  }
  
</script>

<button
  onclick={toggleTheme}
  class="p-2 rounded-lg transition-colors border-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
  class:bg-gray-200={$theme === 'light'}
  class:bg-gray-700={$theme === 'dark'}
  class:text-gray-900={$theme === 'light'}
  class:text-white={$theme === 'dark'}
  aria-label="Toggle dark mode"
  title="Toggle dark mode"
>
  <span class="text-sm font-medium">
    {$theme === 'light' ? '🌙 Dark' : '☀️ Light'}
  </span>
</button>