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

/// UserKerberosSourceConnection : Kerberos Source Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserKerberosSourceConnection {
    #[serde(rename = "pk")]
    pub pk: i32,
    #[serde(rename = "user")]
    pub user: i32,
    #[serde(rename = "source")]
    pub source: models::Source,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "identifier")]
    pub identifier: String,
}

impl UserKerberosSourceConnection {
    /// Kerberos Source Serializer
    pub fn new(
        pk: i32,
        user: i32,
        source: models::Source,
        created: String,
        identifier: String,
    ) -> UserKerberosSourceConnection {
        UserKerberosSourceConnection {
            pk,
            user,
            source,
            created,
            identifier,
        }
    }
}
