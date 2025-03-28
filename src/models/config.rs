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

/// Config : Serialize authentik Config into DRF Object
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "error_reporting")]
    pub error_reporting: models::ErrorReportingConfig,
    #[serde(rename = "capabilities")]
    pub capabilities: Vec<models::CapabilitiesEnum>,
    #[serde(rename = "cache_timeout")]
    pub cache_timeout: i32,
    #[serde(rename = "cache_timeout_flows")]
    pub cache_timeout_flows: i32,
    #[serde(rename = "cache_timeout_policies")]
    pub cache_timeout_policies: i32,
    #[serde(rename = "cache_timeout_reputation")]
    pub cache_timeout_reputation: i32,
}

impl Config {
    /// Serialize authentik Config into DRF Object
    pub fn new(
        error_reporting: models::ErrorReportingConfig,
        capabilities: Vec<models::CapabilitiesEnum>,
        cache_timeout: i32,
        cache_timeout_flows: i32,
        cache_timeout_policies: i32,
        cache_timeout_reputation: i32,
    ) -> Config {
        Config {
            error_reporting,
            capabilities,
            cache_timeout,
            cache_timeout_flows,
            cache_timeout_policies,
            cache_timeout_reputation,
        }
    }
}
