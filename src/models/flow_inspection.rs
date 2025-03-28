/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// FlowInspection : Serializer for inspect endpoint
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlowInspection {
    #[serde(rename = "plans")]
    pub plans: Vec<models::FlowInspectorPlan>,
    #[serde(rename = "current_plan", skip_serializing_if = "Option::is_none")]
    pub current_plan: Option<models::FlowInspectorPlan>,
    #[serde(rename = "is_completed")]
    pub is_completed: bool,
}

impl FlowInspection {
    /// Serializer for inspect endpoint
    pub fn new(plans: Vec<models::FlowInspectorPlan>, is_completed: bool) -> FlowInspection {
        FlowInspection {
            plans,
            current_plan: None,
            is_completed,
        }
    }
}
