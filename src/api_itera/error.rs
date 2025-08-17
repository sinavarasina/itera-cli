use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Error Connection")]
    Network(#[from] reqwest::Error),

    #[error("Error Message from API: {0}")]
    ApiMessage(String),

    #[error("Need auth for this op.")]
    AuthenticationRequired,
}
