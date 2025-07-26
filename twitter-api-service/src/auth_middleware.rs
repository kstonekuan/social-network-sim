use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use std::env;

pub async fn admin_auth(request: Request, next: Next) -> Result<Response, StatusCode> {
    let headers: &HeaderMap = request.headers();

    // Get the expected admin API key from environment
    let expected_key =
        env::var("ADMIN_API_KEY").unwrap_or_else(|_| "default_admin_key".to_string());

    // Check for X-Admin-API-Key header
    if let Some(api_key) = headers.get("X-Admin-API-Key") {
        if let Ok(key_str) = api_key.to_str() {
            if key_str == expected_key {
                return Ok(next.run(request).await);
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
