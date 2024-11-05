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

/// WebAuthnDeviceRequest : Serializer for WebAuthn authenticator devices
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebAuthnDeviceRequest {
    #[serde(rename = "name")]
    pub name: String,
}

impl WebAuthnDeviceRequest {
    /// Serializer for WebAuthn authenticator devices
    pub fn new(name: String) -> WebAuthnDeviceRequest {
        WebAuthnDeviceRequest { name }
    }
}
