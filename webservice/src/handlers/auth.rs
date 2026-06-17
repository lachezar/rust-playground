use axum::{extract::State, Json};
use uuid::Uuid;

use crate::auth::{encode_jwt, hash_password, verify_password};
use crate::error::{AppError, AppResult, ErrorResponse};
use crate::models::{AuthResponse, LoginRequest, RegisterRequest, User};
use crate::state::AppState;

/// Register a new user account
#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "User registered successfully", body = AuthResponse),
        (status = 409, description = "Username already exists", body = ErrorResponse),
        (status = 400, description = "Invalid credentials", body = ErrorResponse),
    ),
    tag = "auth"
)]
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<Json<AuthResponse>> {
    // Validate input
    if payload.username.is_empty() || payload.password.len() < 6 {
        return Err(AppError::InvalidCredentials);
    }

    // Hash password in blocking task (CPU-intensive)
    let password_hash = tokio::task::spawn_blocking({
        let password = payload.password.clone();
        move || hash_password(&password)
    })
    .await
    .map_err(|_| AppError::InternalError)??;

    let user_id = Uuid::new_v4();
    let user = User {
        id: user_id,
        username: payload.username.clone(),
        password_hash,
    };

    // Check if user exists and insert
    let mut users = state.users.write().await;
    if users.contains_key(&payload.username) {
        return Err(AppError::UserAlreadyExists);
    }
    users.insert(payload.username.clone(), user.clone());
    drop(users);

    // Initialize empty URL list for user
    state.urls_by_user.write().await.insert(user_id, Vec::new());

    // Generate JWT token
    let token = encode_jwt(
        user_id,
        &payload.username,
        &state.config.jwt_secret,
        state.config.token_expiry_secs,
    )?;

    Ok(Json(AuthResponse {
        token,
        user_id,
        username: payload.username,
    }))
}

/// Login with existing credentials
#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
    ),
    tag = "auth"
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    // Find user
    let users = state.users.read().await;
    let user = users
        .get(&payload.username)
        .cloned()
        .ok_or(AppError::InvalidCredentials)?;
    drop(users);

    // Verify password in blocking task (CPU-intensive)
    let is_valid = tokio::task::spawn_blocking({
        let password = payload.password.clone();
        let hash = user.password_hash.clone();
        move || verify_password(&password, &hash)
    })
    .await
    .map_err(|_| AppError::InternalError)??;

    if !is_valid {
        return Err(AppError::InvalidCredentials);
    }

    // Generate JWT token
    let token = encode_jwt(
        user.id,
        &user.username,
        &state.config.jwt_secret,
        state.config.token_expiry_secs,
    )?;

    Ok(Json(AuthResponse {
        token,
        user_id: user.id,
        username: user.username,
    }))
}
