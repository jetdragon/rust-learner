mod api;
mod db;
mod models;

use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize database
    db::init_db()?;

    // Get project path from environment or default to parent directory
    let project_path = std::env::var("PROJECT_PATH").unwrap_or_else(|_| "..".to_string());

    let app_state = Arc::new(api::AppState { project_path });

    // Build router
    let app = Router::new()
        .route("/api/modules", get(api::get_modules))
        .route("/api/modules/:id/progress", post(api::update_progress))
        .route("/api/practice/:module", get(api::get_practice_questions))
        .route("/api/practice/submit/:id", post(api::submit_practice))
        .route("/api/achievements", get(api::get_achievements))
        .route("/api/export", get(api::export_data))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    // Server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!(
        "🚀 Learning Companion Web Server running on http://{}",
        addr
    );

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
