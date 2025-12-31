use anyhow::Result;
use std::env;
use url::Url;

#[derive(Debug, Clone)]
pub struct Config {
    pub upstreams: Vec<Url>,
    pub bind_address: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let upstreams_str =
            env::var("UPSTREAMS").unwrap_or_else(|_| "https://cache.nixos.org".to_string());

        let upstreams: Result<Vec<Url>> = upstreams_str
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| {
                Url::parse(s).map_err(|e| anyhow::anyhow!("Invalid upstream URL '{}': {}", s, e))
            })
            .collect();

        let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

        Ok(Config {
            upstreams: upstreams?,
            bind_address,
        })
    }
}
