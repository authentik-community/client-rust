/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.6.4
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// SystemInfo : Get system information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemInfo {
    /// Get HTTP Request headers
    #[serde(rename = "http_headers")]
    pub http_headers: std::collections::HashMap<String, String>,
    /// Get HTTP host
    #[serde(rename = "http_host")]
    pub http_host: String,
    /// Get HTTP Secure flag
    #[serde(rename = "http_is_secure")]
    pub http_is_secure: bool,
    #[serde(rename = "runtime")]
    pub runtime: models::SystemInfoRuntime,
    /// Currently active brand
    #[serde(rename = "brand")]
    pub brand: String,
    /// Current server time
    #[serde(rename = "server_time")]
    pub server_time: String,
    /// Whether the embedded outpost is disabled
    #[serde(rename = "embedded_outpost_disabled")]
    pub embedded_outpost_disabled: bool,
    /// Get the FQDN configured on the embedded outpost
    #[serde(rename = "embedded_outpost_host")]
    pub embedded_outpost_host: String,
}

impl SystemInfo {
    /// Get system information.
    pub fn new(
        http_headers: std::collections::HashMap<String, String>,
        http_host: String,
        http_is_secure: bool,
        runtime: models::SystemInfoRuntime,
        brand: String,
        server_time: String,
        embedded_outpost_disabled: bool,
        embedded_outpost_host: String,
    ) -> SystemInfo {
        SystemInfo {
            http_headers,
            http_host,
            http_is_secure,
            runtime,
            brand,
            server_time,
            embedded_outpost_disabled,
            embedded_outpost_host,
        }
    }
}
