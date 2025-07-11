/*
 * Binance Sub Account REST API
 *
 * OpenAPI Specification for the Binance Sub Account REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::sub_account::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetSummaryOfSubAccountsFuturesAccountV2ResponseDeliveryAccountSummaryResp {
    #[serde(rename = "totalMarginBalanceOfBTC", skip_serializing_if = "Option::is_none")]
    pub total_margin_balance_of_btc: Option<String>,
    #[serde(rename = "totalUnrealizedProfitOfBTC", skip_serializing_if = "Option::is_none")]
    pub total_unrealized_profit_of_btc: Option<String>,
    #[serde(rename = "totalWalletBalanceOfBTC", skip_serializing_if = "Option::is_none")]
    pub total_wallet_balance_of_btc: Option<String>,
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(rename = "subAccountList", skip_serializing_if = "Option::is_none")]
    pub sub_account_list: Option<Vec<models::GetSummaryOfSubAccountsFuturesAccountV2ResponseDeliveryAccountSummaryRespSubAccountListInner>>,
}

impl GetSummaryOfSubAccountsFuturesAccountV2ResponseDeliveryAccountSummaryResp {
    #[must_use]
    pub fn new() -> GetSummaryOfSubAccountsFuturesAccountV2ResponseDeliveryAccountSummaryResp {
        GetSummaryOfSubAccountsFuturesAccountV2ResponseDeliveryAccountSummaryResp {
            total_margin_balance_of_btc: None,
            total_unrealized_profit_of_btc: None,
            total_wallet_balance_of_btc: None,
            asset: None,
            sub_account_list: None,
        }
    }
}
