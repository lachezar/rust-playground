use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};

use crate::auth::jwt::decode_jwt;
use crate::error::AppError;
use crate::state::AppState;

pub async fn require_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;

    let claims = decode_jwt(token, &state.config.jwt_secret)?;

    // Inject claims into request extensions for handlers to use
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}
