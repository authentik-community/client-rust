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

/// WebAuthnDevice : Serializer for WebAuthn authenticator devices
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebAuthnDevice {
    #[serde(rename = "pk")]
    pub pk: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "created_on")]
    pub created_on: String,
    #[serde(rename = "device_type", deserialize_with = "Option::deserialize")]
    pub device_type: Option<models::WebAuthnDeviceType>,
    #[serde(rename = "aaguid")]
    pub aaguid: String,
    #[serde(rename = "user")]
    pub user: models::GroupMember,
}

impl WebAuthnDevice {
    /// Serializer for WebAuthn authenticator devices
    pub fn new(
        pk: i32,
        name: String,
        created_on: String,
        device_type: Option<models::WebAuthnDeviceType>,
        aaguid: String,
        user: models::GroupMember,
    ) -> WebAuthnDevice {
        WebAuthnDevice {
            pk,
            name,
            created_on,
            device_type,
            aaguid,
            user,
        }
    }
}
