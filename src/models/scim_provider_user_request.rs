/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ScimProviderUserRequest : SCIMProviderUser Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScimProviderUserRequest {
    #[serde(rename = "scim_id")]
    pub scim_id: String,
    #[serde(rename = "user")]
    pub user: i32,
    #[serde(rename = "provider")]
    pub provider: i32,
}

impl ScimProviderUserRequest {
    /// SCIMProviderUser Serializer
    pub fn new(scim_id: String, user: i32, provider: i32) -> ScimProviderUserRequest {
        ScimProviderUserRequest {
            scim_id,
            user,
            provider,
        }
    }
}
