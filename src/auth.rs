use crate::config::CONFIG;
use actix_web::{error, Error};
use argon2::{self, Config};
use log;

pub fn hash_password(password: &str) -> Result<String, Error> {
    let config = Config {
        secret: &CONFIG.secret_key.as_bytes(),
        ..Default::default()
    };
    argon2::hash_encoded(password.as_bytes(), &CONFIG.auth_salt.as_bytes(), &config)
        .map_err(|err| {
            log::error!("Failed to hash password: {}", err);
            error::ErrorInternalServerError
        })
}

pub fn verify(hash: &str, password: &str) -> Result<bool, Error> {
    argon2::verify_encoded_ext(hash, password.as_bytes(), &CONFIG.secret_key.as_bytes(), &[])
        .map_err(|err| {
            log::error!("Failed to verify password: {}", err);
            error::ErrorUnauthorized
        })
}
