/*
 * Binance Staking REST API
 *
 * OpenAPI Specification for the Binance Staking REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::staking::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetSolStakingQuotaDetailsResponse {
    #[serde(
        rename = "leftStakingPersonalQuota",
        skip_serializing_if = "Option::is_none"
    )]
    pub left_staking_personal_quota: Option<String>,
    #[serde(
        rename = "leftRedemptionPersonalQuota",
        skip_serializing_if = "Option::is_none"
    )]
    pub left_redemption_personal_quota: Option<String>,
    #[serde(rename = "minStakeAmount", skip_serializing_if = "Option::is_none")]
    pub min_stake_amount: Option<String>,
    #[serde(rename = "minRedeemAmount", skip_serializing_if = "Option::is_none")]
    pub min_redeem_amount: Option<String>,
    #[serde(rename = "redeemPeriod", skip_serializing_if = "Option::is_none")]
    pub redeem_period: Option<i64>,
    #[serde(rename = "stakeable", skip_serializing_if = "Option::is_none")]
    pub stakeable: Option<bool>,
    #[serde(rename = "redeemable", skip_serializing_if = "Option::is_none")]
    pub redeemable: Option<bool>,
    #[serde(rename = "soldOut", skip_serializing_if = "Option::is_none")]
    pub sold_out: Option<bool>,
    #[serde(rename = "commissionFee", skip_serializing_if = "Option::is_none")]
    pub commission_fee: Option<String>,
    #[serde(rename = "nextEpochTime", skip_serializing_if = "Option::is_none")]
    pub next_epoch_time: Option<i64>,
    #[serde(rename = "calculating", skip_serializing_if = "Option::is_none")]
    pub calculating: Option<bool>,
}

impl GetSolStakingQuotaDetailsResponse {
    #[must_use]
    pub fn new() -> GetSolStakingQuotaDetailsResponse {
        GetSolStakingQuotaDetailsResponse {
            left_staking_personal_quota: None,
            left_redemption_personal_quota: None,
            min_stake_amount: None,
            min_redeem_amount: None,
            redeem_period: None,
            stakeable: None,
            redeemable: None,
            sold_out: None,
            commission_fee: None,
            next_epoch_time: None,
            calculating: None,
        }
    }
}
