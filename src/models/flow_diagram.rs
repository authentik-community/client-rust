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

/// FlowDiagram : response of the flow's diagram action
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlowDiagram {
    #[serde(rename = "diagram")]
    pub diagram: String,
}

impl FlowDiagram {
    /// response of the flow's diagram action
    pub fn new(diagram: String) -> FlowDiagram {
        FlowDiagram { diagram }
    }
}
