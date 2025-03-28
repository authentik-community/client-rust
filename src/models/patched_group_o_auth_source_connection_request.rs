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

/// PatchedGroupOAuthSourceConnectionRequest : OAuth Group-Source connection Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedGroupOAuthSourceConnectionRequest {
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<uuid::Uuid>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<uuid::Uuid>,
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

impl PatchedGroupOAuthSourceConnectionRequest {
    /// OAuth Group-Source connection Serializer
    pub fn new() -> PatchedGroupOAuthSourceConnectionRequest {
        PatchedGroupOAuthSourceConnectionRequest {
            group: None,
            source: None,
            identifier: None,
        }
    }
}
