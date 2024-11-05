/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.1
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// EmailChallengeResponseRequest : Email challenge resposen. No fields. This challenge is always declared invalid to give the user a chance to retry
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailChallengeResponseRequest {
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
}

impl EmailChallengeResponseRequest {
    /// Email challenge resposen. No fields. This challenge is always declared invalid to give the user a chance to retry
    pub fn new() -> EmailChallengeResponseRequest {
        EmailChallengeResponseRequest { component: None }
    }
}
