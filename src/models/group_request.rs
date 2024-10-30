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

/// GroupRequest : Group Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupRequest {
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
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<i32>>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<uuid::Uuid>>,
}

impl GroupRequest {
    /// Group Serializer
    pub fn new(name: String) -> GroupRequest {
        GroupRequest {
            name,
            is_superuser: None,
            parent: None,
            users: None,
            attributes: None,
            roles: None,
        }
    }
}
