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

/// AuthenticatorSmsChallengeResponseRequest : SMS Challenge response, device is set by get_response_instance
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatorSmsChallengeResponseRequest {
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(rename = "phone_number", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

impl AuthenticatorSmsChallengeResponseRequest {
    /// SMS Challenge response, device is set by get_response_instance
    pub fn new() -> AuthenticatorSmsChallengeResponseRequest {
        AuthenticatorSmsChallengeResponseRequest {
            component: None,
            code: None,
            phone_number: None,
        }
    }
}
