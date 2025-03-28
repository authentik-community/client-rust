/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PatchedGeoIpPolicyRequest : GeoIP Policy Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedGeoIpPolicyRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// When this option is enabled, all executions of this policy will be logged. By default, only execution errors are logged.
    #[serde(rename = "execution_logging", skip_serializing_if = "Option::is_none")]
    pub execution_logging: Option<bool>,
    #[serde(rename = "asns", skip_serializing_if = "Option::is_none")]
    pub asns: Option<Vec<i32>>,
    #[serde(rename = "countries", skip_serializing_if = "Option::is_none")]
    pub countries: Option<Vec<models::CountryCodeEnum>>,
    #[serde(rename = "check_history_distance", skip_serializing_if = "Option::is_none")]
    pub check_history_distance: Option<bool>,
    #[serde(rename = "history_max_distance_km", skip_serializing_if = "Option::is_none")]
    pub history_max_distance_km: Option<u64>,
    #[serde(rename = "distance_tolerance_km", skip_serializing_if = "Option::is_none")]
    pub distance_tolerance_km: Option<u32>,
    #[serde(rename = "history_login_count", skip_serializing_if = "Option::is_none")]
    pub history_login_count: Option<u32>,
    #[serde(rename = "check_impossible_travel", skip_serializing_if = "Option::is_none")]
    pub check_impossible_travel: Option<bool>,
    #[serde(rename = "impossible_tolerance_km", skip_serializing_if = "Option::is_none")]
    pub impossible_tolerance_km: Option<u32>,
}

impl PatchedGeoIpPolicyRequest {
    /// GeoIP Policy Serializer
    pub fn new() -> PatchedGeoIpPolicyRequest {
        PatchedGeoIpPolicyRequest {
            name: None,
            execution_logging: None,
            asns: None,
            countries: None,
            check_history_distance: None,
            history_max_distance_km: None,
            distance_tolerance_km: None,
            history_login_count: None,
            check_impossible_travel: None,
            impossible_tolerance_km: None,
        }
    }
}
