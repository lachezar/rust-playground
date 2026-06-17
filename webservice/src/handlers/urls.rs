use axum::{
    Extension, Json,
    extract::{Path, State},
    response::{IntoResponse, Redirect, Response},
};
use rand::Rng;
use rand::distr::Alphanumeric;

use crate::auth::Claims;
use crate::error::{AppError, AppResult, ErrorResponse};
use crate::models::{CreateUrlRequest, CreateUrlResponse, ShortUrl, Url, UrlListItem};
use crate::state::AppState;

/// Create a new short URL (requires authentication)
#[utoipa::path(
    post,
    path = "/api/urls",
    request_body = CreateUrlRequest,
    responses(
        (status = 200, description = "Short URL created successfully", body = CreateUrlResponse),
        (status = 400, description = "Invalid URL", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "urls"
)]
pub async fn create_short_url(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateUrlRequest>,
) -> AppResult<Json<CreateUrlResponse>> {
    // Basic URL validation
    if !payload.long_url.starts_with("http://") && !payload.long_url.starts_with("https://") {
        return Err(AppError::InvalidUrl);
    }

    // Generate unique short code
    let short_code = loop {
        let code: String = rand::rng()
            .sample_iter(Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        let urls = state.urls.read().await;
        if !urls.contains_key(&code) {
            break code;
        }
    };

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let short_url_entry = ShortUrl {
        short_code: short_code.clone(),
        long_url: Url(payload.long_url.clone()),
        user_id: claims.sub,
        created_at: now,
    };

    // Store URL
    state
        .urls
        .write()
        .await
        .insert(short_code.clone(), short_url_entry);

    // Update user's URL list
    state
        .urls_by_user
        .write()
        .await
        .entry(claims.sub)
        .or_insert_with(Vec::new)
        .push(short_code.clone());

    Ok(Json(CreateUrlResponse {
        short_code: short_code.clone(),
        short_url: format!("http://localhost:3000/{}", short_code),
        long_url: payload.long_url,
    }))
}

/// Redirect to the original URL using short code
#[utoipa::path(
    get,
    path = "/{short_code}",
    params(
        ("short_code" = String, Path, description = "Short URL code")
    ),
    responses(
        (status = 303, description = "Redirect to original URL"),
        (status = 404, description = "URL not found", body = ErrorResponse),
    ),
    tag = "urls"
)]
pub async fn redirect(
    State(state): State<AppState>,
    Path(short_code): Path<String>,
) -> AppResult<Response> {
    let urls = state.urls.read().await;
    let short_url = urls.get(&short_code).ok_or(AppError::UrlNotFound)?;
    let long_url = short_url.long_url.clone();
    drop(urls);

    Ok(Redirect::to(&long_url.0).into_response())
}

/// List all URLs for the authenticated user
#[utoipa::path(
    get,
    path = "/api/urls",
    responses(
        (status = 200, description = "List of user's URLs", body = Vec<UrlListItem>),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "urls"
)]
pub async fn list_urls(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> AppResult<Json<Vec<UrlListItem>>> {
    let urls_by_user = state.urls_by_user.read().await;
    let user_short_codes = urls_by_user.get(&claims.sub).cloned().unwrap_or_default();
    drop(urls_by_user);

    let urls = state.urls.read().await;
    let user_urls: Vec<UrlListItem> = user_short_codes
        .iter()
        .filter_map(|code| {
            urls.get(code).map(|url| UrlListItem {
                short_code: url.short_code.clone(),
                long_url: url.long_url.0.clone(),
                created_at: url.created_at,
            })
        })
        .collect();

    Ok(Json(user_urls))
}
