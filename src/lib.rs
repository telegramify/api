pub mod auth;
pub mod config;
pub mod handlers;
pub mod models;
pub mod utils;

use crate::auth::JwtService;
use crate::config::Config;
use redis::Client as RedisClient;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: RedisClient,
    pub jwt_service: Arc<JwtService>,
    pub config: Arc<Config>,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, anyhow::Error> {
        // Database connection
        let db = PgPool::connect(&config.database_url).await?;

        // Run migrations
        sqlx::migrate!("./migrations").run(&db).await?;

        // Redis connection
        let redis = RedisClient::open(config.redis_url.clone())?;

        // JWT service
        let jwt_service = Arc::new(JwtService::new(&config.jwt_secret));

        // Ensure upload directory exists
        tokio::fs::create_dir_all(&config.upload_dir).await?;

        Ok(Self {
            db,
            redis,
            jwt_service,
            config: Arc::new(config),
        })
    }
}
