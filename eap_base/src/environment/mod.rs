use std::str::FromStr;

pub mod aws_secret;
pub mod aws_config;

mod dotenv;
pub use dotenv::DotEnv;

mod local;
pub use local::Local;

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


