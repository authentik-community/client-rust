/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.1
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ScimProviderRequest : SCIMProvider Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScimProviderRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "property_mappings", skip_serializing_if = "Option::is_none")]
    pub property_mappings: Option<Vec<uuid::Uuid>>,
    /// Property mappings used for group creation/updating.
    #[serde(rename = "property_mappings_group", skip_serializing_if = "Option::is_none")]
    pub property_mappings_group: Option<Vec<uuid::Uuid>>,
    /// Base URL to SCIM requests, usually ends in /v2
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "verify_certificates", skip_serializing_if = "Option::is_none")]
    pub verify_certificates: Option<bool>,
    /// Authentication token
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "exclude_users_service_account", skip_serializing_if = "Option::is_none")]
    pub exclude_users_service_account: Option<bool>,
    #[serde(
        rename = "filter_group",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub filter_group: Option<Option<uuid::Uuid>>,
}

impl ScimProviderRequest {
    /// SCIMProvider Serializer
    pub fn new(name: String, url: String, token: String) -> ScimProviderRequest {
        ScimProviderRequest {
            name,
            property_mappings: None,
            property_mappings_group: None,
            url,
            verify_certificates: None,
            token,
            exclude_users_service_account: None,
            filter_group: None,
        }
    }
}
