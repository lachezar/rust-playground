mod auth;
mod config;
mod error;
mod handlers;
mod models;
mod state;

use axum::{
    Router, middleware,
    routing::{get, post},
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::auth::require_auth;
use crate::config::AppConfig;
use crate::handlers::{create_short_url, list_urls, login, redirect, register};
use crate::state::AppState;

/// OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::register,
        handlers::login,
        handlers::create_short_url,
        handlers::redirect,
        handlers::list_urls,
    ),
    components(
        schemas(
            models::RegisterRequest,
            models::LoginRequest,
            models::AuthResponse,
            models::CreateUrlRequest,
            models::CreateUrlResponse,
            models::UrlListItem,
            error::ErrorResponse,
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "urls", description = "URL shortening endpoints")
    ),
    modifiers(&SecurityAddon),
)]
struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new(
                        utoipa::openapi::security::HttpAuthScheme::Bearer,
                    ),
                ),
            );
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "webservice=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize config and state
    let config = AppConfig::default();
    let server_addr = config.server_addr.clone();
    let state = AppState::new(config);

    // Build router
    // Protected routes (require JWT)
    let protected_routes = Router::new()
        .route("/api/urls", post(create_short_url))
        .route("/api/urls", get(list_urls))
        .route_layer(middleware::from_fn_with_state(state.clone(), require_auth));

    // Public routes
    let public_routes = Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/:short_code", get(redirect));

    // Combine all routes
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    // Start server
    let listener = tokio::net::TcpListener::bind(&server_addr).await.unwrap();

    println!("Server running on http://{}", server_addr);
    tracing::info!("Server running on http://{}", server_addr);
    tracing::info!("Swagger UI available at http://{}/swagger-ui", server_addr);
    tracing::info!(server_addr = %server_addr, "Swagger UI available at http:///swagger-ui");

    axum::serve(listener, app).await.unwrap();
}
