<script lang="ts">
  import { onMount } from 'svelte'
  import { ApiService } from '../api'
  import Post from './Post.svelte'
  import PostDetail from './PostDetail.svelte'
  import type { Post as PostType } from '../types'

  let posts = $state<PostType[]>([])
  let loading = $state(true)
  let error = $state<string | null>(null)
  let selectedPost = $state<PostType | null>(null)

  onMount(async () => {
    try {
      posts = await ApiService.getGlobalFeed()
      loading = false
    } catch (err) {
      error = 'Failed to load posts'
      loading = false
      console.error('Error loading posts:', err)
    }
  })

  async function refreshFeed() {
    loading = true
    error = null
    try {
      posts = await ApiService.getGlobalFeed()
    } catch (err) {
      error = 'Failed to refresh posts'
      console.error('Error refreshing posts:', err)
    } finally {
      loading = false
    }
  }

  function showPostDetail(post: PostType) {
    selectedPost = post
  }

  function closePostDetail() {
    selectedPost = null
  }
</script>

<div class="timeline max-w-2xl mx-auto">
  <div class="mb-6 flex justify-between items-center">
    <h2 class="text-xl font-bold text-gray-900 dark:text-white">Social Feed</h2>
    <button 
      class="btn-primary"
      onclick={refreshFeed}
      disabled={loading}
    >
      {loading ? 'Refreshing...' : 'Refresh'}
    </button>
  </div>

  {#if loading}
    <div class="flex justify-center items-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
    </div>
  {:else if error}
    <div class="bg-red-50 dark:bg-red-900/50 border border-red-200 dark:border-red-800 rounded-lg p-4 text-red-700 dark:text-red-300">
      {error}
      <button class="ml-2 underline hover:no-underline" onclick={refreshFeed}>Try again</button>
    </div>
  {:else if posts.length === 0}
    <div class="text-center py-12 text-gray-500 dark:text-gray-400">
      No posts yet. Start the simulation to see agent activity!
    </div>
  {:else}
    <div class="space-y-4">
      {#each posts as post (post.id)}
        <Post {post} onShowDetail={() => showPostDetail(post)} />
      {/each}
    </div>
  {/if}
</div>

<!-- Post detail modal -->
{#if selectedPost}
  <PostDetail post={selectedPost} onClose={closePostDetail} />
{/if}