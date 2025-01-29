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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NotConfiguredActionEnum {
    #[serde(rename = "skip")]
    Skip,
    #[serde(rename = "deny")]
    Deny,
    #[serde(rename = "configure")]
    Configure,
}

impl std::fmt::Display for NotConfiguredActionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Skip => write!(f, "skip"),
            Self::Deny => write!(f, "deny"),
            Self::Configure => write!(f, "configure"),
        }
    }
}

impl Default for NotConfiguredActionEnum {
    fn default() -> NotConfiguredActionEnum {
        Self::Skip
    }
}
