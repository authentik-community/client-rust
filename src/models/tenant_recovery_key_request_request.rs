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

/// TenantRecoveryKeyRequestRequest : Tenant recovery key creation request serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantRecoveryKeyRequestRequest {
    #[serde(rename = "user")]
    pub user: String,
    #[serde(rename = "duration_days")]
    pub duration_days: i32,
}

impl TenantRecoveryKeyRequestRequest {
    /// Tenant recovery key creation request serializer
    pub fn new(user: String, duration_days: i32) -> TenantRecoveryKeyRequestRequest {
        TenantRecoveryKeyRequestRequest { user, duration_days }
    }
}
