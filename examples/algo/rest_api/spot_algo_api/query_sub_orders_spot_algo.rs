use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::algo::{AlgoRestApi, rest_api::QuerySubOrdersSpotAlgoParams};
use binance_sdk::config::ConfigurationRestApi;

#[tokio::main]
async fn main() -> Result<()> {
    // Load credentials from env
    let api_key = env::var("API_KEY").context("API_KEY must be set")?;
    let api_secret = env::var("API_SECRET").context("API_SECRET must be set")?;

    // Build REST config
    let rest_conf = ConfigurationRestApi::builder()
        .api_key(api_key)
        .api_secret(api_secret)
        .build()?;

    // Create the Algo REST API client
    let rest_client = AlgoRestApi::production(rest_conf);

    // Setup the API parameters
    let params = QuerySubOrdersSpotAlgoParams::builder(1).build()?;

    // Make the API call
    let response = rest_client
        .query_sub_orders_spot_algo(params)
        .await
        .context("query_sub_orders_spot_algo request failed")?;

    info!(?response.rate_limits, "query_sub_orders_spot_algo rate limits");
    let data = response.data().await?;
    info!(?data, "query_sub_orders_spot_algo data");

    Ok(())
}
