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

/// WebAuthnDeviceType : WebAuthnDeviceType Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebAuthnDeviceType {
    #[serde(rename = "aaguid")]
    pub aaguid: uuid::Uuid,
    #[serde(rename = "description")]
    pub description: String,
}

impl WebAuthnDeviceType {
    /// WebAuthnDeviceType Serializer
    pub fn new(aaguid: uuid::Uuid, description: String) -> WebAuthnDeviceType {
        WebAuthnDeviceType { aaguid, description }
    }
}
