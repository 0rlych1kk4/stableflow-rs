use thiserror::Error;

#[derive(Debug, Error)]
pub enum StableflowError {
    #[error("http transport error: {0}")]
    HttpTransport(#[from] reqwest::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("api returned status {status}: {body}")]
    ApiStatus { status: u16, body: String },

    #[error("invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("webhook verification failed: {0}")]
    WebhookVerification(String),

    #[error("missing required header: {0}")]
    MissingHeader(&'static str),
}
