/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.5
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// RedirectStageRequest : RedirectStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedirectStageRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
    #[serde(rename = "keep_context", skip_serializing_if = "Option::is_none")]
    pub keep_context: Option<bool>,
    #[serde(rename = "mode")]
    pub mode: models::RedirectStageModeEnum,
    #[serde(rename = "target_static", skip_serializing_if = "Option::is_none")]
    pub target_static: Option<String>,
    #[serde(
        rename = "target_flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_flow: Option<Option<uuid::Uuid>>,
}

impl RedirectStageRequest {
    /// RedirectStage Serializer
    pub fn new(name: String, mode: models::RedirectStageModeEnum) -> RedirectStageRequest {
        RedirectStageRequest {
            name,
            flow_set: None,
            keep_context: None,
            mode,
            target_static: None,
            target_flow: None,
        }
    }
}