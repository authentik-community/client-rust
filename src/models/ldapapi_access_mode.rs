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
pub enum LdapapiAccessMode {
    #[serde(rename = "direct")]
    Direct,
    #[serde(rename = "cached")]
    Cached,
}

impl std::fmt::Display for LdapapiAccessMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Direct => write!(f, "direct"),
            Self::Cached => write!(f, "cached"),
        }
    }
}

impl Default for LdapapiAccessMode {
    fn default() -> LdapapiAccessMode {
        Self::Direct
    }
}
