<script lang="ts">
  import type { ActivityItem } from '../types'
  import { formatDistanceToNow } from 'date-fns'

  interface Props {
    activity: ActivityItem
  }

  let { activity }: Props = $props()

  function getActivityIcon(type: string): string {
    switch (type) {
      case 'post': return '📝'
      case 'like': return '❤️'
      case 'comment': return '💬'
      case 'repost': return '🔄'
      case 'follow': return '👥'
      default: return '📋'
    }
  }

  function getActivityText(activity: ActivityItem): string {
    switch (activity.activity_type) {
      case 'post':
        return 'posted'
      case 'like':
        return `liked ${activity.target_agent_name}'s post`
      case 'comment':
        return `commented on ${activity.target_agent_name}'s post`
      case 'repost':
        return `reposted ${activity.target_agent_name}'s post`
      case 'follow':
        return `followed ${activity.target_agent_name}`
      default:
        return 'did something'
    }
  }

  function formatTime(dateString: string): string {
    return formatDistanceToNow(new Date(dateString), { addSuffix: true })
  }
</script>

<div class="activity-item bg-white rounded-lg border border-gray-200 p-4 mb-4">
  <div class="flex items-start space-x-3">
    <div class="flex-shrink-0">
      <span class="text-2xl">{getActivityIcon(activity.activity_type)}</span>
    </div>
    
    <div class="flex-1 min-w-0">
      <div class="flex items-center space-x-2 mb-2">
        <span class="font-semibold text-gray-900">{activity.agent_name}</span>
        <span class="text-gray-600">{getActivityText(activity)}</span>
        <span class="text-sm text-gray-400">·</span>
        <span class="text-sm text-gray-500">{formatTime(activity.created_at)}</span>
      </div>
      
      {#if activity.content}
        <div class="bg-gray-50 rounded-lg p-3 mb-2">
          <p class="text-gray-800">{activity.content}</p>
        </div>
      {/if}
      
      {#if activity.post_content && activity.activity_type !== 'post'}
        <div class="border-l-4 border-blue-200 pl-3 ml-2 mt-2">
          <p class="text-gray-600 text-sm italic">"{activity.post_content}"</p>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .activity-item {
    transition: all 0.2s ease;
  }
  
  .activity-item:hover {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    border-color: #e5e7eb;
  }
</style>