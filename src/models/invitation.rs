/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.5
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Invitation : Invitation Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Invitation {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(
        rename = "expires",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub expires: Option<Option<String>>,
    #[serde(rename = "fixed_data", skip_serializing_if = "Option::is_none")]
    pub fixed_data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "created_by")]
    pub created_by: models::GroupMember,
    /// When enabled, the invitation will be deleted after usage.
    #[serde(rename = "single_use", skip_serializing_if = "Option::is_none")]
    pub single_use: Option<bool>,
    /// When set, only the configured flow can use this invitation.
    #[serde(
        rename = "flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub flow: Option<Option<uuid::Uuid>>,
    #[serde(rename = "flow_obj")]
    pub flow_obj: models::Flow,
}

impl Invitation {
    /// Invitation Serializer
    pub fn new(pk: uuid::Uuid, name: String, created_by: models::GroupMember, flow_obj: models::Flow) -> Invitation {
        Invitation {
            pk,
            name,
            expires: None,
            fixed_data: None,
            created_by,
            single_use: None,
            flow: None,
            flow_obj,
        }
    }
}
