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

/// PasswordChallenge : Password challenge UI fields
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PasswordChallenge {
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
    #[serde(rename = "recovery_url", skip_serializing_if = "Option::is_none")]
    pub recovery_url: Option<String>,
    #[serde(rename = "allow_show_password", skip_serializing_if = "Option::is_none")]
    pub allow_show_password: Option<bool>,
}

impl PasswordChallenge {
    /// Password challenge UI fields
    pub fn new(pending_user: String, pending_user_avatar: String) -> PasswordChallenge {
        PasswordChallenge {
            flow_info: None,
            component: None,
            response_errors: None,
            pending_user,
            pending_user_avatar,
            recovery_url: None,
            allow_show_password: None,
        }
    }
}
