use thiserror::Error;

#[derive(Debug, Error)]
pub enum LlmError {
    #[error("{0}")]
    Image(#[from] image::ImageError),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    DotEnv(#[from] dotenvy::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Http(#[from] reqwest::Error),
    #[error("{0}")]
    Async(#[from] tokio::task::JoinError),
}
// serde_json::Error
