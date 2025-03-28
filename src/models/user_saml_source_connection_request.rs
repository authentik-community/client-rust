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

/// UserSamlSourceConnectionRequest : SAML Source Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSamlSourceConnectionRequest {
    #[serde(rename = "user")]
    pub user: i32,
    #[serde(rename = "source")]
    pub source: uuid::Uuid,
    #[serde(rename = "identifier")]
    pub identifier: String,
}

impl UserSamlSourceConnectionRequest {
    /// SAML Source Serializer
    pub fn new(user: i32, source: uuid::Uuid, identifier: String) -> UserSamlSourceConnectionRequest {
        UserSamlSourceConnectionRequest {
            user,
            source,
            identifier,
        }
    }
}
