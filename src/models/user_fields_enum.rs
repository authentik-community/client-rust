/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserFieldsEnum {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "username")]
    Username,
    #[serde(rename = "upn")]
    Upn,
}

impl std::fmt::Display for UserFieldsEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Email => write!(f, "email"),
            Self::Username => write!(f, "username"),
            Self::Upn => write!(f, "upn"),
        }
    }
}

impl Default for UserFieldsEnum {
    fn default() -> UserFieldsEnum {
        Self::Email
    }
}
