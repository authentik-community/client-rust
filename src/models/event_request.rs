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

/// EventRequest : Event Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventRequest {
    #[serde(
        rename = "user",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub user: Option<Option<serde_json::Value>>,
    #[serde(rename = "action")]
    pub action: models::EventActions,
    #[serde(rename = "app")]
    pub app: String,
    #[serde(
        rename = "context",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub context: Option<Option<serde_json::Value>>,
    #[serde(
        rename = "client_ip",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub client_ip: Option<Option<String>>,
    #[serde(rename = "expires", skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    #[serde(
        rename = "brand",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub brand: Option<Option<serde_json::Value>>,
}

impl EventRequest {
    /// Event Serializer
    pub fn new(action: models::EventActions, app: String) -> EventRequest {
        EventRequest {
            user: None,
            action,
            app,
            context: None,
            client_ip: None,
            expires: None,
            brand: None,
        }
    }
}
