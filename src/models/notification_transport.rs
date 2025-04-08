/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.4
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// NotificationTransport : NotificationTransport Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationTransport {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<models::NotificationTransportModeEnum>,
    /// Return selected mode with a UI Label
    #[serde(rename = "mode_verbose")]
    pub mode_verbose: String,
    #[serde(rename = "webhook_url", skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    /// Customize the body of the request. Mapping should return data that is JSON-serializable.
    #[serde(
        rename = "webhook_mapping_body",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub webhook_mapping_body: Option<Option<uuid::Uuid>>,
    /// Configure additional headers to be sent. Mapping should return a dictionary of key-value pairs
    #[serde(
        rename = "webhook_mapping_headers",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub webhook_mapping_headers: Option<Option<uuid::Uuid>>,
    /// Only send notification once, for example when sending a webhook into a chat channel.
    #[serde(rename = "send_once", skip_serializing_if = "Option::is_none")]
    pub send_once: Option<bool>,
}

impl NotificationTransport {
    /// NotificationTransport Serializer
    pub fn new(pk: uuid::Uuid, name: String, mode_verbose: String) -> NotificationTransport {
        NotificationTransport {
            pk,
            name,
            mode: None,
            mode_verbose,
            webhook_url: None,
            webhook_mapping_body: None,
            webhook_mapping_headers: None,
            send_once: None,
        }
    }
}
