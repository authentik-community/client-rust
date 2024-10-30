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

/// PatchedUserKerberosSourceConnectionRequest : Kerberos Source Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedUserKerberosSourceConnectionRequest {
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<i32>,
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

impl PatchedUserKerberosSourceConnectionRequest {
    /// Kerberos Source Serializer
    pub fn new() -> PatchedUserKerberosSourceConnectionRequest {
        PatchedUserKerberosSourceConnectionRequest {
            user: None,
            identifier: None,
        }
    }
}
