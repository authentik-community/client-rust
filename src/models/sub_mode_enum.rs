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
pub enum SubModeEnum {
    #[serde(rename = "hashed_user_id")]
    HashedUserId,
    #[serde(rename = "user_id")]
    UserId,
    #[serde(rename = "user_uuid")]
    UserUuid,
    #[serde(rename = "user_username")]
    UserUsername,
    #[serde(rename = "user_email")]
    UserEmail,
    #[serde(rename = "user_upn")]
    UserUpn,
}

impl std::fmt::Display for SubModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::HashedUserId => write!(f, "hashed_user_id"),
            Self::UserId => write!(f, "user_id"),
            Self::UserUuid => write!(f, "user_uuid"),
            Self::UserUsername => write!(f, "user_username"),
            Self::UserEmail => write!(f, "user_email"),
            Self::UserUpn => write!(f, "user_upn"),
        }
    }
}

impl Default for SubModeEnum {
    fn default() -> SubModeEnum {
        Self::HashedUserId
    }
}
