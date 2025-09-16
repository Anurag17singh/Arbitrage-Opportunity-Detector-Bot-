use serde_json::json;
use serde_json::Value;
use std::fs::{OpenOptions, create_dir_all};
use std::io::{Read, Write};
use chrono::Utc;
use anyhow::Result;

pub async fn log_opportunity(
    pair: &str,
    amount: f64,
    buy_dex: &str,
    sell_dex: &str,
    buy_price: f64,
    sell_price: f64,
    profit: f64,
) -> Result<()> {
    let obj = json!({
        "timestamp": Utc::now().to_rfc3339(),
        "pair": pair,
        "amount": amount,
        "buy_dex": buy_dex,
        "sell_dex": sell_dex,
        "buy_price": buy_price,
        "sell_price": sell_price,
        "profit": profit
    });

    create_dir_all("logs")?;

    let path = "logs/opportunities.json";

    // Step 1: Read existing file if it exists
    let mut existing: Vec<Value> = if let Ok(mut file) = std::fs::File::open(path) {
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        if content.trim().is_empty() {
            Vec::new()
        } else {
            serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
        }
    } else {
        Vec::new()
    };

    // Step 2: Append new object
    existing.push(obj.clone());

    // Step 3: Write back as JSON array
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true) // overwrite instead of append
        .open(path)?;
    let serialized = serde_json::to_string_pretty(&existing)?;
    file.write_all(serialized.as_bytes())?;

    println!("Logged opportunity: {}", obj);

    Ok(())
}
