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

/// ConnectionToken : ConnectionToken Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionToken {
    #[serde(rename = "pk", skip_serializing_if = "Option::is_none")]
    pub pk: Option<uuid::Uuid>,
    #[serde(rename = "provider")]
    pub provider: i32,
    #[serde(rename = "provider_obj")]
    pub provider_obj: models::RacProvider,
    #[serde(rename = "endpoint")]
    pub endpoint: uuid::Uuid,
    #[serde(rename = "endpoint_obj")]
    pub endpoint_obj: models::Endpoint,
    #[serde(rename = "user")]
    pub user: models::GroupMember,
}

impl ConnectionToken {
    /// ConnectionToken Serializer
    pub fn new(
        provider: i32,
        provider_obj: models::RacProvider,
        endpoint: uuid::Uuid,
        endpoint_obj: models::Endpoint,
        user: models::GroupMember,
    ) -> ConnectionToken {
        ConnectionToken {
            pk: None,
            provider,
            provider_obj,
            endpoint,
            endpoint_obj,
            user,
        }
    }
}
