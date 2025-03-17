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

/// PatchedSmsDeviceRequest : Serializer for sms authenticator devices
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedSmsDeviceRequest {
    /// The human-readable name of this device.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PatchedSmsDeviceRequest {
    /// Serializer for sms authenticator devices
    pub fn new() -> PatchedSmsDeviceRequest {
        PatchedSmsDeviceRequest { name: None }
    }
}
