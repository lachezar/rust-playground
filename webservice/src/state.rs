use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::config::AppConfig;
use crate::models::{User, ShortUrl};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub users: Arc<RwLock<HashMap<String, User>>>,
    pub urls: Arc<RwLock<HashMap<String, ShortUrl>>>,
    pub urls_by_user: Arc<RwLock<HashMap<Uuid, Vec<String>>>>,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            users: Arc::new(RwLock::new(HashMap::new())),
            urls: Arc::new(RwLock::new(HashMap::new())),
            urls_by_user: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
