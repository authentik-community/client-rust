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

/// RedirectUri : A single allowed redirect URI entry
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedirectUri {
    #[serde(rename = "matching_mode")]
    pub matching_mode: models::MatchingModeEnum,
    #[serde(rename = "url")]
    pub url: String,
}

impl RedirectUri {
    /// A single allowed redirect URI entry
    pub fn new(matching_mode: models::MatchingModeEnum, url: String) -> RedirectUri {
        RedirectUri { matching_mode, url }
    }
}
