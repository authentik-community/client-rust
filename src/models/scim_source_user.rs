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

/// ScimSourceUser : SCIMSourceUser Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScimSourceUser {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "user")]
    pub user: i32,
    #[serde(rename = "user_obj")]
    pub user_obj: models::GroupMember,
    #[serde(rename = "source")]
    pub source: uuid::Uuid,
    #[serde(
        rename = "attributes",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<Option<serde_json::Value>>,
}

impl ScimSourceUser {
    /// SCIMSourceUser Serializer
    pub fn new(id: String, user: i32, user_obj: models::GroupMember, source: uuid::Uuid) -> ScimSourceUser {
        ScimSourceUser {
            id,
            user,
            user_obj,
            source,
            attributes: None,
        }
    }
}
