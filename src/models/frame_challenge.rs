/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.8.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// FrameChallenge : Challenge type to render a frame
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FrameChallenge {
    #[serde(rename = "flow_info", skip_serializing_if = "Option::is_none")]
    pub flow_info: Option<models::ContextualFlowInfo>,
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "response_errors", skip_serializing_if = "Option::is_none")]
    pub response_errors: Option<std::collections::HashMap<String, Vec<models::ErrorDetail>>>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "loading_overlay", skip_serializing_if = "Option::is_none")]
    pub loading_overlay: Option<bool>,
    #[serde(rename = "loading_text")]
    pub loading_text: String,
}

impl FrameChallenge {
    /// Challenge type to render a frame
    pub fn new(url: String, loading_text: String) -> FrameChallenge {
        FrameChallenge {
            flow_info: None,
            component: None,
            response_errors: None,
            url,
            loading_overlay: None,
            loading_text,
        }
    }
}