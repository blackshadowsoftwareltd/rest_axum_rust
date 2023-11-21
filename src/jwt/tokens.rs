use anyhow::{Ok, Result};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub fn generate_jwt_token() -> Result<()> {
    let my_claims = Claims {
        sub: "Hello".to_string(),
        company: "black shadow".to_string(),
        exp: 10,
    };
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )?;
    println!("Token : {:?}", token);
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}
