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

/// PatchedFlowStageBindingRequest : FlowStageBinding Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedFlowStageBindingRequest {
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<uuid::Uuid>,
    #[serde(rename = "stage", skip_serializing_if = "Option::is_none")]
    pub stage: Option<uuid::Uuid>,
    /// Evaluate policies during the Flow planning process.
    #[serde(rename = "evaluate_on_plan", skip_serializing_if = "Option::is_none")]
    pub evaluate_on_plan: Option<bool>,
    /// Evaluate policies when the Stage is presented to the user.
    #[serde(rename = "re_evaluate_policies", skip_serializing_if = "Option::is_none")]
    pub re_evaluate_policies: Option<bool>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(rename = "policy_engine_mode", skip_serializing_if = "Option::is_none")]
    pub policy_engine_mode: Option<models::PolicyEngineMode>,
    /// Configure how the flow executor should handle an invalid response to a challenge. RETRY returns the error message and a similar challenge to the executor. RESTART restarts the flow from the beginning, and RESTART_WITH_CONTEXT restarts the flow while keeping the current context.
    #[serde(rename = "invalid_response_action", skip_serializing_if = "Option::is_none")]
    pub invalid_response_action: Option<models::InvalidResponseActionEnum>,
}

impl PatchedFlowStageBindingRequest {
    /// FlowStageBinding Serializer
    pub fn new() -> PatchedFlowStageBindingRequest {
        PatchedFlowStageBindingRequest {
            target: None,
            stage: None,
            evaluate_on_plan: None,
            re_evaluate_policies: None,
            order: None,
            policy_engine_mode: None,
            invalid_response_action: None,
        }
    }
}
