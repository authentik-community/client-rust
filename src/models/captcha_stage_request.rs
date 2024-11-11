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

/// CaptchaStageRequest : CaptchaStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CaptchaStageRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
    /// Public key, acquired your captcha Provider.
    #[serde(rename = "public_key")]
    pub public_key: String,
    /// Private key, acquired your captcha Provider.
    #[serde(rename = "private_key")]
    pub private_key: String,
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

impl CaptchaStageRequest {
    /// CaptchaStage Serializer
    pub fn new(name: String, public_key: String, private_key: String) -> CaptchaStageRequest {
        CaptchaStageRequest {
            name,
            flow_set: None,
            public_key,
            private_key,
            js_url: None,
            api_url: None,
            interactive: None,
            score_min_threshold: None,
            score_max_threshold: None,
            error_on_invalid_score: None,
        }
    }
}
