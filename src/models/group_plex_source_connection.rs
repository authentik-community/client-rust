/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.5
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// GroupPlexSourceConnection : Plex Group-Source connection Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupPlexSourceConnection {
    #[serde(rename = "pk")]
    pub pk: i32,
    #[serde(rename = "group")]
    pub group: uuid::Uuid,
    #[serde(rename = "source")]
    pub source: models::Source,
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "created")]
    pub created: String,
}

impl GroupPlexSourceConnection {
    /// Plex Group-Source connection Serializer
    pub fn new(
        pk: i32,
        group: uuid::Uuid,
        source: models::Source,
        identifier: String,
        created: String,
    ) -> GroupPlexSourceConnection {
        GroupPlexSourceConnection {
            pk,
            group,
            source,
            identifier,
            created,
        }
    }
}
