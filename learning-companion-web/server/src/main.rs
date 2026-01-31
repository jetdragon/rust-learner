mod api;

use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub project_path: String,
    // In-memory storage for module progress and tasks
    pub module_states: Arc<Mutex<HashMap<String, ModuleState>>>,
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ModuleState {
    pub progress: f32,
    pub tasks_completed: HashMap<String, bool>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenv::dotenv().ok();

    let project_path = std::env::var("PROJECT_PATH").unwrap_or_else(|_| "..".to_string());

    let app_state = Arc::new(AppState {
        project_path,
        module_states: Arc::new(Mutex::new(HashMap::new())),
    });

    let app = Router::new()
        .route("/api/modules", get(api::get_modules))
        .route("/api/modules/:id/progress", post(api::update_progress))
        .route(
            "/api/modules/:language/:id/content/:type",
            get(api::get_module_content),
        )
        .route(
            "/api/modules/:language/:id/examples",
            get(api::list_examples),
        )
        .route(
            "/api/modules/:language/:id/examples/:filename",
            get(api::get_example_content),
        )
        .route(
            "/api/practice/:language/:module",
            get(api::get_practice_questions),
        )
        .route(
            "/api/practice/submit/:language/:id",
            post(api::submit_practice),
        )
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

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!(
        "🚀 Learning Companion Web Server running on http://{}",
        addr
    );

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
