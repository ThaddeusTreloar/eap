use std::str::FromStr;

use aws_sdk_ssm::{types::Parameter, Client};
use serde_json::{Map, Value};

use super::Environment;

pub struct AwsEnvironment {
    pub(crate) data: Map<String, Value>,
}

#[derive(Debug, thiserror::Error)]
pub enum AwsEnvironmentError {
    #[error("ConfigLoadError: {0}")]
    ConfigLoadError(String),
    #[error("PullParamError: {0}")]
    PullParamError(String),
    #[error(transparent)]
    DeserError(#[from] serde_json::Error),
    #[error("JsonError: {0}")]
    JsonError(String),
}

impl AwsEnvironment {
    pub async fn from_ssm_client(
        client: Client,
        config: &str,
    ) -> Result<Self, AwsEnvironmentError> {
        let param = client
            .get_parameter()
            .name(config)
            .send()
            .await
            .map_err(|e| AwsEnvironmentError::ConfigLoadError(e.to_string()))?;

        let raw_config = match param.parameter {
            Some(Parameter {
                value: Some(value), ..
            }) => value,
            _ => {
                return Err(AwsEnvironmentError::PullParamError(String::from(
                    "Failed to pull paramter.",
                )))
            }
        };

        let json_value: Value = serde_json::from_str(raw_config.as_str())?;

        let map = match json_value.as_object() {
            Some(map) => map.clone(),
            None => {
                return Err(AwsEnvironmentError::JsonError(String::from(
                    "Root json object not object(Map<String, Value>).",
                )))
            }
        };

        Ok(Self { data: map })
    }
}

impl Environment for AwsEnvironment {
    fn get<T: FromStr>(&self, key: &str) -> Option<T> {
        if let Some(raw_value) = self.data.get(key) {
            FromStr::from_str(raw_value.to_string().as_str()).ok()
        } else {
            None
        }
    }
}
