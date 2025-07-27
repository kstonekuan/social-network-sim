<script lang="ts">
  import { formatDistanceToNow } from 'date-fns'
  import type { Post as PostType } from '../types'
  import { ApiService } from '../api'

  interface Props {
    post: PostType
    onShowDetail?: () => void
  }

  let { post, onShowDetail }: Props = $props()


  function formatTimeAgo(dateString: string): string {
    try {
      return formatDistanceToNow(new Date(dateString), { addSuffix: true })
    } catch {
      return 'Unknown time'
    }
  }
</script>

<article class="post-card">
  <div class="flex items-start space-x-3">
    <!-- Avatar placeholder -->
    <div class="w-10 h-10 bg-blue-500 rounded-full flex items-center justify-center text-white font-bold">
      {post.agent_name ? post.agent_name.charAt(0).toUpperCase() : '?'}
    </div>
    
    <div class="flex-1 min-w-0">
      <!-- Header -->
      <div class="flex items-center space-x-2 mb-2">
        <h3 class="font-bold text-gray-900 dark:text-white truncate">
          {post.agent_name || 'Unknown'}
        </h3>
        <span class="text-gray-500 dark:text-gray-400 text-sm">
          @{post.agent_name ? post.agent_name.toLowerCase().replace(/\s+/g, '') : 'unknown'}
        </span>
        <span class="text-gray-400 dark:text-gray-500 text-sm">·</span>
        <time class="text-gray-500 dark:text-gray-400 text-sm">
          {formatTimeAgo(post.created_at)}
        </time>
      </div>
      
      <!-- Content -->
      <div class="text-gray-900 dark:text-gray-100 mb-3 whitespace-pre-wrap">
        {post.content}
      </div>
      
      <!-- Engagement Stats -->
      <div class="flex items-center space-x-6 text-gray-500 dark:text-gray-400">
        <!-- Comments -->
        <button 
          class="flex items-center space-x-2 hover:text-blue-500 transition-colors"
          onclick={onShowDetail}
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path>
          </svg>
          <span class="text-sm">{post.comments_count}</span>
        </button>
        
        <!-- Repost Count -->
        <div class="flex items-center space-x-2">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4"></path>
          </svg>
          <span class="text-sm">{post.reposts_count}</span>
        </div>
        
        <!-- Like Count -->
        <div class="flex items-center space-x-2">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"></path>
          </svg>
          <span class="text-sm">{post.likes_count}</span>
        </div>
      </div>
    </div>
  </div>
</article>