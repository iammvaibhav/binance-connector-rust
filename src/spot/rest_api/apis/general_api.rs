/*
 * Binance Spot REST API
 *
 * OpenAPI Specifications for the Binance Spot REST API
 *
 * API documents:
 * - [Github rest-api documentation file](https://github.com/binance/binance-spot-api-docs/blob/master/rest-api.md)
 * - [General API information for rest-api on website](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/general-api-information)
 *
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use async_trait::async_trait;
use derive_builder::Builder;
use reqwest;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;

use crate::common::{
    config::ConfigurationRestApi,
    models::{ParamBuildError, RestApiResponse},
    utils::send_request,
};
use crate::spot::rest_api::models;

const HAS_TIME_UNIT: bool = true;

#[async_trait]
pub trait GeneralApi: Send + Sync {
    async fn exchange_info(
        &self,
        params: ExchangeInfoParams,
    ) -> anyhow::Result<RestApiResponse<models::ExchangeInfoResponse>>;
    async fn ping(&self) -> anyhow::Result<RestApiResponse<Value>>;
    async fn time(&self) -> anyhow::Result<RestApiResponse<models::TimeResponse>>;
}

#[derive(Debug, Clone)]
pub struct GeneralApiClient {
    configuration: ConfigurationRestApi,
}

impl GeneralApiClient {
    pub fn new(configuration: ConfigurationRestApi) -> Self {
        Self { configuration }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExchangeInfoSymbolStatusEnum {
    #[serde(rename = "TRADING")]
    Trading,
    #[serde(rename = "END_OF_DAY")]
    EndOfDay,
    #[serde(rename = "HALT")]
    Halt,
    #[serde(rename = "BREAK")]
    Break,
    #[serde(rename = "NON_REPRESENTABLE")]
    NonRepresentable,
}

impl ExchangeInfoSymbolStatusEnum {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            ExchangeInfoSymbolStatusEnum::Trading => "TRADING",
            ExchangeInfoSymbolStatusEnum::EndOfDay => "END_OF_DAY",
            ExchangeInfoSymbolStatusEnum::Halt => "HALT",
            ExchangeInfoSymbolStatusEnum::Break => "BREAK",
            ExchangeInfoSymbolStatusEnum::NonRepresentable => "NON_REPRESENTABLE",
        }
    }
}

/// Request parameters for the [`exchange_info`] operation.
///
/// This struct holds all of the inputs you can pass when calling
/// [`exchange_info`](#method.exchange_info).
#[derive(Clone, Debug, Builder, Default)]
#[builder(pattern = "owned", build_fn(error = "ParamBuildError"))]
pub struct ExchangeInfoParams {
    /// Symbol to query
    ///
    /// This field is **optional.
    #[builder(setter(into), default)]
    pub symbol: Option<String>,
    /// List of symbols to query
    ///
    /// This field is **optional.
    #[builder(setter(into), default)]
    pub symbols: Option<Vec<String>>,
    /// List of permissions to query
    ///
    /// This field is **optional.
    #[builder(setter(into), default)]
    pub permissions: Option<Vec<String>>,
    /// Controls whether the content of the `permissionSets` field is populated or not. Defaults to `true`
    ///
    /// This field is **optional.
    #[builder(setter(into), default)]
    pub show_permission_sets: Option<bool>,
    ///
    /// The `symbol_status` parameter.
    ///
    /// This field is **optional.
    #[builder(setter(into), default)]
    pub symbol_status: Option<ExchangeInfoSymbolStatusEnum>,
}

impl ExchangeInfoParams {
    /// Create a builder for [`exchange_info`].
    ///
    #[must_use]
    pub fn builder() -> ExchangeInfoParamsBuilder {
        ExchangeInfoParamsBuilder::default()
    }
}

#[async_trait]
impl GeneralApi for GeneralApiClient {
    async fn exchange_info(
        &self,
        params: ExchangeInfoParams,
    ) -> anyhow::Result<RestApiResponse<models::ExchangeInfoResponse>> {
        let ExchangeInfoParams {
            symbol,
            symbols,
            permissions,
            show_permission_sets,
            symbol_status,
        } = params;

        let mut query_params = BTreeMap::new();

        if let Some(rw) = symbol {
            query_params.insert("symbol".to_string(), json!(rw));
        }

        if let Some(rw) = symbols {
            query_params.insert("symbols".to_string(), json!(rw));
        }

        if let Some(rw) = permissions {
            query_params.insert("permissions".to_string(), json!(rw));
        }

        if let Some(rw) = show_permission_sets {
            query_params.insert("showPermissionSets".to_string(), json!(rw));
        }

        if let Some(rw) = symbol_status {
            query_params.insert("symbolStatus".to_string(), json!(rw));
        }

        send_request::<models::ExchangeInfoResponse>(
            &self.configuration,
            "/api/v3/exchangeInfo",
            reqwest::Method::GET,
            query_params,
            if HAS_TIME_UNIT {
                self.configuration.time_unit
            } else {
                None
            },
            false,
        )
        .await
    }

    async fn ping(&self) -> anyhow::Result<RestApiResponse<Value>> {
        let query_params = BTreeMap::new();

        send_request::<Value>(
            &self.configuration,
            "/api/v3/ping",
            reqwest::Method::GET,
            query_params,
            if HAS_TIME_UNIT {
                self.configuration.time_unit
            } else {
                None
            },
            false,
        )
        .await
    }

    async fn time(&self) -> anyhow::Result<RestApiResponse<models::TimeResponse>> {
        let query_params = BTreeMap::new();

        send_request::<models::TimeResponse>(
            &self.configuration,
            "/api/v3/time",
            reqwest::Method::GET,
            query_params,
            if HAS_TIME_UNIT {
                self.configuration.time_unit
            } else {
                None
            },
            false,
        )
        .await
    }
}

#[cfg(all(test, feature = "spot"))]
mod tests {
    use super::*;
    use crate::TOKIO_SHARED_RT;
    use crate::{errors::ConnectorError, models::DataFuture, models::RestApiRateLimit};
    use async_trait::async_trait;
    use std::collections::HashMap;

    struct DummyRestApiResponse<T> {
        inner: Box<dyn FnOnce() -> DataFuture<Result<T, ConnectorError>> + Send + Sync>,
        status: u16,
        headers: HashMap<String, String>,
        rate_limits: Option<Vec<RestApiRateLimit>>,
    }

    impl<T> From<DummyRestApiResponse<T>> for RestApiResponse<T> {
        fn from(dummy: DummyRestApiResponse<T>) -> Self {
            Self {
                data_fn: dummy.inner,
                status: dummy.status,
                headers: dummy.headers,
                rate_limits: dummy.rate_limits,
            }
        }
    }

    struct MockGeneralApiClient {
        force_error: bool,
    }

    #[async_trait]
    impl GeneralApi for MockGeneralApiClient {
        async fn exchange_info(
            &self,
            _params: ExchangeInfoParams,
        ) -> anyhow::Result<RestApiResponse<models::ExchangeInfoResponse>> {
            if self.force_error {
                return Err(
                    ConnectorError::ConnectorClientError("ResponseError".to_string()).into(),
                );
            }

            let resp_json: Value = serde_json::from_str(r#"{"timezone":"UTC","serverTime":1565246363776,"rateLimits":[{"rateLimitType":"RAW_REQUESTS","interval":"MINUTE","intervalNum":5,"limit":61000},{"rateLimitType":"ORDERS","interval":"DAY","intervalNum":1,"limit":160000},{"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":6000}],"exchangeFilters":[{"filterType":"EXCHANGE_MAX_NUM_ICEBERG_ORDERS","maxNumIcebergOrders":10000},{"filterType":"EXCHANGE_MAX_NUM_ALGO_ORDERS","maxNumAlgoOrders":200},{"filterType":"EXCHANGE_MAX_NUM_ORDERS","maxNumOrders":1000},{"filterType":"TRAILING_DELTA","minTrailingAboveDelta":10,"maxTrailingAboveDelta":2000,"minTrailingBelowDelta":10,"maxTrailingBelowDelta":2000},{"filterType":"MAX_POSITION","maxPosition":"10.00000000"},{"filterType":"MAX_NUM_ICEBERG_ORDERS","maxNumIcebergOrders":5},{"filterType":"MAX_NUM_ALGO_ORDERS","maxNumAlgoOrders":5},{"filterType":"MAX_NUM_ORDERS","maxNumOrders":25},{"filterType":"MARKET_LOT_SIZE","minQty":"0.00100000","maxQty":"100000.00000000","stepSize":"0.00100000"},{"filterType":"ICEBERG_PARTS","limit":10},{"filterType":"NOTIONAL","minNotional":"10.00000000","applyMinToMarket":false,"maxNotional":"10000.00000000","applyMaxToMarket":false,"avgPriceMins":5},{"filterType":"MIN_NOTIONAL","minNotional":"0.00100000","applyToMarket":true,"avgPriceMins":5},{"filterType":"LOT_SIZE","minQty":"0.00100000","maxQty":"100000.00000000","stepSize":"0.00100000"},{"filterType":"PERCENT_PRICE_BY_SIDE","bidMultiplierUp":"1.2","bidMultiplierDown":"0.2","askMultiplierUp":"5","askMultiplierDown":"0.8","avgPriceMins":1},{"filterType":"PERCENT_PRICE","multiplierUp":"1.3000","multiplierDown":"0.7000","avgPriceMins":5},{"filterType":"PRICE_FILTER","minPrice":"0.00000100","maxPrice":"100000.00000000","tickSize":"0.00000100"}],"symbols":[{"symbol":"ETHBTC","status":"TRADING","baseAsset":"ETH","baseAssetPrecision":8,"quoteAsset":"BTC","quotePrecision":8,"quoteAssetPrecision":8,"baseCommissionPrecision":8,"quoteCommissionPrecision":8,"orderTypes":["LIMIT LIMIT_MAKER MARKET STOP_LOSS STOP_LOSS_LIMIT TAKE_PROFIT TAKE_PROFIT_LIMIT"],"icebergAllowed":true,"ocoAllowed":true,"otoAllowed":true,"quoteOrderQtyMarketAllowed":true,"allowTrailingStop":false,"cancelReplaceAllowed":false,"amendAllowed":false,"isSpotTradingAllowed":true,"isMarginTradingAllowed":true,"filters":[{"filterType":"EXCHANGE_MAX_NUM_ICEBERG_ORDERS","maxNumIcebergOrders":10000},{"filterType":"EXCHANGE_MAX_NUM_ALGO_ORDERS","maxNumAlgoOrders":200},{"filterType":"EXCHANGE_MAX_NUM_ORDERS","maxNumOrders":1000},{"filterType":"TRAILING_DELTA","minTrailingAboveDelta":10,"maxTrailingAboveDelta":2000,"minTrailingBelowDelta":10,"maxTrailingBelowDelta":2000},{"filterType":"MAX_POSITION","maxPosition":"10.00000000"},{"filterType":"MAX_NUM_ICEBERG_ORDERS","maxNumIcebergOrders":5},{"filterType":"MAX_NUM_ALGO_ORDERS","maxNumAlgoOrders":5},{"filterType":"MAX_NUM_ORDERS","maxNumOrders":25},{"filterType":"MARKET_LOT_SIZE","minQty":"0.00100000","maxQty":"100000.00000000","stepSize":"0.00100000"},{"filterType":"ICEBERG_PARTS","limit":10},{"filterType":"NOTIONAL","minNotional":"10.00000000","applyMinToMarket":false,"maxNotional":"10000.00000000","applyMaxToMarket":false,"avgPriceMins":5},{"filterType":"MIN_NOTIONAL","minNotional":"0.00100000","applyToMarket":true,"avgPriceMins":5},{"filterType":"LOT_SIZE","minQty":"0.00100000","maxQty":"100000.00000000","stepSize":"0.00100000"},{"filterType":"PERCENT_PRICE_BY_SIDE","bidMultiplierUp":"1.2","bidMultiplierDown":"0.2","askMultiplierUp":"5","askMultiplierDown":"0.8","avgPriceMins":1},{"filterType":"PERCENT_PRICE","multiplierUp":"1.3000","multiplierDown":"0.7000","avgPriceMins":5},{"filterType":"PRICE_FILTER","minPrice":"0.00000100","maxPrice":"100000.00000000","tickSize":"0.00000100"}],"permissions":[],"permissionSets":[["SPOT","MARGIN"]],"defaultSelfTradePreventionMode":"NONE","allowedSelfTradePreventionModes":["NONE"]}]}"#).unwrap();
            let dummy_response: models::ExchangeInfoResponse =
                serde_json::from_value(resp_json.clone())
                    .expect("should parse into models::ExchangeInfoResponse");

            let dummy = DummyRestApiResponse {
                inner: Box::new(move || Box::pin(async move { Ok(dummy_response) })),
                status: 200,
                headers: HashMap::new(),
                rate_limits: None,
            };

            Ok(dummy.into())
        }

        async fn ping(&self) -> anyhow::Result<RestApiResponse<Value>> {
            if self.force_error {
                return Err(
                    ConnectorError::ConnectorClientError("ResponseError".to_string()).into(),
                );
            }

            let dummy_response = Value::Null;

            let dummy = DummyRestApiResponse {
                inner: Box::new(move || Box::pin(async move { Ok(dummy_response) })),
                status: 200,
                headers: HashMap::new(),
                rate_limits: None,
            };

            Ok(dummy.into())
        }

        async fn time(&self) -> anyhow::Result<RestApiResponse<models::TimeResponse>> {
            if self.force_error {
                return Err(
                    ConnectorError::ConnectorClientError("ResponseError".to_string()).into(),
                );
            }

            let resp_json: Value = serde_json::from_str(r#"{"serverTime":1499827319559}"#).unwrap();
            let dummy_response: models::TimeResponse = serde_json::from_value(resp_json.clone())
                .expect("should parse into models::TimeResponse");

            let dummy = DummyRestApiResponse {
                inner: Box::new(move || Box::pin(async move { Ok(dummy_response) })),
                status: 200,
                headers: HashMap::new(),
                rate_limits: None,
            };

            Ok(dummy.into())
        }
    }

    #[test]
    fn exchange_info_required_params_success() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockGeneralApiClient { force_error: false };

            let params = ExchangeInfoParams::builder().build().unwrap();

            let resp_json: Value = serde_json::from_str(r#"{"timezone":"UTC","serverTime":1565246363776,"rateLimits":[{"rateLimitType":"RAW_REQUESTS","interval":"MINUTE","intervalNum":5,"limit":61000},{"rateLimitType":"ORDERS","interval":"DAY","intervalNum":1,"limit":160000},{"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":6000}],"exchangeFilters":[{"filterType":"EXCHANGE_MAX_NUM_ICEBERG_ORDERS","maxNumIcebergOrders":10000},{"filterType":"EXCHANGE_MAX_NUM_ALGO_ORDERS","maxNumAlgoOrders":200},{"filterType":"EXCHANGE_MAX_NUM_ORDERS","maxNumOrders":1000},{"filterType":"TRAILING_DELTA","minTrailingAboveDelta":10,"maxTrailingAboveDelta":2000,"minTrailingBelowDelta":10,"maxTrailingBelowDelta":2000},{"filterType":"MAX_POSITION","maxPosition":"10.00000000"},{"filterType":"MAX_NUM_ICEBERG_ORDERS","maxNumIcebergOrders":5},{"filterType":"MAX_NUM_ALGO_ORDERS","maxNumAlgoOrders":5},{"filterType":"MAX_NUM_ORDERS","maxNumOrders":25},{"filterType":"MARKET_LOT_SIZE","minQty":"0.00100000","maxQty":"100000.00000000","stepSize":"0.00100000"},{"filterType":"ICEBERG_PARTS","limit":10},{"filterType":"NOTIONAL","minNotional":"10.00000000","applyMinToMarket":false,"maxNotional":"10000.00000000","applyMaxToMarket":false,"avgPriceMins":5},{"filterType":"MIN_NOTIONAL","minNotional":"0.00100000","applyToMarket":true,"avgPriceMins":5},{"filterType":"LOT_SIZE","minQty":"0.00100000","maxQty":"100000.00000000","stepSize":"0.00100000"},{"filterType":"PERCENT_PRICE_BY_SIDE","bidMultiplierUp":"1.2","bidMultiplierDown":"0.2","askMultiplierUp":"5","askMultiplierDown":"0.8","avgPriceMins":1},{"filterType":"PERCENT_PRICE","multiplierUp":"1.3000","multiplierDown":"0.7000","avgPriceMins":5},{"filterType":"PRICE_FILTER","minPrice":"0.00000100","maxPrice":"100000.00000000","tickSize":"0.00000100"}],"symbols":[{"symbol":"ETHBTC","status":"TRADING","baseAsset":"ETH","baseAssetPrecision":8,"quoteAsset":"BTC","quotePrecision":8,"quoteAssetPrecision":8,"baseCommissionPrecision":8,"quoteCommissionPrecision":8,"orderTypes":["LIMIT LIMIT_MAKER MARKET STOP_LOSS STOP_LOSS_LIMIT TAKE_PROFIT TAKE_PROFIT_LIMIT"],"icebergAllowed":true,"ocoAllowed":true,"otoAllowed":true,"quoteOrderQtyMarketAllowed":true,"allowTrailingStop":false,"cancelReplaceAllowed":false,"amendAllowed":false,"isSpotTradingAllowed":true,"isMarginTradingAllowed":true,"filters":[{"filterType":"EXCHANGE_MAX_NUM_ICEBERG_ORDERS","maxNumIcebergOrders":10000},{"filterType":"EXCHANGE_MAX_NUM_ALGO_ORDERS","maxNumAlgoOrders":200},{"filterType":"EXCHANGE_MAX_NUM_ORDERS","maxNumOrders":1000},{"filterType":"TRAILING_DELTA","minTrailingAboveDelta":10,"maxTrailingAboveDelta":2000,"minTrailingBelowDelta":10,"maxTrailingBelowDelta":2000},{"filterType":"MAX_POSITION","maxPosition":"10.00000000"},{"filterType":"MAX_NUM_ICEBERG_ORDERS","maxNumIcebergOrders":5},{"filterType":"MAX_NUM_ALGO_ORDERS","maxNumAlgoOrders":5},{"filterType":"MAX_NUM_ORDERS","maxNumOrders":25},{"filterType":"MARKET_LOT_SIZE","minQty":"0.00100000","maxQty":"100000.00000000","stepSize":"0.00100000"},{"filterType":"ICEBERG_PARTS","limit":10},{"filterType":"NOTIONAL","minNotional":"10.00000000","applyMinToMarket":false,"maxNotional":"10000.00000000","applyMaxToMarket":false,"avgPriceMins":5},{"filterType":"MIN_NOTIONAL","minNotional":"0.00100000","applyToMarket":true,"avgPriceMins":5},{"filterType":"LOT_SIZE","minQty":"0.00100000","maxQty":"100000.00000000","stepSize":"0.00100000"},{"filterType":"PERCENT_PRICE_BY_SIDE","bidMultiplierUp":"1.2","bidMultiplierDown":"0.2","askMultiplierUp":"5","askMultiplierDown":"0.8","avgPriceMins":1},{"filterType":"PERCENT_PRICE","multiplierUp":"1.3000","multiplierDown":"0.7000","avgPriceMins":5},{"filterType":"PRICE_FILTER","minPrice":"0.00000100","maxPrice":"100000.00000000","tickSize":"0.00000100"}],"permissions":[],"permissionSets":[["SPOT","MARGIN"]],"defaultSelfTradePreventionMode":"NONE","allowedSelfTradePreventionModes":["NONE"]}]}"#).unwrap();
            let expected_response : models::ExchangeInfoResponse = serde_json::from_value(resp_json.clone()).expect("should parse into models::ExchangeInfoResponse");

            let resp = client.exchange_info(params).await.expect("Expected a response");
            let data_future = resp.data();
            let actual_response = data_future.await.unwrap();
            assert_eq!(actual_response, expected_response);
        });
    }

    #[test]
    fn exchange_info_optional_params_success() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockGeneralApiClient { force_error: false };

            let params = ExchangeInfoParams::builder().symbol("BNBUSDT".to_string()).symbols(["null".to_string(),].to_vec()).permissions(["null".to_string(),].to_vec()).show_permission_sets(true).symbol_status(ExchangeInfoSymbolStatusEnum::Trading).build().unwrap();

            let resp_json: Value = serde_json::from_str(r#"{"timezone":"UTC","serverTime":1565246363776,"rateLimits":[{"rateLimitType":"RAW_REQUESTS","interval":"MINUTE","intervalNum":5,"limit":61000},{"rateLimitType":"ORDERS","interval":"DAY","intervalNum":1,"limit":160000},{"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":6000}],"exchangeFilters":[{"filterType":"EXCHANGE_MAX_NUM_ICEBERG_ORDERS","maxNumIcebergOrders":10000},{"filterType":"EXCHANGE_MAX_NUM_ALGO_ORDERS","maxNumAlgoOrders":200},{"filterType":"EXCHANGE_MAX_NUM_ORDERS","maxNumOrders":1000},{"filterType":"TRAILING_DELTA","minTrailingAboveDelta":10,"maxTrailingAboveDelta":2000,"minTrailingBelowDelta":10,"maxTrailingBelowDelta":2000},{"filterType":"MAX_POSITION","maxPosition":"10.00000000"},{"filterType":"MAX_NUM_ICEBERG_ORDERS","maxNumIcebergOrders":5},{"filterType":"MAX_NUM_ALGO_ORDERS","maxNumAlgoOrders":5},{"filterType":"MAX_NUM_ORDERS","maxNumOrders":25},{"filterType":"MARKET_LOT_SIZE","minQty":"0.00100000","maxQty":"100000.00000000","stepSize":"0.00100000"},{"filterType":"ICEBERG_PARTS","limit":10},{"filterType":"NOTIONAL","minNotional":"10.00000000","applyMinToMarket":false,"maxNotional":"10000.00000000","applyMaxToMarket":false,"avgPriceMins":5},{"filterType":"MIN_NOTIONAL","minNotional":"0.00100000","applyToMarket":true,"avgPriceMins":5},{"filterType":"LOT_SIZE","minQty":"0.00100000","maxQty":"100000.00000000","stepSize":"0.00100000"},{"filterType":"PERCENT_PRICE_BY_SIDE","bidMultiplierUp":"1.2","bidMultiplierDown":"0.2","askMultiplierUp":"5","askMultiplierDown":"0.8","avgPriceMins":1},{"filterType":"PERCENT_PRICE","multiplierUp":"1.3000","multiplierDown":"0.7000","avgPriceMins":5},{"filterType":"PRICE_FILTER","minPrice":"0.00000100","maxPrice":"100000.00000000","tickSize":"0.00000100"}],"symbols":[{"symbol":"ETHBTC","status":"TRADING","baseAsset":"ETH","baseAssetPrecision":8,"quoteAsset":"BTC","quotePrecision":8,"quoteAssetPrecision":8,"baseCommissionPrecision":8,"quoteCommissionPrecision":8,"orderTypes":["LIMIT LIMIT_MAKER MARKET STOP_LOSS STOP_LOSS_LIMIT TAKE_PROFIT TAKE_PROFIT_LIMIT"],"icebergAllowed":true,"ocoAllowed":true,"otoAllowed":true,"quoteOrderQtyMarketAllowed":true,"allowTrailingStop":false,"cancelReplaceAllowed":false,"amendAllowed":false,"isSpotTradingAllowed":true,"isMarginTradingAllowed":true,"filters":[{"filterType":"EXCHANGE_MAX_NUM_ICEBERG_ORDERS","maxNumIcebergOrders":10000},{"filterType":"EXCHANGE_MAX_NUM_ALGO_ORDERS","maxNumAlgoOrders":200},{"filterType":"EXCHANGE_MAX_NUM_ORDERS","maxNumOrders":1000},{"filterType":"TRAILING_DELTA","minTrailingAboveDelta":10,"maxTrailingAboveDelta":2000,"minTrailingBelowDelta":10,"maxTrailingBelowDelta":2000},{"filterType":"MAX_POSITION","maxPosition":"10.00000000"},{"filterType":"MAX_NUM_ICEBERG_ORDERS","maxNumIcebergOrders":5},{"filterType":"MAX_NUM_ALGO_ORDERS","maxNumAlgoOrders":5},{"filterType":"MAX_NUM_ORDERS","maxNumOrders":25},{"filterType":"MARKET_LOT_SIZE","minQty":"0.00100000","maxQty":"100000.00000000","stepSize":"0.00100000"},{"filterType":"ICEBERG_PARTS","limit":10},{"filterType":"NOTIONAL","minNotional":"10.00000000","applyMinToMarket":false,"maxNotional":"10000.00000000","applyMaxToMarket":false,"avgPriceMins":5},{"filterType":"MIN_NOTIONAL","minNotional":"0.00100000","applyToMarket":true,"avgPriceMins":5},{"filterType":"LOT_SIZE","minQty":"0.00100000","maxQty":"100000.00000000","stepSize":"0.00100000"},{"filterType":"PERCENT_PRICE_BY_SIDE","bidMultiplierUp":"1.2","bidMultiplierDown":"0.2","askMultiplierUp":"5","askMultiplierDown":"0.8","avgPriceMins":1},{"filterType":"PERCENT_PRICE","multiplierUp":"1.3000","multiplierDown":"0.7000","avgPriceMins":5},{"filterType":"PRICE_FILTER","minPrice":"0.00000100","maxPrice":"100000.00000000","tickSize":"0.00000100"}],"permissions":[],"permissionSets":[["SPOT","MARGIN"]],"defaultSelfTradePreventionMode":"NONE","allowedSelfTradePreventionModes":["NONE"]}]}"#).unwrap();
            let expected_response : models::ExchangeInfoResponse = serde_json::from_value(resp_json.clone()).expect("should parse into models::ExchangeInfoResponse");

            let resp = client.exchange_info(params).await.expect("Expected a response");
            let data_future = resp.data();
            let actual_response = data_future.await.unwrap();
            assert_eq!(actual_response, expected_response);
        });
    }

    #[test]
    fn exchange_info_response_error() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockGeneralApiClient { force_error: true };

            let params = ExchangeInfoParams::builder().build().unwrap();

            match client.exchange_info(params).await {
                Ok(_) => panic!("Expected an error"),
                Err(err) => {
                    assert_eq!(err.to_string(), "Connector client error: ResponseError");
                }
            }
        });
    }

    #[test]
    fn ping_required_params_success() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockGeneralApiClient { force_error: false };

            let expected_response = Value::Null;

            let resp = client.ping().await.expect("Expected a response");
            let data_future = resp.data();
            let actual_response = data_future.await.unwrap();
            assert_eq!(actual_response, expected_response);
        });
    }

    #[test]
    fn ping_optional_params_success() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockGeneralApiClient { force_error: false };

            let expected_response = Value::Null;

            let resp = client.ping().await.expect("Expected a response");
            let data_future = resp.data();
            let actual_response = data_future.await.unwrap();
            assert_eq!(actual_response, expected_response);
        });
    }

    #[test]
    fn ping_response_error() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockGeneralApiClient { force_error: true };

            match client.ping().await {
                Ok(_) => panic!("Expected an error"),
                Err(err) => {
                    assert_eq!(err.to_string(), "Connector client error: ResponseError");
                }
            }
        });
    }

    #[test]
    fn time_required_params_success() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockGeneralApiClient { force_error: false };

            let resp_json: Value = serde_json::from_str(r#"{"serverTime":1499827319559}"#).unwrap();
            let expected_response: models::TimeResponse = serde_json::from_value(resp_json.clone())
                .expect("should parse into models::TimeResponse");

            let resp = client.time().await.expect("Expected a response");
            let data_future = resp.data();
            let actual_response = data_future.await.unwrap();
            assert_eq!(actual_response, expected_response);
        });
    }

    #[test]
    fn time_optional_params_success() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockGeneralApiClient { force_error: false };

            let resp_json: Value = serde_json::from_str(r#"{"serverTime":1499827319559}"#).unwrap();
            let expected_response: models::TimeResponse = serde_json::from_value(resp_json.clone())
                .expect("should parse into models::TimeResponse");

            let resp = client.time().await.expect("Expected a response");
            let data_future = resp.data();
            let actual_response = data_future.await.unwrap();
            assert_eq!(actual_response, expected_response);
        });
    }

    #[test]
    fn time_response_error() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockGeneralApiClient { force_error: true };

            match client.time().await {
                Ok(_) => panic!("Expected an error"),
                Err(err) => {
                    assert_eq!(err.to_string(), "Connector client error: ResponseError");
                }
            }
        });
    }
}
