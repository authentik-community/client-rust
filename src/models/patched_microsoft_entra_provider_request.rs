/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.6.4
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PatchedMicrosoftEntraProviderRequest : MicrosoftEntraProvider Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedMicrosoftEntraProviderRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "property_mappings", skip_serializing_if = "Option::is_none")]
    pub property_mappings: Option<Vec<uuid::Uuid>>,
    /// Property mappings used for group creation/updating.
    #[serde(rename = "property_mappings_group", skip_serializing_if = "Option::is_none")]
    pub property_mappings_group: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "client_secret", skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(rename = "tenant_id", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "exclude_users_service_account", skip_serializing_if = "Option::is_none")]
    pub exclude_users_service_account: Option<bool>,
    #[serde(
        rename = "filter_group",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub filter_group: Option<Option<uuid::Uuid>>,
    #[serde(rename = "user_delete_action", skip_serializing_if = "Option::is_none")]
    pub user_delete_action: Option<models::OutgoingSyncDeleteAction>,
    #[serde(rename = "group_delete_action", skip_serializing_if = "Option::is_none")]
    pub group_delete_action: Option<models::OutgoingSyncDeleteAction>,
    /// When enabled, provider will not modify or create objects in the remote system.
    #[serde(rename = "dry_run", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl PatchedMicrosoftEntraProviderRequest {
    /// MicrosoftEntraProvider Serializer
    pub fn new() -> PatchedMicrosoftEntraProviderRequest {
        PatchedMicrosoftEntraProviderRequest {
            name: None,
            property_mappings: None,
            property_mappings_group: None,
            client_id: None,
            client_secret: None,
            tenant_id: None,
            exclude_users_service_account: None,
            filter_group: None,
            user_delete_action: None,
            group_delete_action: None,
            dry_run: None,
        }
    }
}
