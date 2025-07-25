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

/// PromptChallenge : Initial challenge being sent, define fields
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PromptChallenge {
    #[serde(rename = "flow_info", skip_serializing_if = "Option::is_none")]
    pub flow_info: Option<models::ContextualFlowInfo>,
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "response_errors", skip_serializing_if = "Option::is_none")]
    pub response_errors: Option<std::collections::HashMap<String, Vec<models::ErrorDetail>>>,
    #[serde(rename = "fields")]
    pub fields: Vec<models::StagePrompt>,
}

impl PromptChallenge {
    /// Initial challenge being sent, define fields
    pub fn new(fields: Vec<models::StagePrompt>) -> PromptChallenge {
        PromptChallenge {
            flow_info: None,
            component: None,
            response_errors: None,
            fields,
        }
    }
}
