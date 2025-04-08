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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConsentStageModeEnum {
    #[serde(rename = "always_require")]
    AlwaysRequire,
    #[serde(rename = "permanent")]
    Permanent,
    #[serde(rename = "expiring")]
    Expiring,
}

impl std::fmt::Display for ConsentStageModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AlwaysRequire => write!(f, "always_require"),
            Self::Permanent => write!(f, "permanent"),
            Self::Expiring => write!(f, "expiring"),
        }
    }
}

impl Default for ConsentStageModeEnum {
    fn default() -> ConsentStageModeEnum {
        Self::AlwaysRequire
    }
}
