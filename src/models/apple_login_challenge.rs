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

/// AppleLoginChallenge : Special challenge for apple-native authentication flow, which happens on the client.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppleLoginChallenge {
    #[serde(rename = "flow_info", skip_serializing_if = "Option::is_none")]
    pub flow_info: Option<models::ContextualFlowInfo>,
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "response_errors", skip_serializing_if = "Option::is_none")]
    pub response_errors: Option<std::collections::HashMap<String, Vec<models::ErrorDetail>>>,
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(rename = "redirect_uri")]
    pub redirect_uri: String,
    #[serde(rename = "state")]
    pub state: String,
}

impl AppleLoginChallenge {
    /// Special challenge for apple-native authentication flow, which happens on the client.
    pub fn new(client_id: String, scope: String, redirect_uri: String, state: String) -> AppleLoginChallenge {
        AppleLoginChallenge {
            flow_info: None,
            component: None,
            response_errors: None,
            client_id,
            scope,
            redirect_uri,
            state,
        }
    }
}
