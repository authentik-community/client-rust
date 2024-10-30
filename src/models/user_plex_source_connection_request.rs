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

/// UserPlexSourceConnectionRequest : Plex Source connection Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserPlexSourceConnectionRequest {
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "plex_token")]
    pub plex_token: String,
}

impl UserPlexSourceConnectionRequest {
    /// Plex Source connection Serializer
    pub fn new(identifier: String, plex_token: String) -> UserPlexSourceConnectionRequest {
        UserPlexSourceConnectionRequest { identifier, plex_token }
    }
}
