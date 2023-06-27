use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use include_crypt::{include_crypt, EncryptedFile};
use jsonwebtoken::{crypto, Algorithm, EncodingKey};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

/// The number of seconds in an hour
const ONE_HOUR: u64 = 3600;

static PRIVATE_KEY: Lazy<EncryptedFile> = Lazy::new(|| include_crypt!("assets/private_key.p8"));
static TOKEN_ASSETS: Lazy<EncryptedFile> = Lazy::new(|| include_crypt!("assets/token_assets.json"));

#[derive(Debug, Deserialize)]
struct TokenAssets {
    key_id: String,
    service_id: String,
    team_id: String,
}

#[derive(Debug, Serialize)]
struct Header {
    /// Always "JWT"
    typ: String,
    /// Always ECS
    alg: Algorithm,
    kid: String,
    id: String,
}

#[derive(Debug, Serialize)]
struct Claims {
    /// Issuer claim key.
    iss: String,
    /// Issued-at claim key
    iat: u64,
    /// Expiration time claim key
    exp: u64,
    /// Subject public claim key
    sub: String,
}

#[derive(Debug)]
struct Token {
    header: Header,
    claims: Claims,
}

impl Header {
    fn new(kid: &str, id: &str) -> Self {
        Self {
            typ: "JWT".to_owned(),
            alg: Algorithm::ES256,
            kid: kid.to_owned(),
            id: id.to_owned(),
        }
    }
}

fn encode_as_b64<T: Serialize>(t: &T) -> anyhow::Result<String> {
    let mut result = String::new();
    let json = serde_json::to_vec(t)?;
    BASE64_URL_SAFE_NO_PAD.encode_string(json, &mut result);
    Ok(result)
}

fn get_token_assets() -> Result<TokenAssets, serde_json::Error> {
    let decrypted_file = TOKEN_ASSETS.decrypt();
    serde_json::from_slice(&decrypted_file)
}

fn get_claims(token_assets: &TokenAssets) -> anyhow::Result<Claims> {
    Ok(Claims {
        iss: token_assets.team_id.clone(),
        iat: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs(),
        exp: (SystemTime::now() + Duration::from_secs(ONE_HOUR))
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs(),
        sub: token_assets.service_id.clone(),
    })
}

fn get_token(token_assets: &TokenAssets) -> anyhow::Result<Token> {
    let header = Header::new(
        &token_assets.key_id,
        &format!("{}.{}", &token_assets.team_id, &token_assets.service_id),
    );
    let claims = get_claims(token_assets)?;

    Ok(Token { header, claims })
}

pub fn generate_token() -> anyhow::Result<String> {
    let token = get_token(&get_token_assets()?)?;

    // Instead of using serde::Serialize on token, we serialize the header and claims
    // separately and just append the claims to the header; that way, we get two
    // separate json objects as Apple requires
    let header_chars = encode_as_b64(&token.header)?;
    let claims_chars = encode_as_b64(&token.claims)?;
    let token_chars = [header_chars, claims_chars].join(".");

    let key = EncodingKey::from_ec_pem(&PRIVATE_KEY.decrypt())?;
    let signature = crypto::sign(token_chars.as_bytes(), &key, token.header.alg)?;

    Ok([token_chars, signature].join("."))
}
