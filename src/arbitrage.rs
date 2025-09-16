pub fn detect_arbitrage(price_a: f64, price_b: f64, threshold_percent: f64) 
    -> Option<(String, String, f64, f64)> 
{
    // Return (buy_dex, sell_dex, buy_price, sell_price)
    if price_b > price_a * (1.0 + threshold_percent / 100.0) {
        // buy on A, sell on B
        Some(("dex_a".to_string(), "dex_b".to_string(), price_a, price_b))
    } else if price_a > price_b * (1.0 + threshold_percent / 100.0) {
        Some(("dex_b".to_string(), "dex_a".to_string(), price_b, price_a))
    } else {
        None
    }
}

pub fn calculate_profit(buy_price: f64, sell_price: f64, amount_base: f64, gas_cost_quote: f64) -> f64 {
    let gross = (sell_price - buy_price) * amount_base;
    gross - gas_cost_quote
}
