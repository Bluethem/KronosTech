use std::env;

#[derive(Debug, Clone)]
pub struct Settings {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
    pub host: String,
}

impl Settings {
    pub fn new() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok();

        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
        })
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new().expect("Failed to load settings")
    }
}
