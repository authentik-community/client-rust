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

/// RoleRequest : Role serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleRequest {
    #[serde(rename = "name")]
    pub name: String,
}

impl RoleRequest {
    /// Role serializer
    pub fn new(name: String) -> RoleRequest {
        RoleRequest { name }
    }
}
