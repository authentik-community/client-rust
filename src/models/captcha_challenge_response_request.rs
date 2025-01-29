/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CaptchaChallengeResponseRequest : Validate captcha token
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CaptchaChallengeResponseRequest {
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "token")]
    pub token: String,
}

impl CaptchaChallengeResponseRequest {
    /// Validate captcha token
    pub fn new(token: String) -> CaptchaChallengeResponseRequest {
        CaptchaChallengeResponseRequest { component: None, token }
    }
}
