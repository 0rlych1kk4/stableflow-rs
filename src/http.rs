use crate::config::StableflowConfig;
use crate::error::StableflowError;
use reqwest::{header, Client, Response};

pub fn build_http_client(config: &StableflowConfig) -> Result<Client, StableflowError> {
    let mut headers = header::HeaderMap::new();

    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&config.bearer_token())
            .map_err(|e| StableflowError::InvalidConfig(format!("bad auth header: {e}")))?,
    );

    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/json"),
    );

    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_str(config.user_agent())
            .map_err(|e| StableflowError::InvalidConfig(format!("bad user-agent: {e}")))?,
    );

    let client = Client::builder()
        .default_headers(headers)
        .https_only(true)
        .timeout(config.timeout())
        .build()?;

    Ok(client)
}

pub async fn parse_json_or_error<T: serde::de::DeserializeOwned>(
    response: Response,
) -> Result<T, StableflowError> {
    let status = response.status();
    let body = response.text().await?;

    if !status.is_success() {
        return Err(StableflowError::ApiStatus {
            status: status.as_u16(),
            body,
        });
    }

    Ok(serde_json::from_str(&body)?)
}
