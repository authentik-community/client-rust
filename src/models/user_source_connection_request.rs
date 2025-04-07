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

/// UserSourceConnectionRequest : User source connection
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSourceConnectionRequest {
    #[serde(rename = "user")]
    pub user: i32,
    #[serde(rename = "source")]
    pub source: uuid::Uuid,
    #[serde(rename = "identifier")]
    pub identifier: String,
}

impl UserSourceConnectionRequest {
    /// User source connection
    pub fn new(user: i32, source: uuid::Uuid, identifier: String) -> UserSourceConnectionRequest {
        UserSourceConnectionRequest {
            user,
            source,
            identifier,
        }
    }
}
