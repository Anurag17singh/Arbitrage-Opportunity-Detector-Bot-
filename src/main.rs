mod config;
mod dex;
mod arbitrage;
mod logger;

use config::Config;
use ethers::prelude::*;
use std::sync::Arc;
use std::str::FromStr;
use tokio::time::{sleep, Duration};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    // 1. load config
    let cfg = Config::load("config.json")?;
    println!("Loaded config: {:?}", cfg);

    // 2. provider (Polygon RPC)
    let provider = Provider::<Http>::try_from(cfg.rpc_url.clone())?;
    let provider = Arc::new(provider);

    // 3. parse addresses
    let base_addr = Address::from_str(&cfg.token_pair.base_address)?;
    let quote_addr = Address::from_str(&cfg.token_pair.quote_address)?;
    let dex_a_addr = Address::from_str(&cfg.dexes.dex_a)?;
    let dex_b_addr = Address::from_str(&cfg.dexes.dex_b)?;

    // 4. create DexClients
    let dex_a = dex::DexClient::new(provider.clone(), dex_a_addr);
    let dex_b = dex::DexClient::new(provider.clone(), dex_b_addr);

    let pair_name = format!("{}/{}", cfg.token_pair.base, cfg.token_pair.quote);
    let amount = cfg.token_pair.amount;

    // Run for 100 iterations 
    for iteration in 1..=100 {
        println!("Iteration {}", iteration);

        // 5. fetch prices concurrently
        let pa = dex_a.get_price_for_amount(
            amount,
            cfg.token_pair.base_decimals,
            cfg.token_pair.quote_decimals,
            base_addr,
            quote_addr,
        );
        let pb = dex_b.get_price_for_amount(
            amount,
            cfg.token_pair.base_decimals,
            cfg.token_pair.quote_decimals,
            base_addr,
            quote_addr,
        );

        let (res_a, res_b) = tokio::join!(pa, pb);

        match (res_a, res_b) {
            (Ok(price_a), Ok(price_b)) => {
                println!("Prices: dex_a = {}, dex_b = {}", price_a, price_b);

                if let Some((buy_dex_label, sell_dex_label, buy_price, sell_price)) =
                    arbitrage::detect_arbitrage(price_a, price_b, cfg.threshold_percent)
                {
                    let profit = arbitrage::calculate_profit(
                        buy_price,
                        sell_price,
                        amount,
                        cfg.gas_cost_quote,
                    );

                    if profit > 0.0 {
                        println!("Iteration {}: Profitable arbitrage detected!", iteration);

                        logger::log_opportunity(
                            &pair_name,
                            amount,
                            &buy_dex_label,
                            &sell_dex_label,
                            buy_price,
                            sell_price,
                            profit,
                        )
                        .await?;
                    } else {
                        println!("Iteration {}: Arbitrage candidate found but not profitable after gas", iteration);
                    }
                } else {
                    println!(
                        "Iteration {}: No arbitrage above threshold {}%",
                        iteration,
                        cfg.threshold_percent
                    );
                }
            }
            (Err(e), _) => println!("Error fetching price A: {}", e),
            (_, Err(e)) => println!("Error fetching price B: {}", e),
        }

        // 6. wait before next check
        sleep(Duration::from_secs(10)).await;
    }

    println!("Completed 100 iterations. Exiting.");
    Ok(())
}
