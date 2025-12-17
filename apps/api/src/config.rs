use serde::Deserialize;
use anyhow::Result;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_environment")]
    pub environment: String,

    pub loops_api_key: Option<String>,
    pub loops_form_id: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        // Load .env file
        dotenvy::dotenv().ok();

        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or_else(default_port);

        let host = std::env::var("HOST").unwrap_or_else(|_| default_host());

        let environment = std::env::var("ENVIRONMENT").unwrap_or_else(|_| default_environment());

        let loops_api_key = std::env::var("NEXT_PUBLIC_LOOPS_API_KEY").ok();
        let loops_form_id = std::env::var("NEXT_PUBLIC_LOOPS_FORM_ID").ok();

        Ok(Config {
            port,
            host,
            environment,
            loops_api_key,
            loops_form_id,
        })
    }
}

fn default_port() -> u16 {
    4400
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_environment() -> String {
    "development".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: default_port(),
            host: default_host(),
            environment: default_environment(),
            loops_api_key: None,
            loops_form_id: None,
        }
    }
}
