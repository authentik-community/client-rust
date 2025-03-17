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

/// PolicyRequest : Policy Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyRequest {
    #[serde(rename = "name")]
    pub name: String,
    /// When this option is enabled, all executions of this policy will be logged. By default, only execution errors are logged.
    #[serde(rename = "execution_logging", skip_serializing_if = "Option::is_none")]
    pub execution_logging: Option<bool>,
}

impl PolicyRequest {
    /// Policy Serializer
    pub fn new(name: String) -> PolicyRequest {
        PolicyRequest {
            name,
            execution_logging: None,
        }
    }
}
