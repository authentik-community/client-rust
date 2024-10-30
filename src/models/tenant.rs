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

/// Tenant : Tenant Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tenant {
    #[serde(rename = "tenant_uuid")]
    pub tenant_uuid: uuid::Uuid,
    #[serde(rename = "schema_name")]
    pub schema_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "ready", skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
}

impl Tenant {
    /// Tenant Serializer
    pub fn new(tenant_uuid: uuid::Uuid, schema_name: String, name: String) -> Tenant {
        Tenant {
            tenant_uuid,
            schema_name,
            name,
            ready: None,
        }
    }
}
