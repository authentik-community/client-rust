/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.2
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PatchedCaptchaStageRequest : CaptchaStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedCaptchaStageRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
    /// Public key, acquired your captcha Provider.
    #[serde(rename = "public_key", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// Private key, acquired your captcha Provider.
    #[serde(rename = "private_key", skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(rename = "js_url", skip_serializing_if = "Option::is_none")]
    pub js_url: Option<String>,
    #[serde(rename = "api_url", skip_serializing_if = "Option::is_none")]
    pub api_url: Option<String>,
    #[serde(rename = "interactive", skip_serializing_if = "Option::is_none")]
    pub interactive: Option<bool>,
    #[serde(rename = "score_min_threshold", skip_serializing_if = "Option::is_none")]
    pub score_min_threshold: Option<f64>,
    #[serde(rename = "score_max_threshold", skip_serializing_if = "Option::is_none")]
    pub score_max_threshold: Option<f64>,
    /// When enabled and the received captcha score is outside of the given threshold, the stage will show an error message. When not enabled, the flow will continue, but the data from the captcha will be available in the context for policy decisions
    #[serde(rename = "error_on_invalid_score", skip_serializing_if = "Option::is_none")]
    pub error_on_invalid_score: Option<bool>,
}

impl PatchedCaptchaStageRequest {
    /// CaptchaStage Serializer
    pub fn new() -> PatchedCaptchaStageRequest {
        PatchedCaptchaStageRequest {
            name: None,
            flow_set: None,
            public_key: None,
            private_key: None,
            js_url: None,
            api_url: None,
            interactive: None,
            score_min_threshold: None,
            score_max_threshold: None,
            error_on_invalid_score: None,
        }
    }
}
