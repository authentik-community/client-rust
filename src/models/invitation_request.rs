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

/// InvitationRequest : Invitation Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InvitationRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(
        rename = "expires",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub expires: Option<Option<String>>,
    #[serde(rename = "fixed_data", skip_serializing_if = "Option::is_none")]
    pub fixed_data: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// When enabled, the invitation will be deleted after usage.
    #[serde(rename = "single_use", skip_serializing_if = "Option::is_none")]
    pub single_use: Option<bool>,
    /// When set, only the configured flow can use this invitation.
    #[serde(
        rename = "flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub flow: Option<Option<uuid::Uuid>>,
}

impl InvitationRequest {
    /// Invitation Serializer
    pub fn new(name: String) -> InvitationRequest {
        InvitationRequest {
            name,
            expires: None,
            fixed_data: None,
            single_use: None,
            flow: None,
        }
    }
}
