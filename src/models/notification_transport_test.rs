/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.0
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// NotificationTransportTest : Notification test serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationTransportTest {
    #[serde(rename = "messages")]
    pub messages: Vec<String>,
}

impl NotificationTransportTest {
    /// Notification test serializer
    pub fn new(messages: Vec<String>) -> NotificationTransportTest {
        NotificationTransportTest { messages }
    }
}
