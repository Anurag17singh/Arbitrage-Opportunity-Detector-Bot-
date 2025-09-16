use serde::Deserialize;
use std::{fs, env};
use anyhow::Result;

#[derive(Debug, Deserialize, Clone)]
pub struct TokenPair {
    pub base: String,
    pub base_address: String,
    pub base_decimals: u8,
    pub quote: String,
    pub quote_address: String,
    pub quote_decimals: u8,
    pub amount: f64
}

#[derive(Debug, Deserialize, Clone)]
pub struct Dexes {
    pub dex_a: String,
    pub dex_b: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub rpc_url: String,
    pub token_pair: TokenPair,
    pub dexes: Dexes,
    pub threshold_percent: f64,
    pub gas_cost_quote: f64
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let s = fs::read_to_string(path)?;
        let mut cfg: Config = serde_json::from_str(&s)?;
        // override from environment if set
        if let Ok(rpc) = env::var("POLYGON_RPC") {
            cfg.rpc_url = rpc;
        }
        Ok(cfg)
    }
}
