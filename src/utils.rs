use std::env;

use actix_web::error;
use bcrypt::{hash, DEFAULT_COST};


fn hash_password(plain: &str) -> Result<String, > {
    let hashing_cost: u32 = match env::var("HASH_ROUNDS") {
        Ok(cost) => cost.parse().unwrap_or(DEFAULT_COST),
        _ => DEFAULT_COST,
    };

    hash(plain, hashing_cost)
}
