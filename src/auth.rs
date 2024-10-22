use jsonwebtoken::{encode, Header, EncodingKey, errors::Error};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn generate_jwt(email: &str) -> Result<String, Error> {
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    
    let claims = Claims {
        sub: email.to_owned(),
        exp: 10000000000, // Token expiration time (adjust as needed)
    };
    
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_bytes()))
}
