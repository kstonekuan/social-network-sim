<script lang="ts">
  import Sidebar from './lib/components/Sidebar.svelte'
  import Timeline from './lib/components/Timeline.svelte'
  import Profile from './lib/components/Profile.svelte'
  import ThemeToggle from './lib/components/ThemeToggle.svelte'
  import { theme } from './lib/stores/theme'
  import type { Agent } from './lib/types'
  import { onMount } from 'svelte'

  let currentView = $state<'timeline' | 'profile'>('timeline')
  let selectedAgent = $state<Agent | null>(null)

  function handleViewChange(event: { view: 'timeline' | 'profile', agent?: Agent }) {
    currentView = event.view
    selectedAgent = event.agent || null
  }

  onMount(() => {
    theme.init()
  })
</script>

<main class="flex min-h-screen bg-gray-100 dark:bg-gray-900 transition-colors">
  <Sidebar onviewchange={handleViewChange} />
  
  <div class="flex-1 min-h-screen">
    <header class="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 p-4 transition-colors">
      <div class="flex items-center justify-between">
        <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
          AI Social Network Simulation
        </h1>
        <ThemeToggle />
      </div>
    </header>
    
    <div class="p-4 min-h-[calc(100vh-73px)] bg-gray-100 dark:bg-gray-900">
      {#if currentView === 'timeline'}
        <Timeline />
      {:else if currentView === 'profile' && selectedAgent}
        <Profile agent={selectedAgent} />
      {/if}
    </div>
  </div>
</main>