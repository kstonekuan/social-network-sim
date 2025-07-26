use axum::{
    middleware::from_fn,
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tower_http::cors::{Any, CorsLayer};

mod api;
mod auth_middleware;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    println!("Starting Twitter API Service...");
    
    dotenv().ok();
    println!("Environment loaded");

    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => {
            println!("Database URL: {}", url);
            url
        }
        Err(e) => {
            eprintln!("DATABASE_URL must be set: {}", e);
            std::process::exit(1);
        }
    };

    println!("Connecting to database...");
    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Database connection successful");
            pool
        }
        Err(e) => {
            eprintln!("Failed to create database pool: {}", e);
            std::process::exit(1);
        }
    };

    println!("Setting up routes...");
    
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
        .route("/api/v1/activity/feed", get(api::public::get_activity_feed))
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

    let app = Router::new()
        .merge(admin_routes)
        .merge(public_routes)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    println!("Starting server on 0.0.0.0:3000...");
    let listener = match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(listener) => {
            println!("Server listening on port 3000");
            listener
        }
        Err(e) => {
            eprintln!("Failed to bind to port 3000: {}", e);
            std::process::exit(1);
        }
    };
    
    println!("Server ready to accept connections");
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}
