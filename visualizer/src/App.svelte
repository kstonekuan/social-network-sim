<script lang="ts">
  import Sidebar from './lib/components/Sidebar.svelte'
  import Timeline from './lib/components/Timeline.svelte'
  import Profile from './lib/components/Profile.svelte'
  import type { Agent } from './lib/types'

  let currentView = $state<'timeline' | 'profile'>('timeline')
  let selectedAgent = $state<Agent | null>(null)

  function handleViewChange(event: { view: 'timeline' | 'profile', agent?: Agent }) {
    currentView = event.view
    selectedAgent = event.agent || null
  }
</script>

<main class="flex min-h-screen bg-gray-100">
  <Sidebar onviewchange={handleViewChange} />
  
  <div class="flex-1 min-h-screen">
    <header class="bg-white border-b border-gray-200 p-4">
      <h1 class="text-2xl font-bold text-gray-900">
        AI Social Network Simulation
      </h1>
    </header>
    
    <div class="p-4">
      {#if currentView === 'timeline'}
        <Timeline />
      {:else if currentView === 'profile' && selectedAgent}
        <Profile agent={selectedAgent} />
      {/if}
    </div>
  </div>
</main>