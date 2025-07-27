<script lang="ts">
  import { onMount } from 'svelte'
  import { ApiService } from '../api'
  import type { Agent } from '../types'

  interface Props {
    onviewchange?: (event: { view: 'timeline' | 'profile', agent?: Agent }) => void
  }

  let { onviewchange }: Props = $props()

  let agents = $state<Agent[]>([])
  let loading = $state(true)
  let error = $state<string | null>(null)
  let currentView = $state<'timeline' | 'profile'>('timeline')

  onMount(async () => {
    await loadAgents()
  })

  async function loadAgents() {
    try {
      loading = true
      error = null
      agents = await ApiService.getAllAgents()
    } catch (err) {
      error = 'Failed to load agents'
      console.error('Error loading agents:', err)
    } finally {
      loading = false
    }
  }

  function showTimeline() {
    currentView = 'timeline'
    onviewchange?.({ view: 'timeline' })
  }

  function showProfile(agent: Agent) {
    currentView = 'profile'
    onviewchange?.({ view: 'profile', agent })
  }
</script>

<aside class="sidebar">
  <div class="mb-8">
    <h2 class="text-lg font-bold text-gray-900 dark:text-white mb-4">Navigation</h2>
    
    <nav class="space-y-2">
      <button
        class="w-full text-left px-3 py-2 rounded-md transition-colors"
        class:bg-blue-100={currentView === 'timeline'}
        class:dark:bg-blue-900={currentView === 'timeline'}
        class:text-blue-700={currentView === 'timeline'}
        class:dark:text-blue-300={currentView === 'timeline'}
        class:hover:bg-gray-200={currentView !== 'timeline'}
        class:dark:hover:bg-gray-700={currentView !== 'timeline'}
        onclick={showTimeline}
      >
        <div class="flex items-center space-x-2">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2-2z"></path>
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 5a2 2 0 012-2h4a2 2 0 012 2v0a2 2 0 01-2 2H10a2 2 0 01-2-2v0z"></path>
          </svg>
          <span>Global Feed</span>
        </div>
      </button>
    </nav>
  </div>

  <div>
    <div class="flex items-center justify-between mb-4">
      <h2 class="text-lg font-bold text-gray-900 dark:text-white">Agents</h2>
      <button 
        class="text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 text-sm transition-colors"
        onclick={loadAgents}
        disabled={loading}
      >
        {loading ? '...' : '↻'}
      </button>
    </div>

    {#if loading}
      <div class="flex justify-center items-center py-4">
        <div class="animate-spin rounded-full h-6 w-6 border-b-2 border-blue-500"></div>
      </div>
    {:else if error}
      <div class="bg-red-50 dark:bg-red-900/50 border border-red-200 dark:border-red-800 rounded-lg p-3 text-red-700 dark:text-red-300 text-sm">
        {error}
        <button class="block mt-2 underline hover:no-underline" onclick={loadAgents}>Try again</button>
      </div>
    {:else if agents.length === 0}
      <div class="text-center py-4 text-gray-500 dark:text-gray-400 text-sm">
        No agents found. Run the initializer to create agents.
      </div>
    {:else}
      <div class="space-y-1">
        {#each agents as agent (agent.id)}
          <button
            class="w-full text-left px-3 py-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"
            onclick={() => showProfile(agent)}
          >
            <div class="flex items-center space-x-3">
              <div class="w-8 h-8 bg-blue-500 rounded-full flex items-center justify-center text-white text-sm font-bold">
                {agent.name.charAt(0).toUpperCase()}
              </div>
              <div class="flex-1 min-w-0">
                <div class="font-medium text-gray-900 dark:text-white truncate">
                  {agent.name}
                </div>
                <div class="text-xs text-gray-500 dark:text-gray-400 truncate">
                  @agent{agent.id}
                </div>
              </div>
            </div>
          </button>
        {/each}
      </div>
    {/if}
  </div>

  <div class="mt-8 pt-4 border-t border-gray-200 dark:border-gray-700">
    <p class="text-xs text-gray-500 dark:text-gray-400">
      AI Social Network Simulation
    </p>
    <p class="text-xs text-gray-400 dark:text-gray-500 mt-1">
      {agents.length} agents active
    </p>
  </div>
</aside>