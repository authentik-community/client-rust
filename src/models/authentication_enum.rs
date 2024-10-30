/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.0
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthenticationEnum {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "require_authenticated")]
    RequireAuthenticated,
    #[serde(rename = "require_unauthenticated")]
    RequireUnauthenticated,
    #[serde(rename = "require_superuser")]
    RequireSuperuser,
    #[serde(rename = "require_outpost")]
    RequireOutpost,
}

impl std::fmt::Display for AuthenticationEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::RequireAuthenticated => write!(f, "require_authenticated"),
            Self::RequireUnauthenticated => write!(f, "require_unauthenticated"),
            Self::RequireSuperuser => write!(f, "require_superuser"),
            Self::RequireOutpost => write!(f, "require_outpost"),
        }
    }
}

impl Default for AuthenticationEnum {
    fn default() -> AuthenticationEnum {
        Self::None
    }
}
