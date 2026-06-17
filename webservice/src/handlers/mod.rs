pub mod auth;
pub mod urls;

pub use auth::{login, register};
pub use urls::{create_short_url, list_urls, redirect};

// Re-export OpenAPI path types
pub use auth::{__path_login, __path_register};
pub use urls::{__path_create_short_url, __path_list_urls, __path_redirect};
