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

/// GroupOAuthSourceConnectionRequest : Group Source Connection
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupOAuthSourceConnectionRequest {
    #[serde(rename = "group")]
    pub group: uuid::Uuid,
    #[serde(rename = "source")]
    pub source: uuid::Uuid,
    #[serde(rename = "identifier")]
    pub identifier: String,
}

impl GroupOAuthSourceConnectionRequest {
    /// Group Source Connection
    pub fn new(group: uuid::Uuid, source: uuid::Uuid, identifier: String) -> GroupOAuthSourceConnectionRequest {
        GroupOAuthSourceConnectionRequest {
            group,
            source,
            identifier,
        }
    }
}
