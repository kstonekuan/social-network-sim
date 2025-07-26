use axum::{
    middleware::from_fn,
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod api;
mod auth_middleware;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    // Admin routes with authentication
    let admin_routes = Router::new()
        .route("/api/v1/admin/agents", post(api::admin::create_agent))
        .route("/api/v1/admin/reset", post(api::admin::reset))
        .layer(from_fn(auth_middleware::admin_auth))
        .with_state(db_pool.clone());

    // Public routes without authentication
    let public_routes = Router::new()
        .route("/api/v1/agents", get(api::public::get_agents))
        .route("/api/v1/agents/{id}", get(api::public::get_agent))
        .route("/api/v1/posts", post(api::public::create_post))
        .route("/api/v1/posts/feed", get(api::public::get_global_feed))
        .route("/api/v1/posts/{id}/like", post(api::public::like_post))
        .route(
            "/api/v1/posts/{id}/comments",
            post(api::public::create_comment),
        )
        .route(
            "/api/v1/posts/{id}/comments",
            get(api::public::get_comments),
        )
        .route(
            "/api/v1/posts/{id}/repost",
            post(api::public::create_repost),
        )
        .route(
            "/api/v1/agents/{id}/follow",
            post(api::public::follow_agent),
        )
        .route(
            "/api/v1/agents/{id}/timeline",
            get(api::public::get_timeline),
        )
        .with_state(db_pool);

    let app = Router::new().merge(admin_routes).merge(public_routes);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
