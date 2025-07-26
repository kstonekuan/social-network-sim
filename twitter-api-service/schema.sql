-- AI Social Network Simulation Database Schema
-- This file contains the complete database schema for the project

-- Agents table: Stores AI agent profiles
CREATE TABLE agents (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    persona_summary TEXT NOT NULL,
    core_topics TEXT[] NOT NULL,
    posting_frequency VARCHAR(50) NOT NULL,
    content_style TEXT NOT NULL,
    initial_behavioral_rules TEXT[] NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Posts table: Stores content created by agents
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    agent_id INTEGER NOT NULL REFERENCES agents(id),
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Likes table: Tracks which agents liked which posts
CREATE TABLE likes (
    id SERIAL PRIMARY KEY,
    agent_id INTEGER NOT NULL REFERENCES agents(id),
    post_id INTEGER NOT NULL REFERENCES posts(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(agent_id, post_id) -- Prevent duplicate likes
);

-- Followers table: Tracks agent follow relationships
CREATE TABLE followers (
    id SERIAL PRIMARY KEY,
    follower_id INTEGER NOT NULL REFERENCES agents(id),
    followed_id INTEGER NOT NULL REFERENCES agents(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(follower_id, followed_id), -- Prevent duplicate follows
    CHECK(follower_id != followed_id) -- Prevent self-follows
);

-- Comments table: Stores comments on posts
CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    agent_id INTEGER NOT NULL REFERENCES agents(id),
    post_id INTEGER NOT NULL REFERENCES posts(id),
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Reposts table: Tracks reposts/retweets with optional comments
CREATE TABLE reposts (
    id SERIAL PRIMARY KEY,
    agent_id INTEGER NOT NULL REFERENCES agents(id),
    original_post_id INTEGER NOT NULL REFERENCES posts(id),
    comment TEXT, -- Optional comment when reposting
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(agent_id, original_post_id) -- Prevent duplicate reposts
);

-- Performance indexes
CREATE INDEX idx_posts_agent_id ON posts(agent_id);
CREATE INDEX idx_posts_created_at ON posts(created_at DESC);
CREATE INDEX idx_likes_post_id ON likes(post_id);
CREATE INDEX idx_likes_agent_id ON likes(agent_id);
CREATE INDEX idx_followers_follower_id ON followers(follower_id);
CREATE INDEX idx_followers_followed_id ON followers(followed_id);
CREATE INDEX idx_comments_post_id ON comments(post_id);
CREATE INDEX idx_comments_created_at ON comments(created_at DESC);
CREATE INDEX idx_reposts_original_post_id ON reposts(original_post_id);
CREATE INDEX idx_reposts_created_at ON reposts(created_at DESC);