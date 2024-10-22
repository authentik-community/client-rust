/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.8.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// FrameChallengeResponseRequest : Base class for all challenge responses
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FrameChallengeResponseRequest {
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
}

impl FrameChallengeResponseRequest {
    /// Base class for all challenge responses
    pub fn new() -> FrameChallengeResponseRequest {
        FrameChallengeResponseRequest { component: None }
    }
}