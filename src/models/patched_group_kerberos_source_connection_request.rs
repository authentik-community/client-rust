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

/// PatchedGroupKerberosSourceConnectionRequest : Group Source Connection
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedGroupKerberosSourceConnectionRequest {
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<uuid::Uuid>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<uuid::Uuid>,
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

impl PatchedGroupKerberosSourceConnectionRequest {
    /// Group Source Connection
    pub fn new() -> PatchedGroupKerberosSourceConnectionRequest {
        PatchedGroupKerberosSourceConnectionRequest {
            group: None,
            source: None,
            identifier: None,
        }
    }
}
