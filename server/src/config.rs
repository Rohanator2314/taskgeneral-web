use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub supabase_url: String,
    pub supabase_jwt_secret: String,
    pub taskgeneral_data_dir: Option<String>,
    pub taskgeneral_port: u16,
    pub taskgeneral_static_dir: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            supabase_url: std::env::var("SUPABASE_URL")?,
            supabase_jwt_secret: std::env::var("SUPABASE_JWT_SECRET")?,
            taskgeneral_data_dir: std::env::var("TASKGENERAL_DATA_DIR").ok(),
            taskgeneral_port: std::env::var("TASKGENERAL_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            taskgeneral_static_dir: std::env::var("TASKGENERAL_STATIC_DIR").ok(),
        })
    }

    pub fn get() -> &'static Config {
        CONFIG.get_or_init(|| Config::from_env().expect("Failed to load configuration"))
    }
}
