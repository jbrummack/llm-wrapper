use image::ImageError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LlmError {
    #[error("{0}")]
    Image(#[from] ImageError),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    DotEnv(#[from] dotenvy::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Http(#[from] reqwest::Error),
}
// serde_json::Error
