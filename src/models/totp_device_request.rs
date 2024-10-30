/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.0
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// TotpDeviceRequest : Serializer for totp authenticator devices
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TotpDeviceRequest {
    /// The human-readable name of this device.
    #[serde(rename = "name")]
    pub name: String,
}

impl TotpDeviceRequest {
    /// Serializer for totp authenticator devices
    pub fn new(name: String) -> TotpDeviceRequest {
        TotpDeviceRequest { name }
    }
}
