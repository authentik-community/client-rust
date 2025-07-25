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

/// AuthenticatedSessionUserAgentOs : User agent os
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatedSessionUserAgentOs {
    #[serde(rename = "family")]
    pub family: String,
    #[serde(rename = "major")]
    pub major: String,
    #[serde(rename = "minor")]
    pub minor: String,
    #[serde(rename = "patch")]
    pub patch: String,
    #[serde(rename = "patch_minor")]
    pub patch_minor: String,
}

impl AuthenticatedSessionUserAgentOs {
    /// User agent os
    pub fn new(
        family: String,
        major: String,
        minor: String,
        patch: String,
        patch_minor: String,
    ) -> AuthenticatedSessionUserAgentOs {
        AuthenticatedSessionUserAgentOs {
            family,
            major,
            minor,
            patch,
            patch_minor,
        }
    }
}
