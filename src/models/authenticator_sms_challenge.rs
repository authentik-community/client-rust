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

/// AuthenticatorSmsChallenge : SMS Setup challenge
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatorSmsChallenge {
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
    #[serde(rename = "phone_number_required", skip_serializing_if = "Option::is_none")]
    pub phone_number_required: Option<bool>,
}

impl AuthenticatorSmsChallenge {
    /// SMS Setup challenge
    pub fn new(pending_user: String, pending_user_avatar: String) -> AuthenticatorSmsChallenge {
        AuthenticatorSmsChallenge {
            flow_info: None,
            component: None,
            response_errors: None,
            pending_user,
            pending_user_avatar,
            phone_number_required: None,
        }
    }
}
