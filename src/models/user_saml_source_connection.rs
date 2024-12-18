/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.5
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UserSamlSourceConnection : SAML Source Serializer
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
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "identifier")]
    pub identifier: String,
}

impl UserSamlSourceConnection {
    /// SAML Source Serializer
    pub fn new(
        pk: i32,
        user: i32,
        source: uuid::Uuid,
        source_obj: models::Source,
        created: String,
        identifier: String,
    ) -> UserSamlSourceConnection {
        UserSamlSourceConnection {
            pk,
            user,
            source,
            source_obj,
            created,
            identifier,
        }
    }
}
