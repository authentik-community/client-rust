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

/// GroupPlexSourceConnectionRequest : Plex Group-Source connection Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupPlexSourceConnectionRequest {
    #[serde(rename = "group")]
    pub group: uuid::Uuid,
    #[serde(rename = "source")]
    pub source: uuid::Uuid,
    #[serde(rename = "identifier")]
    pub identifier: String,
}

impl GroupPlexSourceConnectionRequest {
    /// Plex Group-Source connection Serializer
    pub fn new(group: uuid::Uuid, source: uuid::Uuid, identifier: String) -> GroupPlexSourceConnectionRequest {
        GroupPlexSourceConnectionRequest {
            group,
            source,
            identifier,
        }
    }
}
