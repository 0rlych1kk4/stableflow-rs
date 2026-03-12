use crate::client::StableflowClient;
use crate::error::StableflowError;

use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine as _;
use rsa::pkcs8::DecodePublicKey;
use rsa::rand_core::OsRng;
use rsa::{Oaep, RsaPublicKey};
use serde::Deserialize;
use sha2::Sha256;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityPublicKeyResponse {
    pub data: EntityPublicKeyData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityPublicKeyData {
    pub public_key: String,
}

/// Fetches the Circle entity public key from:
/// GET /v1/w3s/config/entity/publicKey
pub async fn fetch_entity_public_key(
    client: &StableflowClient,
) -> Result<String, StableflowError> {
    let url = format!(
        "{}/v1/w3s/config/entity/publicKey",
        client.config().base_url()
    );

    let response = client.http.get(url).send().await?;
    let parsed: EntityPublicKeyResponse = crate::http::parse_json_or_error(response).await?;

    Ok(parsed.data.public_key)
}

/// Generates a fresh entitySecretCiphertext for a single request.
///
/// Circle requires:
/// - a hex-encoded 32-byte Entity Secret (64 hex chars)
/// - encryption with the entity public key
/// - RSA-OAEP with SHA-256
/// - base64 output
/// - a unique ciphertext for each API request
pub fn generate_entity_secret_ciphertext(
    entity_secret_hex: &str,
    entity_public_key_pem: &str,
) -> Result<String, StableflowError> {
    let secret = entity_secret_hex.trim();

    if secret.len() != 64 {
        return Err(StableflowError::InvalidConfig(
            "CIRCLE_ENTITY_SECRET must be a 32-byte hex string (64 hex chars)".into(),
        ));
    }

    if !secret.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(StableflowError::InvalidConfig(
            "CIRCLE_ENTITY_SECRET must contain only hex characters".into(),
        ));
    }

    let public_key = RsaPublicKey::from_public_key_pem(entity_public_key_pem).map_err(|e| {
        StableflowError::InvalidConfig(format!("failed to parse entity public key PEM: {e}"))
    })?;

    // Circle’s sample code encrypts the 32-byte entity secret value.
    // We decode the 64-char hex into the raw 32 bytes here.
    let secret_bytes = hex::decode(secret).map_err(|e| {
        StableflowError::InvalidConfig(format!("failed to decode CIRCLE_ENTITY_SECRET hex: {e}"))
    })?;

    if secret_bytes.len() != 32 {
        return Err(StableflowError::InvalidConfig(
            "decoded CIRCLE_ENTITY_SECRET must be exactly 32 bytes".into(),
        ));
    }

    let padding = Oaep::new::<Sha256>();
    let mut rng = OsRng;

    let ciphertext = public_key
        .encrypt(&mut rng, padding, &secret_bytes)
        .map_err(|e| {
            StableflowError::InvalidConfig(format!(
                "failed to encrypt entity secret with RSA-OAEP-SHA256: {e}"
            ))
        })?;

    Ok(BASE64_STANDARD.encode(ciphertext))
}

/// Convenience helper:
/// - loads the entity secret from env via `client.entity_secret()`
/// - fetches the entity public key from Circle
/// - generates a fresh entitySecretCiphertext
pub async fn generate_entity_secret_ciphertext_for_request(
    client: &StableflowClient,
) -> Result<String, StableflowError> {
    let entity_secret = client.entity_secret()?;
    let public_key_pem = fetch_entity_public_key(client).await?;

    generate_entity_secret_ciphertext(entity_secret.as_str(), &public_key_pem)
}
