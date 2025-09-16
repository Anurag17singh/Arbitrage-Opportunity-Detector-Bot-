use ethers::prelude::*;
use std::sync::Arc;
use anyhow::Result;
//use std::str::FromStr;

// Generate bindings for UniswapV2-style router (getAmountsOut)
abigen!(
    IUniswapV2Router,
    r#"[
        function getAmountsOut(uint256 amountIn, address[] memory path) external view returns (uint256[] memory amounts)
    ]"#
);

pub struct DexClient {
    router: IUniswapV2Router<Provider<Http>>,
    _provider: Arc<Provider<Http>>,
}

impl DexClient {
    pub fn new(provider: Arc<Provider<Http>>, router_addr: Address) -> Self {
        let router = IUniswapV2Router::new(router_addr, provider.clone());
        Self { router, _provider: provider }
    }

    // low-level wrapper
    pub async fn get_amounts_out(&self, amount_in: U256, path: Vec<Address>) -> Result<Vec<U256>> {
        let amounts = self.router.get_amounts_out(amount_in, path).call().await?;
        Ok(amounts)
    }

    // higher-level price fetch (returns price (quote units) per 1 base unit)
    // NOTE: this uses f64 for simplicity (ok for prototyping). For production use BigDecimals.
    pub async fn get_price_for_amount(
        &self,
        amount_in_units: f64,
        base_decimals: u8,
        quote_decimals: u8,
        base_address: Address,
        quote_address: Address
    ) -> Result<f64> {
        let multiplier_base = 10f64.powi(base_decimals as i32);
        let amount_in_wei_f = amount_in_units * multiplier_base;
        // convert to u128 then U256 - safe for small amounts (demo)
        let amount_in_u128: u128 = amount_in_wei_f as u128;
        let amount_in = U256::from(amount_in_u128);
        let path = vec![base_address, quote_address];
        let amounts = self.get_amounts_out(amount_in, path).await?;
        let amount_out = amounts.last().ok_or_else(|| anyhow::anyhow!("empty amounts"))?;
        let amount_out_u128 = amount_out.as_u128();
        let amount_out_f = amount_out_u128 as f64 / 10f64.powi(quote_decimals as i32);
        // Price per base token (quote units per 1 base)
        let price = amount_out_f / amount_in_units;
        Ok(price)
    }
}
