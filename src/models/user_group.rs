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

/// UserGroup : Simplified Group Serializer for user's groups
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserGroup {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    /// Get a numerical, int32 ID for the group
    #[serde(rename = "num_pk")]
    pub num_pk: i32,
    #[serde(rename = "name")]
    pub name: String,
    /// Users added to this group will be superusers.
    #[serde(rename = "is_superuser", skip_serializing_if = "Option::is_none")]
    pub is_superuser: Option<bool>,
    #[serde(
        rename = "parent",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent: Option<Option<uuid::Uuid>>,
    #[serde(rename = "parent_name", deserialize_with = "Option::deserialize")]
    pub parent_name: Option<String>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl UserGroup {
    /// Simplified Group Serializer for user's groups
    pub fn new(pk: uuid::Uuid, num_pk: i32, name: String, parent_name: Option<String>) -> UserGroup {
        UserGroup {
            pk,
            num_pk,
            name,
            is_superuser: None,
            parent: None,
            parent_name,
            attributes: None,
        }
    }
}
