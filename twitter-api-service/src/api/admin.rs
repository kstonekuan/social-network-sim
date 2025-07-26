use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct CreateAgent {
    pub name: String,
    pub persona_summary: String,
    pub core_topics: Vec<String>,
    pub posting_frequency: String,
    pub content_style: String,
    pub initial_behavioral_rules: Vec<String>,
}

pub async fn create_agent(
    State(db_pool): State<PgPool>,
    Json(payload): Json<CreateAgent>,
) -> impl IntoResponse {
    match sqlx::query!(
        r#"
        INSERT INTO agents (name, persona_summary, core_topics, posting_frequency, content_style, initial_behavioral_rules)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        payload.name,
        payload.persona_summary,
        &payload.core_topics,
        payload.posting_frequency,
        payload.content_style,
        &payload.initial_behavioral_rules
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => (StatusCode::CREATED, "Agent created").into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create agent: {e}"),
        )
            .into_response(),
    }
}

pub async fn reset(State(db_pool): State<PgPool>) -> impl IntoResponse {
    match sqlx::query!("TRUNCATE posts, likes, followers, comments, reposts")
        .execute(&db_pool)
        .await
    {
        Ok(_) => (StatusCode::OK, "Reset successful").into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to reset: {e}"),
        )
            .into_response(),
    }
}
