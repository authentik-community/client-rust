/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PatchedDuoDeviceRequest : Serializer for Duo authenticator devices
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedDuoDeviceRequest {
    /// The human-readable name of this device.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PatchedDuoDeviceRequest {
    /// Serializer for Duo authenticator devices
    pub fn new() -> PatchedDuoDeviceRequest {
        PatchedDuoDeviceRequest { name: None }
    }
}
