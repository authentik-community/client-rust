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

/// UserOAuthSourceConnectionRequest : OAuth Source Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserOAuthSourceConnectionRequest {
    #[serde(rename = "user")]
    pub user: i32,
    #[serde(rename = "source")]
    pub source: uuid::Uuid,
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(
        rename = "access_token",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub access_token: Option<Option<String>>,
}

impl UserOAuthSourceConnectionRequest {
    /// OAuth Source Serializer
    pub fn new(user: i32, source: uuid::Uuid, identifier: String) -> UserOAuthSourceConnectionRequest {
        UserOAuthSourceConnectionRequest {
            user,
            source,
            identifier,
            access_token: None,
        }
    }
}
