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

/// UserSelf : User Serializer for information a user can retrieve about themselves
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSelf {
    #[serde(rename = "pk")]
    pub pk: i32,
    /// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
    #[serde(rename = "username")]
    pub username: String,
    /// User's display name.
    #[serde(rename = "name")]
    pub name: String,
    /// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
    #[serde(rename = "is_active")]
    pub is_active: bool,
    #[serde(rename = "is_superuser")]
    pub is_superuser: bool,
    #[serde(rename = "groups")]
    pub groups: Vec<models::UserSelfGroups>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// User's avatar, either a http/https URL or a data URI
    #[serde(rename = "avatar")]
    pub avatar: String,
    #[serde(rename = "uid")]
    pub uid: String,
    /// Get user settings with brand and group settings applied
    #[serde(rename = "settings")]
    pub settings: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::UserTypeEnum>,
    /// Get all system permissions assigned to the user
    #[serde(rename = "system_permissions")]
    pub system_permissions: Vec<String>,
}

impl UserSelf {
    /// User Serializer for information a user can retrieve about themselves
    pub fn new(
        pk: i32,
        username: String,
        name: String,
        is_active: bool,
        is_superuser: bool,
        groups: Vec<models::UserSelfGroups>,
        avatar: String,
        uid: String,
        settings: std::collections::HashMap<String, serde_json::Value>,
        system_permissions: Vec<String>,
    ) -> UserSelf {
        UserSelf {
            pk,
            username,
            name,
            is_active,
            is_superuser,
            groups,
            email: None,
            avatar,
            uid,
            settings,
            r#type: None,
            system_permissions,
        }
    }
}
