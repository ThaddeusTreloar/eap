use std::str::FromStr;

use aws_sdk_secretsmanager::Client;
use serde_json::{Map, Value};
use base64::prelude::{BASE64_STANDARD, Engine};

use super::Environment;

pub struct AwsSecret {
    pub(crate) data: Map<String, Value>,
}

#[derive(Debug, thiserror::Error)]
pub enum AwsSecretError {
    #[error("SecretLoadError: {0}")]
    SecretLoadError(String),
    #[error("Base64DecodeError: {0}")]
    Base64DecodeError(String),
    #[error("Utf8DecodeError: {0}")]
    Utf8DecodeError(String),
    #[error("PullSecretError: {0}")]
    PullSecretError(String),
    #[error(transparent)]
    DeserError(#[from] serde_json::Error),
    #[error("JsonError: {0}")]
    JsonError(String),
    #[error("UnknownError: {0}")]
    UnknownError(String),
}

impl AwsSecret {
    pub async fn from_ssm_client(
        client: Client,
        secret_name: &str,
    ) -> Result<Self, AwsSecretError> {
        let client_response = client
            .get_secret_value()
            .secret_id(secret_name)
            .send()
            .await
            .map_err(|e| AwsSecretError::SecretLoadError(e.to_string()))?;

        let mut maybe_raw_secret = Option::from(String::new());
        
        if let Some(secret_string) = client_response.secret_string() {
            maybe_raw_secret = Some(secret_string.into());
        } else if let Some(secret_binary) = client_response.secret_binary() {
            let decoded = BASE64_STANDARD
                .decode(secret_binary)
                .map_err(|e| AwsSecretError::Base64DecodeError(e.to_string()))?;

            maybe_raw_secret = Some(
                String::from_utf8(decoded)
                    .map_err(|e| AwsSecretError::Utf8DecodeError(e.to_string()))?,
            );
        }

        let raw_secret = maybe_raw_secret.map_or(
            Err(AwsSecretError::UnknownError("Retrieved secret neither Blol nor &str.".to_string())), 
            Ok
        )?;

        let json_value: Value = serde_json::from_str(raw_secret.as_str())?;

        let map = match json_value.as_object() {
            Some(map) => map.clone(),
            None => {
                return Err(AwsSecretError::JsonError(String::from(
                    "Root json object not object(Map<String, Value>).",
                )))
            }
        };

        Ok(Self { data: map })
    }
}

impl Environment for AwsSecret {
    fn get<T: FromStr>(&self, key: &str) -> Option<T> {
        if let Some(raw_value) = self.data.get(key) {
            FromStr::from_str(raw_value.to_string().as_str()).ok()
        } else {
            None
        }
    }
}
