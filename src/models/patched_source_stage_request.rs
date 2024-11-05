/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.1
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PatchedSourceStageRequest : SourceStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedSourceStageRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<uuid::Uuid>,
    /// Amount of time a user can take to return from the source to continue the flow (Format: hours=-1;minutes=-2;seconds=-3)
    #[serde(rename = "resume_timeout", skip_serializing_if = "Option::is_none")]
    pub resume_timeout: Option<String>,
}

impl PatchedSourceStageRequest {
    /// SourceStage Serializer
    pub fn new() -> PatchedSourceStageRequest {
        PatchedSourceStageRequest {
            name: None,
            flow_set: None,
            source: None,
            resume_timeout: None,
        }
    }
}
