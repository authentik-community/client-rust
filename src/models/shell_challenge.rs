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

/// ShellChallenge : challenge type to render HTML as-is
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShellChallenge {
    #[serde(rename = "flow_info", skip_serializing_if = "Option::is_none")]
    pub flow_info: Option<models::ContextualFlowInfo>,
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "response_errors", skip_serializing_if = "Option::is_none")]
    pub response_errors: Option<std::collections::HashMap<String, Vec<models::ErrorDetail>>>,
    #[serde(rename = "body")]
    pub body: String,
}

impl ShellChallenge {
    /// challenge type to render HTML as-is
    pub fn new(body: String) -> ShellChallenge {
        ShellChallenge {
            flow_info: None,
            component: None,
            response_errors: None,
            body,
        }
    }
}
