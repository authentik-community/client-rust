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

/// ApplicationEntitlementRequest : ApplicationEntitlement Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationEntitlementRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "app")]
    pub app: uuid::Uuid,
    #[serde(
        rename = "attributes",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<Option<serde_json::Value>>,
}

impl ApplicationEntitlementRequest {
    /// ApplicationEntitlement Serializer
    pub fn new(name: String, app: uuid::Uuid) -> ApplicationEntitlementRequest {
        ApplicationEntitlementRequest {
            name,
            app,
            attributes: None,
        }
    }
}
