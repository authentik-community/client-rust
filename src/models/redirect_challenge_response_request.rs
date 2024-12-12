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

/// RedirectChallengeResponseRequest : Redirect challenge response
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedirectChallengeResponseRequest {
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "to")]
    pub to: String,
}

impl RedirectChallengeResponseRequest {
    /// Redirect challenge response
    pub fn new(to: String) -> RedirectChallengeResponseRequest {
        RedirectChallengeResponseRequest { component: None, to }
    }
}
