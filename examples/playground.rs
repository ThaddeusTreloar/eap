extern crate eap;
use eap::environment::{DotEnv, Environment, Local};
use eap::config::Config;

#[derive(eap::Config)]
struct CustomConfig {
    #[var(default = 443)]
    port: u16,
    #[var(default = "localhost".to_string())]
    host: String,
}

fn main() {
    let e = Local::default();

    let s: u16 = Local::default().get_or("PORT", 8080);

    println!("{s}");

    let config = CustomConfig::parse(e);

    println!("PORT: {}", config.port);
    println!("HOST: {}", config.host);

    let dot_config = CustomConfig::parse(DotEnv::default());

    println!("PORT: {}", dot_config.port);
    println!("HOST: {}", dot_config.host);

    let def_config = CustomConfig::parse_env::<Local>();

    println!("PORT: {}", def_config.port);
}