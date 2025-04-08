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

/// UserSamlSourceConnection : User source connection
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSamlSourceConnection {
    #[serde(rename = "pk")]
    pub pk: i32,
    #[serde(rename = "user")]
    pub user: i32,
    #[serde(rename = "source")]
    pub source: uuid::Uuid,
    #[serde(rename = "source_obj")]
    pub source_obj: models::Source,
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "last_updated")]
    pub last_updated: String,
}

impl UserSamlSourceConnection {
    /// User source connection
    pub fn new(
        pk: i32,
        user: i32,
        source: uuid::Uuid,
        source_obj: models::Source,
        identifier: String,
        created: String,
        last_updated: String,
    ) -> UserSamlSourceConnection {
        UserSamlSourceConnection {
            pk,
            user,
            source,
            source_obj,
            identifier,
            created,
            last_updated,
        }
    }
}
