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
pub enum DeniedActionEnum {
    #[serde(rename = "message_continue")]
    MessageContinue,
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "continue")]
    Continue,
}

impl std::fmt::Display for DeniedActionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::MessageContinue => write!(f, "message_continue"),
            Self::Message => write!(f, "message"),
            Self::Continue => write!(f, "continue"),
        }
    }
}

impl Default for DeniedActionEnum {
    fn default() -> DeniedActionEnum {
        Self::MessageContinue
    }
}
