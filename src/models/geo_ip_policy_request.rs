/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.1
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// GeoIpPolicyRequest : GeoIP Policy Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoIpPolicyRequest {
    #[serde(rename = "name")]
    pub name: String,
    /// When this option is enabled, all executions of this policy will be logged. By default, only execution errors are logged.
    #[serde(rename = "execution_logging", skip_serializing_if = "Option::is_none")]
    pub execution_logging: Option<bool>,
    #[serde(rename = "asns", skip_serializing_if = "Option::is_none")]
    pub asns: Option<Vec<i32>>,
    #[serde(rename = "countries")]
    pub countries: Vec<models::CountryCodeEnum>,
}

impl GeoIpPolicyRequest {
    /// GeoIP Policy Serializer
    pub fn new(name: String, countries: Vec<models::CountryCodeEnum>) -> GeoIpPolicyRequest {
        GeoIpPolicyRequest {
            name,
            execution_logging: None,
            asns: None,
            countries,
        }
    }
}
