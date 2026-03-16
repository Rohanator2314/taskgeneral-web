use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
    pub iat: u64,
    pub role: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Jwk {
    pub kid: String,
    pub alg: String,
    pub kty: String,
    pub x: Option<String>,
    pub y: Option<String>,
    pub n: Option<String>,
    pub e: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwkSet {
    pub keys: Vec<Jwk>,
}

#[derive(Clone)]
pub struct Auth {
    pub supabase_url: String,
    pub jwt_secret: String,
    pub jwks: JwkSet,
}

impl Auth {
    pub fn from_env() -> Self {
        Self {
            supabase_url: std::env::var("SUPABASE_URL").expect("SUPABASE_URL required"),
            jwt_secret: std::env::var("SUPABASE_JWT_SECRET").expect("SUPABASE_JWT_SECRET required"),
            jwks: JwkSet { keys: vec![] },
        }
    }

    pub async fn load_jwks(&mut self) -> Result<(), String> {
        let url = format!(
            "{}/auth/v1/.well-known/jwks.json",
            self.supabase_url.trim_end_matches('/')
        );
        let resp = reqwest::get(&url)
            .await
            .map_err(|e| format!("JWKS fetch failed: {e}"))?;
        self.jwks = resp
            .json::<JwkSet>()
            .await
            .map_err(|e| format!("JWKS parse failed: {e}"))?;
        Ok(())
    }

    pub async fn validate_token(&self, token: &str) -> Result<Uuid, String> {
        let header = decode_header(token).map_err(|e| format!("bad header: {e}"))?;
        let kid = header.kid.as_deref().unwrap_or("");

        let jwk = self
            .jwks
            .keys
            .iter()
            .find(|k| k.kid == kid)
            .ok_or_else(|| format!("no matching key for kid={kid}"))?;

        let decoding_key = match jwk.alg.as_str() {
            "ES256" => {
                let x = jwk.x.as_deref().ok_or("missing x")?;
                let y = jwk.y.as_deref().ok_or("missing y")?;
                DecodingKey::from_ec_components(x, y).map_err(|e| e.to_string())?
            }
            "RS256" | "RS384" | "RS512" => {
                let n = jwk.n.as_deref().ok_or("missing n")?;
                let e_val = jwk.e.as_deref().ok_or("missing e")?;
                DecodingKey::from_rsa_components(n, e_val).map_err(|e| e.to_string())?
            }
            _ => {
                let secret = base64_decode_secret(&self.jwt_secret);
                DecodingKey::from_secret(&secret)
            }
        };

        let algorithm = match jwk.alg.as_str() {
            "ES256" => Algorithm::ES256,
            "RS256" => Algorithm::RS256,
            "RS384" => Algorithm::RS384,
            "RS512" => Algorithm::RS512,
            _ => Algorithm::HS256,
        };

        let mut validation = Validation::new(algorithm);
        validation.validate_exp = true;
        let issuer = format!("{}/auth/v1", self.supabase_url.trim_end_matches('/'));
        validation.set_issuer(&[&issuer]);
        validation.set_audience(&["authenticated"]);

        let token_data = decode::<Claims>(token, &decoding_key, &validation)
            .map_err(|e| format!("JWT decode: {e}"))?;

        Uuid::parse_str(&token_data.claims.sub).map_err(|e| e.to_string())
    }
}

fn base64_decode_secret(secret: &str) -> Vec<u8> {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(secret)
        .unwrap_or_else(|_| secret.as_bytes().to_vec())
}
