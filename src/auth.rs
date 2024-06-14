use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::Deserialize;
use std::error::Error;
use reqwest;

#[derive(Deserialize)]
struct FirebaseClaims {
    email: String,
    // 他の必要なクレームがあればここに追加
}

async fn verify_firebase_token(token: &str) -> Result<FirebaseClaims, Box<dyn Error>> {
    let google_jwks_url = "https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com";
    let jwks: serde_json::Value = reqwest::get(google_jwks_url).await?.json().await?;

    let header = decode::<serde_json::Value>(token, &DecodingKey::from_secret("your_secret".as_ref()), &Validation::new(Algorithm::RS256))
        .unwrap()
        .header;
    let kid = header.kid.ok_or("Invalid token: missing kid")?;

    let jwk = jwks.get(&kid).ok_or("Invalid token: unknown kid")?;
    let public_key = jwk["n"].as_str().ok_or("Invalid token: missing public key")?;

    let decoding_key = DecodingKey::from_rsa_components(public_key, jwk["e"].as_str().unwrap())?;

    let token_data = decode::<FirebaseClaims>(token, &decoding_key, &Validation::new(Algorithm::RS256))?;

    Ok(token_data.claims)
}
