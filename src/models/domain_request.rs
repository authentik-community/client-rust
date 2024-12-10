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

/// DomainRequest : Domain Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainRequest {
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(rename = "is_primary", skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    #[serde(rename = "tenant")]
    pub tenant: uuid::Uuid,
}

impl DomainRequest {
    /// Domain Serializer
    pub fn new(domain: String, tenant: uuid::Uuid) -> DomainRequest {
        DomainRequest {
            domain,
            is_primary: None,
            tenant,
        }
    }
}
