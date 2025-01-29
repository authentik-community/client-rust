/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PatchedGoogleWorkspaceProviderRequest : GoogleWorkspaceProvider Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedGoogleWorkspaceProviderRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "property_mappings", skip_serializing_if = "Option::is_none")]
    pub property_mappings: Option<Vec<uuid::Uuid>>,
    /// Property mappings used for group creation/updating.
    #[serde(rename = "property_mappings_group", skip_serializing_if = "Option::is_none")]
    pub property_mappings_group: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "delegated_subject", skip_serializing_if = "Option::is_none")]
    pub delegated_subject: Option<String>,
    #[serde(
        rename = "credentials",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub credentials: Option<Option<serde_json::Value>>,
    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Option<String>,
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
    #[serde(rename = "default_group_email_domain", skip_serializing_if = "Option::is_none")]
    pub default_group_email_domain: Option<String>,
}

impl PatchedGoogleWorkspaceProviderRequest {
    /// GoogleWorkspaceProvider Serializer
    pub fn new() -> PatchedGoogleWorkspaceProviderRequest {
        PatchedGoogleWorkspaceProviderRequest {
            name: None,
            property_mappings: None,
            property_mappings_group: None,
            delegated_subject: None,
            credentials: None,
            scopes: None,
            exclude_users_service_account: None,
            filter_group: None,
            user_delete_action: None,
            group_delete_action: None,
            default_group_email_domain: None,
        }
    }
}
