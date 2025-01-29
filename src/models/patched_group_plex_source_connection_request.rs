/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PatchedGroupPlexSourceConnectionRequest : Plex Group-Source connection Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedGroupPlexSourceConnectionRequest {
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<uuid::Uuid>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<uuid::Uuid>,
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

impl PatchedGroupPlexSourceConnectionRequest {
    /// Plex Group-Source connection Serializer
    pub fn new() -> PatchedGroupPlexSourceConnectionRequest {
        PatchedGroupPlexSourceConnectionRequest {
            group: None,
            source: None,
            identifier: None,
        }
    }
}
