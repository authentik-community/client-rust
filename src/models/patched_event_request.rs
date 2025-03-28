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

/// PatchedEventRequest : Event Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedEventRequest {
    #[serde(
        rename = "user",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub user: Option<Option<serde_json::Value>>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<models::EventActions>,
    #[serde(rename = "app", skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,
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

impl PatchedEventRequest {
    /// Event Serializer
    pub fn new() -> PatchedEventRequest {
        PatchedEventRequest {
            user: None,
            action: None,
            app: None,
            context: None,
            client_ip: None,
            expires: None,
            brand: None,
        }
    }
}
