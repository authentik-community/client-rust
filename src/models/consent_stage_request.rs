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

/// ConsentStageRequest : ConsentStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsentStageRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<models::ConsentStageModeEnum>,
    /// Offset after which consent expires. (Format: hours=1;minutes=2;seconds=3).
    #[serde(rename = "consent_expire_in", skip_serializing_if = "Option::is_none")]
    pub consent_expire_in: Option<String>,
}

impl ConsentStageRequest {
    /// ConsentStage Serializer
    pub fn new(name: String) -> ConsentStageRequest {
        ConsentStageRequest {
            name,
            flow_set: None,
            mode: None,
            consent_expire_in: None,
        }
    }
}
