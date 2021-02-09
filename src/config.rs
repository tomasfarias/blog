use dotenv;
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub auth_salt: String,
    pub secret_key: String,
    pub database_url: String,
    pub rust_backtrace: u8,
}

lazy_static!{
    pub static ref CONFIG: Config = get_config();
}

fn get_config() -> Config {
    dotenv::dotenv().ok();

    match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Configuration error: {:#?}", error),
    }
}
