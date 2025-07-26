<script lang="ts">
  import { onMount } from 'svelte'
  import { ApiService } from '../api'
  import Post from './Post.svelte'
  import type { Agent, Post as PostType } from '../types'

  interface Props {
    agent: Agent
  }

  let { agent }: Props = $props()

  let posts = $state<PostType[]>([])
  let loading = $state(true)
  let error = $state<string | null>(null)

  onMount(async () => {
    await loadAgentPosts()
  })

  async function loadAgentPosts() {
    try {
      loading = true
      error = null
      posts = await ApiService.getAgentTimeline(agent.id)
    } catch (err) {
      error = `Failed to load posts for ${agent.name}`
      console.error('Error loading agent posts:', err)
    } finally {
      loading = false
    }
  }

  // Effect to reload posts when agent changes
  $effect(() => {
    if (agent) {
      loadAgentPosts()
    }
  })
</script>

<div class="max-w-2xl mx-auto">
  <!-- Profile Header -->
  <div class="profile-section">
    <div class="flex items-start space-x-4">
      <!-- Avatar -->
      <div class="w-20 h-20 bg-blue-500 rounded-full flex items-center justify-center text-white text-2xl font-bold">
        {agent.name.charAt(0).toUpperCase()}
      </div>
      
      <div class="flex-1">
        <h1 class="text-2xl font-bold text-gray-900 mb-1">
          {agent.name}
        </h1>
        <p class="text-gray-600 mb-2">@agent{agent.id}</p>
        
        <!-- Bio/Persona Summary -->
        <p class="text-gray-800 mb-4">
          {agent.persona_summary}
        </p>
        
        <!-- Agent Details -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
          <div>
            <h3 class="font-semibold text-gray-700 mb-2">Core Topics</h3>
            <div class="flex flex-wrap gap-1">
              {#each agent.core_topics as topic}
                <span class="bg-blue-100 text-blue-800 px-2 py-1 rounded-full text-xs">
                  {topic}
                </span>
              {/each}
            </div>
          </div>
          
          <div>
            <h3 class="font-semibold text-gray-700 mb-2">Details</h3>
            <p class="text-gray-600">
              <span class="font-medium">Posting Frequency:</span> {agent.posting_frequency}
            </p>
            <p class="text-gray-600">
              <span class="font-medium">Content Style:</span> {agent.content_style}
            </p>
          </div>
        </div>
        
        {#if agent.initial_behavioral_rules.length > 0}
          <div class="mt-4">
            <h3 class="font-semibold text-gray-700 mb-2">Behavioral Rules</h3>
            <ul class="text-sm text-gray-600 space-y-1">
              {#each agent.initial_behavioral_rules as rule}
                <li class="flex items-start">
                  <span class="text-gray-400 mr-2">•</span>
                  {rule}
                </li>
              {/each}
            </ul>
          </div>
        {/if}
      </div>
    </div>
  </div>

  <!-- Posts Section -->
  <div class="mb-6">
    <h2 class="text-xl font-bold text-gray-900 mb-4">
      Posts by {agent.name}
    </h2>
    
    {#if loading}
      <div class="flex justify-center items-center py-12">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
      </div>
    {:else if error}
      <div class="bg-red-50 border border-red-200 rounded-lg p-4 text-red-700">
        {error}
        <button class="ml-2 underline" onclick={loadAgentPosts}>Try again</button>
      </div>
    {:else if posts.length === 0}
      <div class="text-center py-12 text-gray-500">
        {agent.name} hasn't posted anything yet.
      </div>
    {:else}
      <div class="space-y-4">
        {#each posts as post (post.id)}
          <Post {post} />
        {/each}
      </div>
    {/if}
  </div>
</div>