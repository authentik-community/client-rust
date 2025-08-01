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

/// PatchedNotificationRuleRequest : NotificationRule Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedNotificationRuleRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Select which transports should be used to notify the user. If none are selected, the notification will only be shown in the authentik UI.
    #[serde(rename = "transports", skip_serializing_if = "Option::is_none")]
    pub transports: Option<Vec<uuid::Uuid>>,
    /// Controls which severity level the created notifications will have.
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<models::SeverityEnum>,
    /// Define which group of users this notification should be sent and shown to. If left empty, Notification won't ben sent.
    #[serde(
        rename = "destination_group",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_group: Option<Option<uuid::Uuid>>,
    /// When enabled, notification will be sent to user the user that triggered the event.When destination_group is configured, notification is sent to both.
    #[serde(rename = "destination_event_user", skip_serializing_if = "Option::is_none")]
    pub destination_event_user: Option<bool>,
}

impl PatchedNotificationRuleRequest {
    /// NotificationRule Serializer
    pub fn new() -> PatchedNotificationRuleRequest {
        PatchedNotificationRuleRequest {
            name: None,
            transports: None,
            severity: None,
            destination_group: None,
            destination_event_user: None,
        }
    }
}
