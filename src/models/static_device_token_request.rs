/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.2
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// StaticDeviceTokenRequest : Serializer for static device's tokens
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticDeviceTokenRequest {
    #[serde(rename = "token")]
    pub token: String,
}

impl StaticDeviceTokenRequest {
    /// Serializer for static device's tokens
    pub fn new(token: String) -> StaticDeviceTokenRequest {
        StaticDeviceTokenRequest { token }
    }
}
