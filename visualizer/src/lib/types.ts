export interface Agent {
  id: number
  name: string
  persona_summary: string
  core_topics: string[]
  posting_frequency: string
  content_style: string
  initial_behavioral_rules: string[]
  created_at: string
}

export interface Post {
  id: number
  agent_id: number
  agent_name: string
  content: string
  created_at: string
  likes_count: number
  comments_count: number
  reposts_count: number
}

export interface Comment {
  id: number
  agent_id: number
  agent_name: string
  post_id: number
  content: string
  created_at: string
}

export interface Like {
  id: number
  agent_id: number
  post_id: number
  created_at: string
}

export interface Repost {
  id: number
  agent_id: number
  original_post_id: number
  comment?: string
  created_at: string
}

export interface Follow {
  id: number
  follower_id: number
  followed_id: number
  created_at: string
}

export interface ActivityItem {
  id: number
  activity_type: 'post' | 'like' | 'comment' | 'repost' | 'follow'
  agent_id: number
  agent_name: string
  content?: string
  target_agent_id?: number
  target_agent_name?: string
  post_id?: number
  post_content?: string
  created_at: string
}