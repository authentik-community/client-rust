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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImpersonationRequest {
    #[serde(rename = "reason")]
    pub reason: String,
}

impl ImpersonationRequest {
    pub fn new(reason: String) -> ImpersonationRequest {
        ImpersonationRequest { reason }
    }
}
