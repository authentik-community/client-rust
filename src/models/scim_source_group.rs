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

/// ScimSourceGroup : SCIMSourceGroup Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScimSourceGroup {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "external_id")]
    pub external_id: String,
    #[serde(rename = "group")]
    pub group: uuid::Uuid,
    #[serde(rename = "group_obj")]
    pub group_obj: models::UserGroup,
    #[serde(rename = "source")]
    pub source: uuid::Uuid,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl ScimSourceGroup {
    /// SCIMSourceGroup Serializer
    pub fn new(
        external_id: String,
        group: uuid::Uuid,
        group_obj: models::UserGroup,
        source: uuid::Uuid,
    ) -> ScimSourceGroup {
        ScimSourceGroup {
            id: None,
            external_id,
            group,
            group_obj,
            source,
            attributes: None,
        }
    }
}
