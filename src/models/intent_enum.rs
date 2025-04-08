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
pub enum IntentEnum {
    #[serde(rename = "verification")]
    Verification,
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "recovery")]
    Recovery,
    #[serde(rename = "app_password")]
    AppPassword,
}

impl std::fmt::Display for IntentEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Verification => write!(f, "verification"),
            Self::Api => write!(f, "api"),
            Self::Recovery => write!(f, "recovery"),
            Self::AppPassword => write!(f, "app_password"),
        }
    }
}

impl Default for IntentEnum {
    fn default() -> IntentEnum {
        Self::Verification
    }
}
