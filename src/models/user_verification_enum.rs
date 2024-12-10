/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.5
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserVerificationEnum {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "preferred")]
    Preferred,
    #[serde(rename = "discouraged")]
    Discouraged,
}

impl std::fmt::Display for UserVerificationEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Required => write!(f, "required"),
            Self::Preferred => write!(f, "preferred"),
            Self::Discouraged => write!(f, "discouraged"),
        }
    }
}

impl Default for UserVerificationEnum {
    fn default() -> UserVerificationEnum {
        Self::Required
    }
}
