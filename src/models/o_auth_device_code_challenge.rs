/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.6.4
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// OAuthDeviceCodeChallenge : OAuth Device code challenge
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuthDeviceCodeChallenge {
    #[serde(rename = "flow_info", skip_serializing_if = "Option::is_none")]
    pub flow_info: Option<models::ContextualFlowInfo>,
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "response_errors", skip_serializing_if = "Option::is_none")]
    pub response_errors: Option<std::collections::HashMap<String, Vec<models::ErrorDetail>>>,
}

impl OAuthDeviceCodeChallenge {
    /// OAuth Device code challenge
    pub fn new() -> OAuthDeviceCodeChallenge {
        OAuthDeviceCodeChallenge {
            flow_info: None,
            component: None,
            response_errors: None,
        }
    }
}
