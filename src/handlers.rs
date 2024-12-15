use crate::models::{CreateUser, User};
use axum::{extract::{Path, State}, http::StatusCode, Json};
use tracing::{error, info};
use sqlx::PgPool;

pub async fn healthz() -> &'static str {
    info!("Health check endpoint called");
    "OK"
}

pub async fn get_user(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<Json<User>, StatusCode> {
    info!("Fetching user with id: {}", id);
    
    let user = sqlx::query_as!(
          User,
          "SELECT id, name, email FROM users WHERE id = $1",
          id
      )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        error!("Failed to fetch user: {:?}", e);
        StatusCode::NOT_FOUND
    })?;

    info!("User found: {:?}", user);
    Ok(Json(user))
}

pub async fn create_user(State(pool): State<PgPool>, Json(payload): Json<CreateUser>) -> Result<Json<User>, StatusCode> {
    info!("Creating user with payload: {:?}", payload);

    let user = sqlx::query_as!(
      User,
      "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
      payload.name,
      payload.email
  ).fetch_one(&pool)
    .await
    .map_err(|e| {
        error!("Failed to create user: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;


    info!("User created: {:?}", user);
    Ok(Json(user))
}
