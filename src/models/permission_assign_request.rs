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

/// PermissionAssignRequest : Request to assign a new permission
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PermissionAssignRequest {
    #[serde(rename = "permissions")]
    pub permissions: Vec<String>,
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<models::ModelEnum>,
    #[serde(rename = "object_pk", skip_serializing_if = "Option::is_none")]
    pub object_pk: Option<String>,
}

impl PermissionAssignRequest {
    /// Request to assign a new permission
    pub fn new(permissions: Vec<String>) -> PermissionAssignRequest {
        PermissionAssignRequest {
            permissions,
            model: None,
            object_pk: None,
        }
    }
}
