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

/// UserGroupRequest : Simplified Group Serializer for user's groups
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserGroupRequest {
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
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl UserGroupRequest {
    /// Simplified Group Serializer for user's groups
    pub fn new(name: String) -> UserGroupRequest {
        UserGroupRequest {
            name,
            is_superuser: None,
            parent: None,
            attributes: None,
        }
    }
}
