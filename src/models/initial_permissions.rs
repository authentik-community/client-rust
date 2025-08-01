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

/// InitialPermissions : InitialPermissions serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InitialPermissions {
    #[serde(rename = "pk")]
    pub pk: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "mode")]
    pub mode: models::InitialPermissionsModeEnum,
    #[serde(rename = "role")]
    pub role: uuid::Uuid,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<i32>>,
    #[serde(rename = "permissions_obj")]
    pub permissions_obj: Vec<models::Permission>,
}

impl InitialPermissions {
    /// InitialPermissions serializer
    pub fn new(
        pk: i32,
        name: String,
        mode: models::InitialPermissionsModeEnum,
        role: uuid::Uuid,
        permissions_obj: Vec<models::Permission>,
    ) -> InitialPermissions {
        InitialPermissions {
            pk,
            name,
            mode,
            role,
            permissions: None,
            permissions_obj,
        }
    }
}
