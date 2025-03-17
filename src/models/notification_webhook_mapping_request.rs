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

/// NotificationWebhookMappingRequest : NotificationWebhookMapping Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationWebhookMappingRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "expression")]
    pub expression: String,
}

impl NotificationWebhookMappingRequest {
    /// NotificationWebhookMapping Serializer
    pub fn new(name: String, expression: String) -> NotificationWebhookMappingRequest {
        NotificationWebhookMappingRequest { name, expression }
    }
}
