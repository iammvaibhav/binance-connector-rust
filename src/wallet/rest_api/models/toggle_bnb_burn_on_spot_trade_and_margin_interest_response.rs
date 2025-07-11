/*
 * Binance Wallet REST API
 *
 * OpenAPI Specification for the Binance Wallet REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::wallet::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToggleBnbBurnOnSpotTradeAndMarginInterestResponse {
    #[serde(rename = "spotBNBBurn", skip_serializing_if = "Option::is_none")]
    pub spot_bnb_burn: Option<bool>,
    #[serde(rename = "interestBNBBurn", skip_serializing_if = "Option::is_none")]
    pub interest_bnb_burn: Option<bool>,
}

impl ToggleBnbBurnOnSpotTradeAndMarginInterestResponse {
    #[must_use]
    pub fn new() -> ToggleBnbBurnOnSpotTradeAndMarginInterestResponse {
        ToggleBnbBurnOnSpotTradeAndMarginInterestResponse {
            spot_bnb_burn: None,
            interest_bnb_burn: None,
        }
    }
}
