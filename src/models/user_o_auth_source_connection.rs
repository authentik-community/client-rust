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

/// UserOAuthSourceConnection : User source connection
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserOAuthSourceConnection {
    #[serde(rename = "pk")]
    pub pk: i32,
    #[serde(rename = "user")]
    pub user: i32,
    #[serde(rename = "source")]
    pub source: uuid::Uuid,
    #[serde(rename = "source_obj")]
    pub source_obj: models::Source,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "identifier")]
    pub identifier: String,
}

impl UserOAuthSourceConnection {
    /// User source connection
    pub fn new(
        pk: i32,
        user: i32,
        source: uuid::Uuid,
        source_obj: models::Source,
        created: String,
        identifier: String,
    ) -> UserOAuthSourceConnection {
        UserOAuthSourceConnection {
            pk,
            user,
            source,
            source_obj,
            created,
            identifier,
        }
    }
}
