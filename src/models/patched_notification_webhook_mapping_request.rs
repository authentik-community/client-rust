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

/// PatchedNotificationWebhookMappingRequest : NotificationWebhookMapping Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedNotificationWebhookMappingRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "expression", skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
}

impl PatchedNotificationWebhookMappingRequest {
    /// NotificationWebhookMapping Serializer
    pub fn new() -> PatchedNotificationWebhookMappingRequest {
        PatchedNotificationWebhookMappingRequest {
            name: None,
            expression: None,
        }
    }
}
