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

/// ExpressionPolicyRequest : Group Membership Policy Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExpressionPolicyRequest {
    #[serde(rename = "name")]
    pub name: String,
    /// When this option is enabled, all executions of this policy will be logged. By default, only execution errors are logged.
    #[serde(rename = "execution_logging", skip_serializing_if = "Option::is_none")]
    pub execution_logging: Option<bool>,
    #[serde(rename = "expression")]
    pub expression: String,
}

impl ExpressionPolicyRequest {
    /// Group Membership Policy Serializer
    pub fn new(name: String, expression: String) -> ExpressionPolicyRequest {
        ExpressionPolicyRequest {
            name,
            execution_logging: None,
            expression,
        }
    }
}
