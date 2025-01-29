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

/// PatchedExtraUserObjectPermissionRequest : User permission with additional object-related data
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedExtraUserObjectPermissionRequest {
    #[serde(rename = "object_pk", skip_serializing_if = "Option::is_none")]
    pub object_pk: Option<String>,
}

impl PatchedExtraUserObjectPermissionRequest {
    /// User permission with additional object-related data
    pub fn new() -> PatchedExtraUserObjectPermissionRequest {
        PatchedExtraUserObjectPermissionRequest { object_pk: None }
    }
}
