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

/// PatchedUserSamlSourceConnectionRequest : User source connection
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedUserSamlSourceConnectionRequest {
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<i32>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<uuid::Uuid>,
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

impl PatchedUserSamlSourceConnectionRequest {
    /// User source connection
    pub fn new() -> PatchedUserSamlSourceConnectionRequest {
        PatchedUserSamlSourceConnectionRequest {
            user: None,
            source: None,
            identifier: None,
        }
    }
}
