/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.2
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DuoResponseEnum {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "waiting")]
    Waiting,
    #[serde(rename = "invalid")]
    Invalid,
}

impl std::fmt::Display for DuoResponseEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "success"),
            Self::Waiting => write!(f, "waiting"),
            Self::Invalid => write!(f, "invalid"),
        }
    }
}

impl Default for DuoResponseEnum {
    fn default() -> DuoResponseEnum {
        Self::Success
    }
}
