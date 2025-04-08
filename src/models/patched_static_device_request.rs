/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.4
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PatchedStaticDeviceRequest : Serializer for static authenticator devices
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedStaticDeviceRequest {
    /// The human-readable name of this device.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PatchedStaticDeviceRequest {
    /// Serializer for static authenticator devices
    pub fn new() -> PatchedStaticDeviceRequest {
        PatchedStaticDeviceRequest { name: None }
    }
}
