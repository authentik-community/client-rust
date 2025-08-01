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

/// NotificationWebhookMapping : NotificationWebhookMapping Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationWebhookMapping {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "expression")]
    pub expression: String,
}

impl NotificationWebhookMapping {
    /// NotificationWebhookMapping Serializer
    pub fn new(pk: uuid::Uuid, name: String, expression: String) -> NotificationWebhookMapping {
        NotificationWebhookMapping { pk, name, expression }
    }
}
