use std::{collections::HashMap, str::FromStr, env::vars};

pub enum Error {
    InvalidValue
}

pub trait Environment {
    fn try_get<T: FromStr>(&self, key: &str) -> Result<Option<T>, Error> 
    {
        match self.get::<String>(key) {
            Some(value) => Ok(Some(
                value.parse()
                .map_err(|_| Error::InvalidValue)?
            )),
            None => Ok(None),
        }
    }

    fn try_get_or<T: FromStr>(&self, key: &str, default: T) -> T {
        match self.try_get(key) {
            Ok(Some(value)) => value,
            _ => default,
        }
    }

    fn get<T: FromStr>(&self, key: &str) -> Option<T>;

    fn get_or<T: FromStr>(&self, key: &str, default: T) -> T {
        match self.get(key) {
            Some(value) => value,
            None => default,
        }
    }
}

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

pub struct DotEnv {
    pub(crate) data: HashMap<String, String>,
}

impl Default for DotEnv {
    fn default() -> Self {
        Self {
            data: dotenvy::dotenv_iter().unwrap().filter_map(|r|r.ok()).collect(),
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