# Eap (Environment Argument Parser)
!! UNDER CONSTRUCTION !!

A crate for easily creating environment configurations.

Example usage:
```
use eap::environment::{DotEnv, Environment, Local};
use eap::config::Config;

#[derive(eap::Config)]
struct MyConfig {
    #[var(default = 443)]
    port: u16,
}

fn main() {
    let config = MyConfig::parse_env::<Local>();

    println!("Port: {}", config.port);

    let dotenv_config = MyConfig::parse_env::<DotEnv>();

    println!("Port: {}", dotenv_config.port);
}

```

It also provides the ability to build custom backends:
```
pub struct CustomBackend {
    pub(crate) some_api_client: SomeApiClient,
}

impl Environment for CustomBackend {
    fn get<T: FromStr>(&self, key: &str) -> Option<T> {
        self.some_api_client.get(key)
            .map(
                move |value| FromStr::from_str(value.as_str()).ok()
            ).unwrap_or(None)
    }
}
```