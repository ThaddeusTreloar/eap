use std::{collections::HashMap, str::FromStr};
use super::Environment;

pub struct DotEnv {
    pub(crate) data: HashMap<String, String>,
}

impl Default for DotEnv {
    fn default() -> Self {
        Self {
            data: dotenvy::dotenv_iter()
                .unwrap()
                .filter_map(|r|r.ok()).collect(),
        }
    }
}

impl Environment for DotEnv {
    fn get<T: FromStr>(&self, key: &str) -> Option<T> {
        self.data.get(&key.to_uppercase())
            .map(
                move |value| FromStr::from_str(value.as_str()).ok()
            ).unwrap_or(None)
    }
}