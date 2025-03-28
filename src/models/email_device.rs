/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// EmailDevice : Serializer for email authenticator devices
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailDevice {
    /// The human-readable name of this device.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "pk")]
    pub pk: i32,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "user")]
    pub user: models::GroupMember,
}

impl EmailDevice {
    /// Serializer for email authenticator devices
    pub fn new(name: String, pk: i32, email: String, user: models::GroupMember) -> EmailDevice {
        EmailDevice { name, pk, email, user }
    }
}
