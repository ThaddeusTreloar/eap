use aws_config::BehaviorVersion;

extern crate eap;
use eap::prelude::*;
use eap_base::environment::aws_config::{AwsEnvironment, AwsEnvironmentError};

#[derive(eap::Config)]
struct CustomConfig {
    #[var(default = 443)]
    port: u16,
    #[var(default = "localhost".to_string())]
    host: String,
    #[var(default = false)]
    enable_ssl: bool,
}

#[tokio::main]
async fn main() -> Result<(), AwsEnvironmentError> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;

    let custom_config: CustomConfig = AwsEnvironment::from_sdkconfig(&config, "sdk.test")
        .await?.into();

    println!("port: {}", custom_config.port);
    println!("host: {}", custom_config.host);
    println!("enable_ssl: {}", custom_config.enable_ssl);

    Ok(())
}