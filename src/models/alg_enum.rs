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
pub enum AlgEnum {
    #[serde(rename = "rsa")]
    Rsa,
    #[serde(rename = "ecdsa")]
    Ecdsa,
}

impl std::fmt::Display for AlgEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Rsa => write!(f, "rsa"),
            Self::Ecdsa => write!(f, "ecdsa"),
        }
    }
}

impl Default for AlgEnum {
    fn default() -> AlgEnum {
        Self::Rsa
    }
}
