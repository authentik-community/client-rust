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

/// PatchedScimProviderRequest : SCIMProvider Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedScimProviderRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "property_mappings", skip_serializing_if = "Option::is_none")]
    pub property_mappings: Option<Vec<uuid::Uuid>>,
    /// Property mappings used for group creation/updating.
    #[serde(rename = "property_mappings_group", skip_serializing_if = "Option::is_none")]
    pub property_mappings_group: Option<Vec<uuid::Uuid>>,
    /// Base URL to SCIM requests, usually ends in /v2
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "verify_certificates", skip_serializing_if = "Option::is_none")]
    pub verify_certificates: Option<bool>,
    /// Authentication token
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Alter authentik behavior for vendor-specific SCIM implementations.
    #[serde(rename = "compatibility_mode", skip_serializing_if = "Option::is_none")]
    pub compatibility_mode: Option<models::CompatibilityModeEnum>,
    #[serde(rename = "exclude_users_service_account", skip_serializing_if = "Option::is_none")]
    pub exclude_users_service_account: Option<bool>,
    #[serde(
        rename = "filter_group",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub filter_group: Option<Option<uuid::Uuid>>,
    /// When enabled, provider will not modify or create objects in the remote system.
    #[serde(rename = "dry_run", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl PatchedScimProviderRequest {
    /// SCIMProvider Serializer
    pub fn new() -> PatchedScimProviderRequest {
        PatchedScimProviderRequest {
            name: None,
            property_mappings: None,
            property_mappings_group: None,
            url: None,
            verify_certificates: None,
            token: None,
            compatibility_mode: None,
            exclude_users_service_account: None,
            filter_group: None,
            dry_run: None,
        }
    }
}
