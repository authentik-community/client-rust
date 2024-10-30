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

/// PatchedPermissionAssignRequest : Request to assign a new permission
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedPermissionAssignRequest {
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<models::ModelEnum>,
    #[serde(rename = "object_pk", skip_serializing_if = "Option::is_none")]
    pub object_pk: Option<String>,
}

impl PatchedPermissionAssignRequest {
    /// Request to assign a new permission
    pub fn new() -> PatchedPermissionAssignRequest {
        PatchedPermissionAssignRequest {
            permissions: None,
            model: None,
            object_pk: None,
        }
    }
}
