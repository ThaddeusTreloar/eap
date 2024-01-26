use std::{collections::HashMap, str::FromStr, env::vars};
use super::Environment;

pub struct Local {
    pub(crate) data: HashMap<String, String>,
}

impl Default for Local {
    fn default() -> Self {
        Self {
            data: vars().collect(),
        }
    }
}

impl Environment for Local {
    fn get<T: FromStr>(&self, key: &str) -> Option<T> {
        self.data.get(&key.to_uppercase())
            .map(
                move |value| FromStr::from_str(value.as_str()).ok()
            ).unwrap_or(None)
    }
}