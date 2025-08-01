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

/// PatchedPromptStageRequest : PromptStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedPromptStageRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "validation_policies", skip_serializing_if = "Option::is_none")]
    pub validation_policies: Option<Vec<uuid::Uuid>>,
}

impl PatchedPromptStageRequest {
    /// PromptStage Serializer
    pub fn new() -> PatchedPromptStageRequest {
        PatchedPromptStageRequest {
            name: None,
            flow_set: None,
            fields: None,
            validation_policies: None,
        }
    }
}
