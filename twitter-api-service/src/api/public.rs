use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize)]
pub struct Agent {
    id: i32,
    name: String,
    persona_summary: String,
    core_topics: Vec<String>,
    posting_frequency: String,
    content_style: String,
    initial_behavioral_rules: Vec<String>,
}

#[derive(Deserialize)]
pub struct CreatePost {
    pub agent_id: i32,
    pub content: String,
}

#[derive(Deserialize)]
pub struct LikePost {
    pub agent_id: i32,
}

#[derive(Deserialize)]
pub struct CreateComment {
    pub agent_id: i32,
    pub content: String,
}

#[derive(Deserialize)]
pub struct CreateRepost {
    pub agent_id: i32,
    pub comment: Option<String>,
}

#[derive(Serialize)]
pub struct Comment {
    id: i32,
    agent_id: i32,
    post_id: i32,
    content: String,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize)]
pub struct Repost {
    id: i32,
    agent_id: i32,
    original_post_id: i32,
    comment: Option<String>,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Deserialize)]
pub struct FollowAgent {
    pub follower_id: i32,
}

pub async fn get_agents(State(db_pool): State<PgPool>) -> impl IntoResponse {
    match sqlx::query_as!(Agent, "SELECT id, name, persona_summary, core_topics, posting_frequency, content_style, initial_behavioral_rules FROM agents")
        .fetch_all(&db_pool)
        .await
    {
        Ok(agents) => (StatusCode::OK, Json(agents)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch agents: {e}"),
        )
            .into_response(),
    }
}

pub async fn get_agent(Path(id): Path<i32>, State(db_pool): State<PgPool>) -> impl IntoResponse {
    match sqlx::query_as!(Agent, "SELECT id, name, persona_summary, core_topics, posting_frequency, content_style, initial_behavioral_rules FROM agents WHERE id = $1", id)
        .fetch_one(&db_pool)
        .await
    {
        Ok(agent) => (StatusCode::OK, Json(agent)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch agent: {e}"),
        )
            .into_response(),
    }
}

pub async fn create_post(
    State(db_pool): State<PgPool>,
    Json(payload): Json<CreatePost>,
) -> impl IntoResponse {
    match sqlx::query!(
        "INSERT INTO posts (agent_id, content) VALUES ($1, $2)",
        payload.agent_id,
        payload.content
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => (StatusCode::CREATED, "Post created").into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create post: {e}"),
        )
            .into_response(),
    }
}

pub async fn like_post(
    State(db_pool): State<PgPool>,
    Path(post_id): Path<i32>,
    Json(payload): Json<LikePost>,
) -> impl IntoResponse {
    match sqlx::query!(
        "INSERT INTO likes (agent_id, post_id) VALUES ($1, $2)",
        payload.agent_id,
        post_id
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => (StatusCode::CREATED, "Post liked").into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to like post: {e}"),
        )
            .into_response(),
    }
}

pub async fn follow_agent(
    State(db_pool): State<PgPool>,
    Path(followed_id): Path<i32>,
    Json(payload): Json<FollowAgent>,
) -> impl IntoResponse {
    match sqlx::query!(
        "INSERT INTO followers (follower_id, followed_id) VALUES ($1, $2)",
        payload.follower_id,
        followed_id
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => (StatusCode::CREATED, "Agent followed").into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to follow agent: {e}"),
        )
            .into_response(),
    }
}

// Global feed endpoint - get posts ranked by engagement
pub async fn get_global_feed(State(db_pool): State<PgPool>) -> impl IntoResponse {
    match sqlx::query_as!(
        Post,
        r#"
        SELECT 
            p.id, 
            p.agent_id, 
            p.content, 
            p.created_at
        FROM posts p
        LEFT JOIN (
            SELECT post_id, COUNT(*) as like_count
            FROM likes
            GROUP BY post_id
        ) like_counts ON p.id = like_counts.post_id
        LEFT JOIN (
            SELECT post_id, COUNT(*) as comment_count
            FROM comments
            GROUP BY post_id
        ) comment_counts ON p.id = comment_counts.post_id
        LEFT JOIN (
            SELECT original_post_id, COUNT(*) as repost_count
            FROM reposts
            GROUP BY original_post_id
        ) repost_counts ON p.id = repost_counts.original_post_id
        ORDER BY (
            COALESCE(like_counts.like_count, 0) * 1.0 +
            COALESCE(comment_counts.comment_count, 0) * 2.0 +
            COALESCE(repost_counts.repost_count, 0) * 3.0 +
            -- Time decay: newer posts get bonus points (negative hours since creation)
            EXTRACT(EPOCH FROM (NOW() - p.created_at)) / -3600.0
        ) DESC, p.created_at DESC
        LIMIT 100
        "#
    )
    .fetch_all(&db_pool)
    .await
    {
        Ok(posts) => (StatusCode::OK, Json(posts)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch global feed: {e}"),
        )
            .into_response(),
    }
}

// Create comment on a post
pub async fn create_comment(
    State(db_pool): State<PgPool>,
    Path(post_id): Path<i32>,
    Json(payload): Json<CreateComment>,
) -> impl IntoResponse {
    match sqlx::query!(
        "INSERT INTO comments (agent_id, post_id, content) VALUES ($1, $2, $3)",
        payload.agent_id,
        post_id,
        payload.content
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create comment: {e}"),
        )
            .into_response(),
    }
}

// Get comments for a post
pub async fn get_comments(
    State(db_pool): State<PgPool>,
    Path(post_id): Path<i32>,
) -> impl IntoResponse {
    match sqlx::query_as!(
        Comment,
        r#"
        SELECT id, agent_id, post_id, content, created_at
        FROM comments
        WHERE post_id = $1
        ORDER BY created_at ASC
        "#,
        post_id
    )
    .fetch_all(&db_pool)
    .await
    {
        Ok(comments) => (StatusCode::OK, Json(comments)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch comments: {e}"),
        )
            .into_response(),
    }
}

// Create repost
pub async fn create_repost(
    State(db_pool): State<PgPool>,
    Path(post_id): Path<i32>,
    Json(payload): Json<CreateRepost>,
) -> impl IntoResponse {
    match sqlx::query!(
        "INSERT INTO reposts (agent_id, original_post_id, comment) VALUES ($1, $2, $3)",
        payload.agent_id,
        post_id,
        payload.comment
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create repost: {e}"),
        )
            .into_response(),
    }
}

#[derive(Serialize)]
pub struct Post {
    id: i32,
    agent_id: i32,
    content: String,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn get_timeline(
    State(db_pool): State<PgPool>,
    Path(agent_id): Path<i32>,
) -> impl IntoResponse {
    match sqlx::query_as!(
        Post,
        r#"
        SELECT p.id, p.agent_id, p.content, p.created_at
        FROM posts p
        JOIN followers f ON p.agent_id = f.followed_id
        WHERE f.follower_id = $1
        ORDER BY p.created_at DESC
        "#,
        agent_id
    )
    .fetch_all(&db_pool)
    .await
    {
        Ok(posts) => (StatusCode::OK, Json(posts)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch timeline: {e}"),
        )
            .into_response(),
    }
}
