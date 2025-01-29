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

/// RoleAssignedObjectPermission : Roles assigned object permission serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignedObjectPermission {
    #[serde(rename = "role_pk")]
    pub role_pk: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "permissions")]
    pub permissions: Vec<models::RoleObjectPermission>,
}

impl RoleAssignedObjectPermission {
    /// Roles assigned object permission serializer
    pub fn new(
        role_pk: String,
        name: String,
        permissions: Vec<models::RoleObjectPermission>,
    ) -> RoleAssignedObjectPermission {
        RoleAssignedObjectPermission {
            role_pk,
            name,
            permissions,
        }
    }
}
