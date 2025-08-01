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

/// SmsDevice : Serializer for sms authenticator devices
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsDevice {
    /// The human-readable name of this device.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "pk")]
    pub pk: i32,
    #[serde(rename = "phone_number")]
    pub phone_number: String,
    #[serde(rename = "user")]
    pub user: models::GroupMember,
}

impl SmsDevice {
    /// Serializer for sms authenticator devices
    pub fn new(name: String, pk: i32, phone_number: String, user: models::GroupMember) -> SmsDevice {
        SmsDevice {
            name,
            pk,
            phone_number,
            user,
        }
    }
}
