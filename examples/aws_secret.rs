use aws_config::BehaviorVersion;

extern crate eap;
use eap::prelude::*;
use eap_base::environment::aws_secret::{AwsSecret, AwsSecretError};

#[derive(eap::Config)]
struct MySecret {
    #[var(default = "".into())]
    client_id: String,
    #[var(default = "".into())]
    client_secret: String,
}

#[tokio::main]
async fn main() -> Result<(), AwsSecretError> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;

    let custom_config: MySecret = AwsSecret::from_sdkconfig(&config, "secrets.test")
        .await?.into();

    println!("client_id: {}", custom_config.client_id);
    println!("client_secret: {}", custom_config.client_secret);

    Ok(())
}