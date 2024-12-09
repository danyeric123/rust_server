mod handlers;
mod models;

use axum::{
    routing::{get, post}, Router,
};
use handlers::{create_user, get_user, healthz};
use axum_server;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::trace::{TraceLayer, DefaultMakeSpan};
use tracing_subscriber;
use tracing::{Span};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load environment variables from .env file
    dotenv().ok();

    // Get the database URL from environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to the database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");
    
    // Build our application with routes
    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/users/:id", get(get_user))
        .route("/users", post(create_user))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true))
                .on_request(|request: &axum::http::Request<_>, _span: &Span| {
                    tracing::info!(
                        method = %request.method(),
                        uri = %request.uri(),
                        headers = ?request.headers(),
                        "started processing request"
                    );
                })
                .on_response(|response: &axum::http::Response<_>, latency: std::time::Duration, _span: &Span| {
                    tracing::info!(
                        status = %response.status(),
                        latency = ?latency,
                        headers = ?response.headers(),
                        "finished processing request"
                    );
                }),
        ).with_state(pool.clone());

    // Bind the address and port
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::info!("Listening on {}", addr);
    if let Err(e) = axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
    {
        tracing::error!("Server error: {}", e);
    }
}