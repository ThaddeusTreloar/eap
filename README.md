# Eap (Environment Parser)
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
We currently provide rudimentary support for aws ssm:
```
#[derive(eap::Config)]
struct CustomConfig {
    #[var(default = 443)]
    port: u16,
    #[var(default = "localhost".to_string())]
    host: String,
}

#[tokio::main]
async fn main() -> Result<(), AwsEnvironmentError> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;

    let client = aws_sdk_ssm::Client::new(&config);

    let aws_environment = AwsEnvironment::from_ssm_client(client, "sdk.test")
        .await?;

    let custom_config = CustomConfig::parse(aws_environment);

    println!("port: {}", custom_config.port);
    println!("host: {}", custom_config.host);

    Ok(())
}
```
Custom backends can also be build by implementing the Environment Trait
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