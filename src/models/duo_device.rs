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

/// DuoDevice : Serializer for Duo authenticator devices
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DuoDevice {
    #[serde(rename = "pk")]
    pub pk: i32,
    /// The human-readable name of this device.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "user")]
    pub user: models::GroupMember,
}

impl DuoDevice {
    /// Serializer for Duo authenticator devices
    pub fn new(pk: i32, name: String, user: models::GroupMember) -> DuoDevice {
        DuoDevice { pk, name, user }
    }
}
