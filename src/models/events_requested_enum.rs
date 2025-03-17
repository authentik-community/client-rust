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
pub enum EventsRequestedEnum {
    #[serde(rename = "https://schemas.openid.net/secevent/caep/event-type/session-revoked")]
    CaepSlashEventTypeSlashSessionRevoked,
    #[serde(rename = "https://schemas.openid.net/secevent/caep/event-type/credential-change")]
    CaepSlashEventTypeSlashCredentialChange,
    #[serde(rename = "https://schemas.openid.net/secevent/ssf/event-type/verification")]
    SsfSlashEventTypeSlashVerification,
}

impl std::fmt::Display for EventsRequestedEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CaepSlashEventTypeSlashSessionRevoked => {
                write!(f, "https://schemas.openid.net/secevent/caep/event-type/session-revoked")
            }
            Self::CaepSlashEventTypeSlashCredentialChange => write!(
                f,
                "https://schemas.openid.net/secevent/caep/event-type/credential-change"
            ),
            Self::SsfSlashEventTypeSlashVerification => {
                write!(f, "https://schemas.openid.net/secevent/ssf/event-type/verification")
            }
        }
    }
}

impl Default for EventsRequestedEnum {
    fn default() -> EventsRequestedEnum {
        Self::CaepSlashEventTypeSlashSessionRevoked
    }
}
