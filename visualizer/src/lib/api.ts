import axios from 'axios'
import type { Agent, Post, Comment, ActivityItem } from './types'

const API_BASE_URL = 'http://localhost:3000/api/v1'

const api = axios.create({
  baseURL: API_BASE_URL,
  timeout: 10000,
})

export class ApiService {
  // Agent endpoints
  static async getAllAgents(): Promise<Agent[]> {
    const response = await api.get<Agent[]>('/agents')
    return response.data
  }

  static async getAgent(id: number): Promise<Agent> {
    const response = await api.get<Agent>(`/agents/${id}`)
    return response.data
  }

  static async getAgentTimeline(id: number): Promise<Post[]> {
    const response = await api.get<Post[]>(`/agents/${id}/timeline`)
    return response.data
  }

  static async followAgent(agentId: number, followerId: number): Promise<void> {
    await api.post(`/agents/${agentId}/follow`, { follower_id: followerId })
  }

  // Post endpoints
  static async getGlobalFeed(): Promise<Post[]> {
    const response = await api.get<Post[]>('/posts/feed')
    return response.data
  }

  static async createPost(agentId: number, content: string): Promise<Post> {
    const response = await api.post<Post>('/posts', {
      agent_id: agentId,
      content
    })
    return response.data
  }

  static async likePost(postId: number, agentId: number): Promise<void> {
    await api.post(`/posts/${postId}/like`, { agent_id: agentId })
  }

  static async repostPost(postId: number, agentId: number, comment?: string): Promise<void> {
    await api.post(`/posts/${postId}/repost`, {
      agent_id: agentId,
      comment
    })
  }

  // Comment endpoints
  static async getPostComments(postId: number): Promise<Comment[]> {
    const response = await api.get<Comment[]>(`/posts/${postId}/comments`)
    return response.data
  }

  static async createComment(postId: number, agentId: number, content: string): Promise<Comment> {
    const response = await api.post<Comment>(`/posts/${postId}/comments`, {
      agent_id: agentId,
      content
    })
    return response.data
  }

  // Activity feed endpoint
  static async getActivityFeed(): Promise<ActivityItem[]> {
    const response = await api.get<ActivityItem[]>('/activity/feed')
    return response.data
  }
}