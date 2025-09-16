# Arbitrage-Opportunity-Detector-Bot
* A bot in Rust that detects potential arbitrage opportunities on the Polygon network.

* It monitors two decentralized exchanges (DEXes) — QuickSwap and SushiSwap — and checks whether a profitable price difference exists for a given token pair (e.g., WETH/USDC).

* It is implemented in Rust using the ethers-rs library and designed to run against a live Polygon RPC endpoint.

## Features

* Connects to Polygon RPC.

* Fetches live token prices from UniswapV2-style DEX routers (getAmountsOut).

* Simulates trades of configurable size (e.g., 0.5 WETH).

* Detects arbitrage if price difference exceeds a configured threshold_percent.

* Accounts for gas cost in profit calculation.

* Logs each iteration with whether arbitrage is detected.

* Runs for a fixed number of iterations (default: 100).

## Structure

* polygon-arb-bot/
* ├── Cargo.toml          # Rust dependencies
* ├── Cargo.lock          # Auto-generated dependency lock file
* ├── config.json         # Configuration file
* └── src/
    * ├── main.rs         # Entry point, loop controller
    * ├── config.rs       # Loads config.json
    * ├── dex.rs          # DEX client abstraction (UniswapV2 router binding)
    * ├── arbitrage.rs    # Arbitrage detection & profit calculation
    * └── logger.rs       # Logs detected opportunities

