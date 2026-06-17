use std::time::Duration;

#[derive(Clone)]
pub struct AppConfig {
    pub jwt_secret: String,
    pub server_addr: String,
    pub token_expiry_secs: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "super-secret-jwt-key-change-in-production".to_string(),
            server_addr: "0.0.0.0:3000".to_string(),
            token_expiry_secs: 86400, // 24 hours
        }
    }
}

impl AppConfig {
    pub fn token_expiry(&self) -> Duration {
        Duration::from_secs(self.token_expiry_secs)
    }
}
