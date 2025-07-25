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

/// SessionEndChallenge : Challenge for ending a session
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionEndChallenge {
    #[serde(rename = "flow_info", skip_serializing_if = "Option::is_none")]
    pub flow_info: Option<models::ContextualFlowInfo>,
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "response_errors", skip_serializing_if = "Option::is_none")]
    pub response_errors: Option<std::collections::HashMap<String, Vec<models::ErrorDetail>>>,
    #[serde(rename = "pending_user")]
    pub pending_user: String,
    #[serde(rename = "pending_user_avatar")]
    pub pending_user_avatar: String,
    #[serde(rename = "application_name", skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "application_launch_url", skip_serializing_if = "Option::is_none")]
    pub application_launch_url: Option<String>,
    #[serde(rename = "invalidation_flow_url", skip_serializing_if = "Option::is_none")]
    pub invalidation_flow_url: Option<String>,
    #[serde(rename = "brand_name")]
    pub brand_name: String,
}

impl SessionEndChallenge {
    /// Challenge for ending a session
    pub fn new(pending_user: String, pending_user_avatar: String, brand_name: String) -> SessionEndChallenge {
        SessionEndChallenge {
            flow_info: None,
            component: None,
            response_errors: None,
            pending_user,
            pending_user_avatar,
            application_name: None,
            application_launch_url: None,
            invalidation_flow_url: None,
            brand_name,
        }
    }
}
