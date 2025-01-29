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
pub enum UserMatchingModeEnum {
    #[serde(rename = "identifier")]
    Identifier,
    #[serde(rename = "email_link")]
    EmailLink,
    #[serde(rename = "email_deny")]
    EmailDeny,
    #[serde(rename = "username_link")]
    UsernameLink,
    #[serde(rename = "username_deny")]
    UsernameDeny,
}

impl std::fmt::Display for UserMatchingModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Identifier => write!(f, "identifier"),
            Self::EmailLink => write!(f, "email_link"),
            Self::EmailDeny => write!(f, "email_deny"),
            Self::UsernameLink => write!(f, "username_link"),
            Self::UsernameDeny => write!(f, "username_deny"),
        }
    }
}

impl Default for UserMatchingModeEnum {
    fn default() -> UserMatchingModeEnum {
        Self::Identifier
    }
}
