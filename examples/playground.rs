extern crate eap;
use eap::environment::{DotEnv, Environment, Local};
use eap::config::Config;

#[derive(eap::Config)]
struct CustomConfig {
    #[var(default = 443)]
    port: u16,
    #[var(default = "localhost".to_string())]
    host: String,
    qps_limit: Option<usize>,
}

fn main() {
    let e = Local::default();

    let s: u16 = Local::default().get_or("PORT", 8080);

    println!("{s}");

    let mut config: CustomConfig = e.into();

    println!("PORT: {}", config.port);
    println!("HOST: {}", config.host);

    config = CustomConfig::parse_env::<DotEnv>();
    
    println!("PORT: {}", config.port);
    println!("HOST: {}", config.host);

    config = CustomConfig::parse_env::<Local>();

    println!("PORT: {}", config.port);
    
    match config.qps_limit {
        Some(limit) => println!("QPS_LIMIT: {}", limit),
        None => println!("QPS_LIMIT: None"),
    }
}