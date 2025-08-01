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

/// PatchedInitialPermissionsRequest : InitialPermissions serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedInitialPermissionsRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<models::InitialPermissionsModeEnum>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<uuid::Uuid>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<i32>>,
}

impl PatchedInitialPermissionsRequest {
    /// InitialPermissions serializer
    pub fn new() -> PatchedInitialPermissionsRequest {
        PatchedInitialPermissionsRequest {
            name: None,
            mode: None,
            role: None,
            permissions: None,
        }
    }
}
