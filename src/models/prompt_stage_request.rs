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

/// PromptStageRequest : PromptStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PromptStageRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
    #[serde(rename = "fields")]
    pub fields: Vec<uuid::Uuid>,
    #[serde(rename = "validation_policies", skip_serializing_if = "Option::is_none")]
    pub validation_policies: Option<Vec<uuid::Uuid>>,
}

impl PromptStageRequest {
    /// PromptStage Serializer
    pub fn new(name: String, fields: Vec<uuid::Uuid>) -> PromptStageRequest {
        PromptStageRequest {
            name,
            flow_set: None,
            fields,
            validation_policies: None,
        }
    }
}
