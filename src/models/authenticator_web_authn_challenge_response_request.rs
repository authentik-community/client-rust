/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.4
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// AuthenticatorWebAuthnChallengeResponseRequest : WebAuthn Challenge response
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatorWebAuthnChallengeResponseRequest {
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "response")]
    pub response: std::collections::HashMap<String, serde_json::Value>,
}

impl AuthenticatorWebAuthnChallengeResponseRequest {
    /// WebAuthn Challenge response
    pub fn new(
        response: std::collections::HashMap<String, serde_json::Value>,
    ) -> AuthenticatorWebAuthnChallengeResponseRequest {
        AuthenticatorWebAuthnChallengeResponseRequest {
            component: None,
            response,
        }
    }
}
