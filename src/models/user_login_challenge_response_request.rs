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

/// UserLoginChallengeResponseRequest : User login challenge
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserLoginChallengeResponseRequest {
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "remember_me")]
    pub remember_me: bool,
}

impl UserLoginChallengeResponseRequest {
    /// User login challenge
    pub fn new(remember_me: bool) -> UserLoginChallengeResponseRequest {
        UserLoginChallengeResponseRequest {
            component: None,
            remember_me,
        }
    }
}
