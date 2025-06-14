/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.6.1
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// NotificationRule : NotificationRule Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationRule {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    /// Select which transports should be used to notify the user. If none are selected, the notification will only be shown in the authentik UI.
    #[serde(rename = "transports", skip_serializing_if = "Option::is_none")]
    pub transports: Option<Vec<uuid::Uuid>>,
    /// Controls which severity level the created notifications will have.
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<models::SeverityEnum>,
    /// Define which group of users this notification should be sent and shown to. If left empty, Notification won't ben sent.
    #[serde(
        rename = "group",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub group: Option<Option<uuid::Uuid>>,
    #[serde(rename = "group_obj")]
    pub group_obj: models::Group,
}

impl NotificationRule {
    /// NotificationRule Serializer
    pub fn new(pk: uuid::Uuid, name: String, group_obj: models::Group) -> NotificationRule {
        NotificationRule {
            pk,
            name,
            transports: None,
            severity: None,
            group: None,
            group_obj,
        }
    }
}
