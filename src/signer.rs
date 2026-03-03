use base64::{Engine, prelude::BASE64_STANDARD};
use chrono::Utc;
use ed25519_dalek::{SigningKey, VerifyingKey, ed25519::signature::SignerMut};

use crate::BackpackError;

const DEFAULT_WINDOW: u64 = 5000;

pub struct AuthHeaders {
    pub x_api_key: String,
    pub x_signature: String,
    pub x_timestamp: String,
    pub x_window: String,
}

pub struct Signer {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
    api_key_header: String,
}

impl Signer {
    pub fn from_base64(private_key_b64: &str) -> Result<Self, BackpackError> {
        let key_bytes = BASE64_STANDARD
            .decode(private_key_b64.trim())
            .map_err(|e| {
                BackpackError::InvalidApiKey(format!("private key is a valid base64 {}", e))
            })?;

        let key_arrays = key_bytes.try_into().map_err(|e| {
            BackpackError::InvalidApiKey(
                "private key must be exactly 32 bytes (44 base64 chars)".to_string(),
            )
        })?;

        let signing_key = SigningKey::from_bytes(&key_arrays);

        let verifying_key = signing_key.verifying_key();

        let api_key_header = BASE64_STANDARD.encode(verifying_key.as_bytes());

        Ok(Self {
            signing_key,
            verifying_key,
            api_key_header,
        })
    }

    pub fn sign(&self, instruction: &str, params: &str) -> AuthHeaders {
        let timestamp = Utc::now().timestamp_millis() as u64;
        let window = DEFAULT_WINDOW;

        let signing_string = if params.is_empty() {
            format!(
                "instruction={}&timestamp={}&window={}",
                instruction, timestamp, window
            )
        } else {
            format!(
                "instruction={}&{}&timestamp={}&window={}",
                instruction, params, timestamp, window
            )
        };

        let signature = self.signing_key.clone().sign(signing_string.as_bytes());

        let signature_b64 = BASE64_STANDARD.encode(signature.to_bytes());

        AuthHeaders {
            x_api_key: self.api_key_header.clone(),
            x_signature: signature_b64,
            x_timestamp: timestamp.to_string(),
            x_window: window.to_string(),
        }
    }
}
