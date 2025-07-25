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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SystemTaskStatusEnum {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "successful")]
    Successful,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
}

impl std::fmt::Display for SystemTaskStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::Successful => write!(f, "successful"),
            Self::Warning => write!(f, "warning"),
            Self::Error => write!(f, "error"),
        }
    }
}

impl Default for SystemTaskStatusEnum {
    fn default() -> SystemTaskStatusEnum {
        Self::Unknown
    }
}
