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

/// ScimProviderGroupRequest : SCIMProviderGroup Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScimProviderGroupRequest {
    #[serde(rename = "scim_id")]
    pub scim_id: String,
    #[serde(rename = "group")]
    pub group: uuid::Uuid,
    #[serde(rename = "provider")]
    pub provider: i32,
}

impl ScimProviderGroupRequest {
    /// SCIMProviderGroup Serializer
    pub fn new(scim_id: String, group: uuid::Uuid, provider: i32) -> ScimProviderGroupRequest {
        ScimProviderGroupRequest {
            scim_id,
            group,
            provider,
        }
    }
}
