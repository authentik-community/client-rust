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

/// OAuthDeviceCodeFinishChallengeResponseRequest : Response that device has been authenticated and tab can be closed
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuthDeviceCodeFinishChallengeResponseRequest {
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
}

impl OAuthDeviceCodeFinishChallengeResponseRequest {
    /// Response that device has been authenticated and tab can be closed
    pub fn new() -> OAuthDeviceCodeFinishChallengeResponseRequest {
        OAuthDeviceCodeFinishChallengeResponseRequest { component: None }
    }
}
