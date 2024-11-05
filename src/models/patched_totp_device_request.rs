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

/// PatchedTotpDeviceRequest : Serializer for totp authenticator devices
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedTotpDeviceRequest {
    /// The human-readable name of this device.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PatchedTotpDeviceRequest {
    /// Serializer for totp authenticator devices
    pub fn new() -> PatchedTotpDeviceRequest {
        PatchedTotpDeviceRequest { name: None }
    }
}
