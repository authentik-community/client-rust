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

/// PatchedEndpointRequest : Endpoint Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedEndpointRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<i32>,
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<models::ProtocolEnum>,
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(
        rename = "settings",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub settings: Option<Option<serde_json::Value>>,
    #[serde(rename = "property_mappings", skip_serializing_if = "Option::is_none")]
    pub property_mappings: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "auth_mode", skip_serializing_if = "Option::is_none")]
    pub auth_mode: Option<models::AuthModeEnum>,
    #[serde(rename = "maximum_connections", skip_serializing_if = "Option::is_none")]
    pub maximum_connections: Option<i32>,
}

impl PatchedEndpointRequest {
    /// Endpoint Serializer
    pub fn new() -> PatchedEndpointRequest {
        PatchedEndpointRequest {
            name: None,
            provider: None,
            protocol: None,
            host: None,
            settings: None,
            property_mappings: None,
            auth_mode: None,
            maximum_connections: None,
        }
    }
}
