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

/// PropertyMappingTestRequest : Test property mapping execution for a user/group with context
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyMappingTestRequest {
    #[serde(
        rename = "user",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub user: Option<Option<i32>>,
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(
        rename = "group",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub group: Option<Option<uuid::Uuid>>,
}

impl PropertyMappingTestRequest {
    /// Test property mapping execution for a user/group with context
    pub fn new() -> PropertyMappingTestRequest {
        PropertyMappingTestRequest {
            user: None,
            context: None,
            group: None,
        }
    }
}
