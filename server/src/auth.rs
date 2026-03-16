use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub aud: String,
    pub iss: String,
    pub exp: u64,
    pub iat: u64,
    pub role: Option<String>,
    pub email: Option<String>,
}

#[derive(Clone)]
pub struct Auth {
    pub supabase_url: String,
    pub jwt_secret: String,
}

impl Auth {
    pub fn from_env() -> Self {
        Self {
            supabase_url: std::env::var("SUPABASE_URL").expect("SUPABASE_URL required"),
            jwt_secret: std::env::var("SUPABASE_JWT_SECRET").expect("SUPABASE_JWT_SECRET required"),
        }
    }

    pub async fn validate_token(&self, token: &str) -> Result<Uuid, String> {
        let decoding_key = DecodingKey::from_secret(self.jwt_secret.as_bytes());

        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        validation.set_audience(&["authenticated"]);
        validation.set_issuer(&[&self.supabase_url]);

        let token_data = decode::<Claims>(token, &decoding_key, &validation)
            .map_err(|e| e.to_string())?;

        Uuid::parse_str(&token_data.claims.sub).map_err(|e| e.to_string())
    }
}
