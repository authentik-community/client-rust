/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UserAssignedObjectPermission : Users assigned object permission serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAssignedObjectPermission {
    #[serde(rename = "pk")]
    pub pk: i32,
    /// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
    #[serde(rename = "username")]
    pub username: String,
    /// User's display name.
    #[serde(rename = "name")]
    pub name: String,
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
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "uid")]
    pub uid: String,
    #[serde(rename = "permissions")]
    pub permissions: Vec<models::UserObjectPermission>,
    #[serde(rename = "is_superuser")]
    pub is_superuser: bool,
}

impl UserAssignedObjectPermission {
    /// Users assigned object permission serializer
    pub fn new(
        pk: i32,
        username: String,
        name: String,
        uid: String,
        permissions: Vec<models::UserObjectPermission>,
        is_superuser: bool,
    ) -> UserAssignedObjectPermission {
        UserAssignedObjectPermission {
            pk,
            username,
            name,
            is_active: None,
            last_login: None,
            email: None,
            attributes: None,
            uid,
            permissions,
            is_superuser,
        }
    }
}
