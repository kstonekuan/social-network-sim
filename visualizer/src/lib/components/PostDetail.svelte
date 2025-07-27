<script lang="ts">
  import { onMount } from 'svelte'
  import { formatDistanceToNow } from 'date-fns'
  import { ApiService } from '../api'
  import type { Post, Comment } from '../types'

  interface Props {
    post: Post
    onClose: () => void
  }

  let { post, onClose }: Props = $props()
  let comments = $state<Comment[]>([])
  let loading = $state(true)
  let error = $state<string | null>(null)

  onMount(async () => {
    try {
      comments = await ApiService.getPostComments(post.id)
      loading = false
    } catch (err) {
      error = 'Failed to load comments'
      loading = false
      console.error('Error loading comments:', err)
    }
  })

  function formatTimeAgo(dateString: string): string {
    try {
      return formatDistanceToNow(new Date(dateString), { addSuffix: true })
    } catch {
      return 'Unknown time'
    }
  }
</script>

<!-- Modal backdrop -->
<div 
  class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50" 
  onclick={onClose}
  onkeydown={(e) => e.key === 'Escape' && onClose()}
  role="button"
  tabindex="0"
  aria-label="Close modal"
>
  <!-- Modal content -->
  <div 
    class="bg-white dark:bg-gray-800 rounded-lg max-w-2xl w-full mx-4 max-h-[90vh] overflow-hidden transition-colors" 
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.stopPropagation()}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <!-- Header -->
    <div class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700">
      <h2 class="text-lg font-semibold text-gray-900 dark:text-white">Post Details</h2>
      <button 
        class="text-gray-400 dark:text-gray-500 hover:text-gray-600 dark:hover:text-gray-300 text-2xl transition-colors"
        onclick={onClose}
      >
        ×
      </button>
    </div>
    
    <!-- Scrollable content -->
    <div class="overflow-y-auto max-h-[calc(90vh-80px)]">
      <!-- Original Post -->
      <div class="p-6 border-b border-gray-100 dark:border-gray-700">
        <div class="flex items-start space-x-3">
          <!-- Avatar -->
          <div class="w-12 h-12 bg-blue-500 rounded-full flex items-center justify-center text-white font-bold">
            {post.agent_name.charAt(0).toUpperCase()}
          </div>
          
          <div class="flex-1 min-w-0">
            <!-- Header -->
            <div class="flex items-center space-x-2 mb-3">
              <h3 class="font-bold text-gray-900 dark:text-white">
                {post.agent_name}
              </h3>
              <span class="text-gray-500 dark:text-gray-400 text-sm">
                @{post.agent_name.toLowerCase().replace(/\s+/g, '')}
              </span>
              <span class="text-gray-400 dark:text-gray-500 text-sm">·</span>
              <time class="text-gray-500 dark:text-gray-400 text-sm">
                {formatTimeAgo(post.created_at)}
              </time>
            </div>
            
            <!-- Content -->
            <div class="text-gray-900 dark:text-gray-100 text-lg mb-4 whitespace-pre-wrap">
              {post.content}
            </div>
            
            <!-- Engagement stats -->
            <div class="flex items-center space-x-6 text-gray-500 dark:text-gray-400 text-sm">
              <span>{post.comments_count} comments</span>
              <span>{post.reposts_count} reposts</span>
              <span>{post.likes_count} likes</span>
            </div>
          </div>
        </div>
      </div>
      
      <!-- Comments section -->
      <div class="p-6">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          Comments ({post.comments_count})
        </h3>
        
        {#if loading}
          <div class="flex justify-center py-8">
            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
          </div>
        {:else if error}
          <div class="bg-red-50 dark:bg-red-900/50 border border-red-200 dark:border-red-800 rounded-lg p-4 text-red-700 dark:text-red-300">
            {error}
          </div>
        {:else if comments.length === 0}
          <div class="text-center py-8 text-gray-500 dark:text-gray-400">
            No comments yet. Be the first to comment!
          </div>
        {:else}
          <div class="space-y-4">
            {#each comments as comment (comment.id)}
              <div class="flex items-start space-x-3 p-4 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
                <!-- Comment avatar -->
                <div class="w-8 h-8 bg-green-500 rounded-full flex items-center justify-center text-white font-bold text-sm">
                  {comment.agent_name.charAt(0).toUpperCase()}
                </div>
                
                <div class="flex-1 min-w-0">
                  <!-- Comment header -->
                  <div class="flex items-center space-x-2 mb-2">
                    <span class="font-semibold text-gray-900 dark:text-white text-sm">
                      {comment.agent_name}
                    </span>
                    <span class="text-gray-500 dark:text-gray-400 text-xs">
                      {formatTimeAgo(comment.created_at)}
                    </span>
                  </div>
                  
                  <!-- Comment content -->
                  <div class="text-gray-800 dark:text-gray-200 text-sm whitespace-pre-wrap">
                    {comment.content}
                  </div>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>