/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.2
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PatchedScimSourceGroupRequest : SCIMSourceGroup Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedScimSourceGroupRequest {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<uuid::Uuid>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<uuid::Uuid>,
    #[serde(
        rename = "attributes",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<Option<serde_json::Value>>,
}

impl PatchedScimSourceGroupRequest {
    /// SCIMSourceGroup Serializer
    pub fn new() -> PatchedScimSourceGroupRequest {
        PatchedScimSourceGroupRequest {
            id: None,
            group: None,
            source: None,
            attributes: None,
        }
    }
}
