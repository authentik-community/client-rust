/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.6.4
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// AuthenticatedSession : AuthenticatedSession Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatedSession {
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<uuid::Uuid>,
    /// Check if session is currently active session
    #[serde(rename = "current")]
    pub current: bool,
    #[serde(rename = "user_agent")]
    pub user_agent: models::AuthenticatedSessionUserAgent,
    #[serde(rename = "geo_ip", deserialize_with = "Option::deserialize")]
    pub geo_ip: Option<models::AuthenticatedSessionGeoIp>,
    #[serde(rename = "asn", deserialize_with = "Option::deserialize")]
    pub asn: Option<models::AuthenticatedSessionAsn>,
    #[serde(rename = "user")]
    pub user: i32,
    #[serde(rename = "last_ip")]
    pub last_ip: String,
    #[serde(rename = "last_user_agent")]
    pub last_user_agent: String,
    #[serde(rename = "last_used")]
    pub last_used: String,
    #[serde(rename = "expires")]
    pub expires: String,
}

impl AuthenticatedSession {
    /// AuthenticatedSession Serializer
    pub fn new(
        current: bool,
        user_agent: models::AuthenticatedSessionUserAgent,
        geo_ip: Option<models::AuthenticatedSessionGeoIp>,
        asn: Option<models::AuthenticatedSessionAsn>,
        user: i32,
        last_ip: String,
        last_user_agent: String,
        last_used: String,
        expires: String,
    ) -> AuthenticatedSession {
        AuthenticatedSession {
            uuid: None,
            current,
            user_agent,
            geo_ip,
            asn,
            user,
            last_ip,
            last_user_agent,
            last_used,
            expires,
        }
    }
}
