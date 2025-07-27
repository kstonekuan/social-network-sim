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
pub struct CommentWithAgent {
    id: i32,
    agent_id: i32,
    agent_name: String,
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
    match sqlx::query!(
        r#"
        SELECT 
            p.id, 
            p.agent_id,
            a.name as agent_name,
            p.content, 
            p.created_at,
            COALESCE(like_counts.like_count, 0) as likes_count,
            COALESCE(comment_counts.comment_count, 0) as comments_count,
            COALESCE(repost_counts.repost_count, 0) as reposts_count
        FROM posts p
        JOIN agents a ON p.agent_id = a.id
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
        Ok(rows) => {
            let posts: Vec<PostWithEngagement> = rows
                .into_iter()
                .map(|row| PostWithEngagement {
                    id: row.id,
                    agent_id: row.agent_id,
                    agent_name: row.agent_name,
                    content: row.content,
                    created_at: Some(row.created_at),
                    likes_count: row.likes_count.unwrap_or(0),
                    comments_count: row.comments_count.unwrap_or(0),
                    reposts_count: row.reposts_count.unwrap_or(0),
                })
                .collect();
            (StatusCode::OK, Json(posts)).into_response()
        }
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
    match sqlx::query!(
        r#"
        SELECT c.id, c.agent_id, c.post_id, c.content, c.created_at, a.name as agent_name
        FROM comments c
        JOIN agents a ON c.agent_id = a.id
        WHERE c.post_id = $1
        ORDER BY c.created_at ASC
        "#,
        post_id
    )
    .fetch_all(&db_pool)
    .await
    {
        Ok(rows) => {
            let comments: Vec<CommentWithAgent> = rows
                .into_iter()
                .map(|row| CommentWithAgent {
                    id: row.id,
                    agent_id: row.agent_id,
                    agent_name: row.agent_name,
                    post_id: row.post_id,
                    content: row.content,
                    created_at: row.created_at,
                })
                .collect();
            (StatusCode::OK, Json(comments)).into_response()
        }
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

#[derive(Serialize)]
pub struct PostWithEngagement {
    id: i32,
    agent_id: i32,
    agent_name: String,
    content: String,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
    likes_count: i64,
    comments_count: i64,
    reposts_count: i64,
}

pub async fn get_timeline(
    State(db_pool): State<PgPool>,
    Path(agent_id): Path<i32>,
) -> impl IntoResponse {
    match sqlx::query!(
        r#"
        SELECT 
            p.id, 
            p.agent_id,
            a.name as agent_name,
            p.content, 
            p.created_at,
            COALESCE(like_counts.like_count, 0) as likes_count,
            COALESCE(comment_counts.comment_count, 0) as comments_count,
            COALESCE(repost_counts.repost_count, 0) as reposts_count
        FROM posts p
        JOIN agents a ON p.agent_id = a.id
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
        WHERE p.agent_id = $1
        ORDER BY p.created_at DESC
        LIMIT 100
        "#,
        agent_id
    )
    .fetch_all(&db_pool)
    .await
    {
        Ok(rows) => {
            let posts: Vec<PostWithEngagement> = rows
                .into_iter()
                .map(|row| PostWithEngagement {
                    id: row.id,
                    agent_id: row.agent_id,
                    agent_name: row.agent_name,
                    content: row.content,
                    created_at: Some(row.created_at),
                    likes_count: row.likes_count.unwrap_or(0),
                    comments_count: row.comments_count.unwrap_or(0),
                    reposts_count: row.reposts_count.unwrap_or(0),
                })
                .collect();
            (StatusCode::OK, Json(posts)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch timeline: {e}"),
        )
            .into_response(),
    }
}

#[derive(Serialize)]
pub struct ActivityItem {
    pub id: i32,
    pub activity_type: String, // "post", "like", "comment", "repost", "follow"
    pub agent_id: i32,
    pub agent_name: String,
    pub content: Option<String>,
    pub target_agent_id: Option<i32>,
    pub target_agent_name: Option<String>,
    pub post_id: Option<i32>,
    pub post_content: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn get_activity_feed(State(db_pool): State<PgPool>) -> impl IntoResponse {
    let activities = match get_unified_activities(&db_pool).await {
        Ok(activities) => activities,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch activities: {e}"),
            )
                .into_response()
        }
    };

    (StatusCode::OK, Json(activities)).into_response()
}

async fn get_unified_activities(db_pool: &PgPool) -> Result<Vec<ActivityItem>, sqlx::Error> {
    let mut activities = Vec::new();

    // Get posts
    let posts = sqlx::query!(
        r#"
        SELECT p.id, p.agent_id, p.content, p.created_at, a.name as agent_name
        FROM posts p
        JOIN agents a ON p.agent_id = a.id
        ORDER BY p.created_at DESC
        LIMIT 50
        "#
    )
    .fetch_all(db_pool)
    .await?;

    for post in posts {
        activities.push(ActivityItem {
            id: post.id,
            activity_type: "post".to_string(),
            agent_id: post.agent_id,
            agent_name: post.agent_name,
            content: Some(post.content),
            target_agent_id: None,
            target_agent_name: None,
            post_id: Some(post.id),
            post_content: None,
            created_at: Some(post.created_at),
        });
    }

    // Get likes
    let likes = sqlx::query!(
        r#"
        SELECT l.id, l.agent_id, l.post_id, l.created_at,
               a.name as agent_name,
               p.content as post_content,
               pa.name as post_agent_name
        FROM likes l
        JOIN agents a ON l.agent_id = a.id
        JOIN posts p ON l.post_id = p.id
        JOIN agents pa ON p.agent_id = pa.id
        ORDER BY l.created_at DESC
        LIMIT 50
        "#
    )
    .fetch_all(db_pool)
    .await?;

    for like in likes {
        activities.push(ActivityItem {
            id: like.id,
            activity_type: "like".to_string(),
            agent_id: like.agent_id,
            agent_name: like.agent_name,
            content: None,
            target_agent_id: None,
            target_agent_name: Some(like.post_agent_name),
            post_id: Some(like.post_id),
            post_content: Some(like.post_content),
            created_at: Some(like.created_at),
        });
    }

    // Get comments
    let comments = sqlx::query!(
        r#"
        SELECT c.id, c.agent_id, c.post_id, c.content, c.created_at,
               a.name as agent_name,
               p.content as post_content,
               pa.name as post_agent_name
        FROM comments c
        JOIN agents a ON c.agent_id = a.id
        JOIN posts p ON c.post_id = p.id
        JOIN agents pa ON p.agent_id = pa.id
        ORDER BY c.created_at DESC
        LIMIT 50
        "#
    )
    .fetch_all(db_pool)
    .await?;

    for comment in comments {
        activities.push(ActivityItem {
            id: comment.id,
            activity_type: "comment".to_string(),
            agent_id: comment.agent_id,
            agent_name: comment.agent_name,
            content: Some(comment.content),
            target_agent_id: None,
            target_agent_name: Some(comment.post_agent_name),
            post_id: Some(comment.post_id),
            post_content: Some(comment.post_content),
            created_at: comment.created_at,
        });
    }

    // Get reposts
    let reposts = sqlx::query!(
        r#"
        SELECT r.id, r.agent_id, r.original_post_id, r.comment, r.created_at,
               a.name as agent_name,
               p.content as post_content,
               pa.name as post_agent_name
        FROM reposts r
        JOIN agents a ON r.agent_id = a.id
        JOIN posts p ON r.original_post_id = p.id
        JOIN agents pa ON p.agent_id = pa.id
        ORDER BY r.created_at DESC
        LIMIT 50
        "#
    )
    .fetch_all(db_pool)
    .await?;

    for repost in reposts {
        activities.push(ActivityItem {
            id: repost.id,
            activity_type: "repost".to_string(),
            agent_id: repost.agent_id,
            agent_name: repost.agent_name,
            content: repost.comment,
            target_agent_id: None,
            target_agent_name: Some(repost.post_agent_name),
            post_id: Some(repost.original_post_id),
            post_content: Some(repost.post_content),
            created_at: repost.created_at,
        });
    }

    // Get follows
    let follows = sqlx::query!(
        r#"
        SELECT f.id, f.follower_id, f.followed_id, f.created_at,
               fa.name as follower_name,
               foa.name as followed_name
        FROM followers f
        JOIN agents fa ON f.follower_id = fa.id
        JOIN agents foa ON f.followed_id = foa.id
        ORDER BY f.created_at DESC
        LIMIT 50
        "#
    )
    .fetch_all(db_pool)
    .await?;

    for follow in follows {
        activities.push(ActivityItem {
            id: follow.id,
            activity_type: "follow".to_string(),
            agent_id: follow.follower_id,
            agent_name: follow.follower_name,
            content: None,
            target_agent_id: Some(follow.followed_id),
            target_agent_name: Some(follow.followed_name),
            post_id: None,
            post_content: None,
            created_at: Some(follow.created_at),
        });
    }

    // Sort all activities by creation time (most recent first)
    activities.sort_by(|a, b| {
        b.created_at
            .unwrap_or_default()
            .cmp(&a.created_at.unwrap_or_default())
    });

    // Limit to most recent 100 activities
    activities.truncate(100);

    Ok(activities)
}
