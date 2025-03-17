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

/// Notification : Notification Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    #[serde(rename = "severity")]
    pub severity: models::SeverityEnum,
    #[serde(rename = "body")]
    pub body: String,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub event: Option<models::Event>,
    #[serde(rename = "seen", skip_serializing_if = "Option::is_none")]
    pub seen: Option<bool>,
}

impl Notification {
    /// Notification Serializer
    pub fn new(pk: uuid::Uuid, severity: models::SeverityEnum, body: String, created: String) -> Notification {
        Notification {
            pk,
            severity,
            body,
            created,
            event: None,
            seen: None,
        }
    }
}
