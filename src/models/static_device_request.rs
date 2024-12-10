/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.5
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// StaticDeviceRequest : Serializer for static authenticator devices
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticDeviceRequest {
    /// The human-readable name of this device.
    #[serde(rename = "name")]
    pub name: String,
}

impl StaticDeviceRequest {
    /// Serializer for static authenticator devices
    pub fn new(name: String) -> StaticDeviceRequest {
        StaticDeviceRequest { name }
    }
}
