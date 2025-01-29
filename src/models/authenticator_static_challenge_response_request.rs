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

/// AuthenticatorStaticChallengeResponseRequest : Pseudo class for static response
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatorStaticChallengeResponseRequest {
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
}

impl AuthenticatorStaticChallengeResponseRequest {
    /// Pseudo class for static response
    pub fn new() -> AuthenticatorStaticChallengeResponseRequest {
        AuthenticatorStaticChallengeResponseRequest { component: None }
    }
}
