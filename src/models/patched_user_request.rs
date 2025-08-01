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

/// PatchedUserRequest : User Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedUserRequest {
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// User's display name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(
        rename = "last_login",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_login: Option<Option<String>>,
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::UserTypeEnum>,
}

impl PatchedUserRequest {
    /// User Serializer
    pub fn new() -> PatchedUserRequest {
        PatchedUserRequest {
            username: None,
            name: None,
            is_active: None,
            last_login: None,
            groups: None,
            email: None,
            attributes: None,
            path: None,
            r#type: None,
        }
    }
}
