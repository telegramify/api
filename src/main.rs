use axum::{
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

use telegramify_api::{
    config::Config,
    handlers::{
        auth::telegram_auth,
        files::{process_form, upload_file},
        health::{detailed_health_check, health_check},
        users::{get_user, list_users},
    },
    AppState,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        telegramify_api::handlers::health::health_check,
        telegramify_api::handlers::health::detailed_health_check,
        telegramify_api::handlers::auth::telegram_auth,
        telegramify_api::handlers::users::get_user,
        telegramify_api::handlers::users::list_users,
        telegramify_api::handlers::files::upload_file,
        telegramify_api::handlers::files::process_form,
    ),
    components(
        schemas(
            telegramify_api::handlers::health::HealthResponse,
            telegramify_api::handlers::health::DatabaseHealthResponse,
            telegramify_api::auth::TelegramAuthData,
            telegramify_api::auth::AuthResponse,
            telegramify_api::models::UserResponse,
            telegramify_api::handlers::files::FileUploadResponse,
            telegramify_api::handlers::files::FormDataRequest,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Authentication", description = "Authentication endpoints"),
        (name = "Users", description = "User management endpoints"),
        (name = "Files", description = "File upload endpoints"),
        (name = "Forms", description = "Form processing endpoints"),
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "telegramify_api=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    let server_address = config.server_address();

    // Initialize application state
    let state = AppState::new(config).await?;

    // Build our application with routes
    let app = Router::new()
        // Health endpoints
        .route("/health", get(health_check))
        .route("/health/detailed", get(detailed_health_check))
        // Authentication endpoints
        .route("/auth/telegram", post(telegram_auth))
        // User endpoints
        .route("/users", get(list_users))
        .route("/users/:user_id", get(get_user))
        // File endpoints
        .route("/upload", post(upload_file))
        .route("/form", post(process_form))
        // Swagger UI
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&server_address).await?;

    tracing::info!("🚀 Server starting on http://{}", server_address);
    tracing::info!(
        "📚 API documentation available at http://{}/swagger-ui",
        server_address
    );

    axum::serve(listener, app).await?;

    Ok(())
}
