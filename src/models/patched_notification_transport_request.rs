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

/// PatchedNotificationTransportRequest : NotificationTransport Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedNotificationTransportRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<models::NotificationTransportModeEnum>,
    #[serde(rename = "webhook_url", skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    #[serde(
        rename = "webhook_mapping",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub webhook_mapping: Option<Option<uuid::Uuid>>,
    /// Only send notification once, for example when sending a webhook into a chat channel.
    #[serde(rename = "send_once", skip_serializing_if = "Option::is_none")]
    pub send_once: Option<bool>,
}

impl PatchedNotificationTransportRequest {
    /// NotificationTransport Serializer
    pub fn new() -> PatchedNotificationTransportRequest {
        PatchedNotificationTransportRequest {
            name: None,
            mode: None,
            webhook_url: None,
            webhook_mapping: None,
            send_once: None,
        }
    }
}
