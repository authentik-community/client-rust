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

/// AutoSubmitChallengeResponseRequest : Pseudo class for autosubmit response
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoSubmitChallengeResponseRequest {
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
}

impl AutoSubmitChallengeResponseRequest {
    /// Pseudo class for autosubmit response
    pub fn new() -> AutoSubmitChallengeResponseRequest {
        AutoSubmitChallengeResponseRequest { component: None }
    }
}
