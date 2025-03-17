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

/// PolicyTestRequest : Test policy execution for a user with context
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyTestRequest {
    #[serde(rename = "user")]
    pub user: i32,
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl PolicyTestRequest {
    /// Test policy execution for a user with context
    pub fn new(user: i32) -> PolicyTestRequest {
        PolicyTestRequest { user, context: None }
    }
}
