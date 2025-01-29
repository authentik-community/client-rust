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

/// DummyStageRequest : DummyStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DummyStageRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
    #[serde(rename = "throw_error", skip_serializing_if = "Option::is_none")]
    pub throw_error: Option<bool>,
}

impl DummyStageRequest {
    /// DummyStage Serializer
    pub fn new(name: String) -> DummyStageRequest {
        DummyStageRequest {
            name,
            flow_set: None,
            throw_error: None,
        }
    }
}
