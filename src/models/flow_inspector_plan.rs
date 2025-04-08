/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.4
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// FlowInspectorPlan : Serializer for an active FlowPlan
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlowInspectorPlan {
    #[serde(rename = "current_stage")]
    pub current_stage: models::FlowStageBinding,
    #[serde(rename = "next_planned_stage")]
    pub next_planned_stage: models::FlowStageBinding,
    /// Get the plan's context, sanitized
    #[serde(rename = "plan_context")]
    pub plan_context: std::collections::HashMap<String, serde_json::Value>,
    /// Get a unique session ID
    #[serde(rename = "session_id")]
    pub session_id: String,
}

impl FlowInspectorPlan {
    /// Serializer for an active FlowPlan
    pub fn new(
        current_stage: models::FlowStageBinding,
        next_planned_stage: models::FlowStageBinding,
        plan_context: std::collections::HashMap<String, serde_json::Value>,
        session_id: String,
    ) -> FlowInspectorPlan {
        FlowInspectorPlan {
            current_stage,
            next_planned_stage,
            plan_context,
            session_id,
        }
    }
}
