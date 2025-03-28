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

/// ScimProviderUser : SCIMProviderUser Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScimProviderUser {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "scim_id")]
    pub scim_id: String,
    #[serde(rename = "user")]
    pub user: i32,
    #[serde(rename = "user_obj")]
    pub user_obj: models::GroupMember,
    #[serde(rename = "provider")]
    pub provider: i32,
    #[serde(rename = "attributes", deserialize_with = "Option::deserialize")]
    pub attributes: Option<serde_json::Value>,
}

impl ScimProviderUser {
    /// SCIMProviderUser Serializer
    pub fn new(
        id: uuid::Uuid,
        scim_id: String,
        user: i32,
        user_obj: models::GroupMember,
        provider: i32,
        attributes: Option<serde_json::Value>,
    ) -> ScimProviderUser {
        ScimProviderUser {
            id,
            scim_id,
            user,
            user_obj,
            provider,
            attributes,
        }
    }
}
