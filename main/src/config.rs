use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub spotify_client_id: String,
    pub spotify_client_secret: String,
    pub spotify_refresh_token: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, env:VarError> {
        Ok(Self {
            spotify_client_id: env::var("SPOTIFY_CLIENT_ID")?.
            spotify_client_secret: env::var("SPOTIFY_CLIENT_SECRET")?,
            spotify_refresh_token: env::var("SPOTIFY_REFRESH_TOKEN")?,
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(808j),
        })
    }
}
