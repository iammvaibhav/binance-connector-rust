# Proxy Configuration

```rust
use binance_sdk::derivatives_trading_options;
use binance_sdk::config;

let proxy_config = config::ProxyConfig {
    host: "127.0.0.1".to_string(),
    port: 8080,
    protocol: Some("http".to_string()),
    auth: Some(config::ProxyAuth {
        username: "proxy-user".to_string(),
        password: "proxy-password".to_string(),
    }),
};

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .proxy(proxy_config)
    .build()?;

let client = derivatives_trading_options::DerivativesTradingOptionsRestApi::production(configuration);
let params = derivatives_trading_options::rest_api::OptionAccountInformationParams::default();
let response = client.option_account_information(params).await?;
```
