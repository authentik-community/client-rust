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

/// GoogleWorkspaceProviderRequest : GoogleWorkspaceProvider Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GoogleWorkspaceProviderRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "property_mappings", skip_serializing_if = "Option::is_none")]
    pub property_mappings: Option<Vec<uuid::Uuid>>,
    /// Property mappings used for group creation/updating.
    #[serde(rename = "property_mappings_group", skip_serializing_if = "Option::is_none")]
    pub property_mappings_group: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "delegated_subject")]
    pub delegated_subject: String,
    #[serde(rename = "credentials", deserialize_with = "Option::deserialize")]
    pub credentials: Option<serde_json::Value>,
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
    #[serde(rename = "default_group_email_domain")]
    pub default_group_email_domain: String,
}

impl GoogleWorkspaceProviderRequest {
    /// GoogleWorkspaceProvider Serializer
    pub fn new(
        name: String,
        delegated_subject: String,
        credentials: Option<serde_json::Value>,
        default_group_email_domain: String,
    ) -> GoogleWorkspaceProviderRequest {
        GoogleWorkspaceProviderRequest {
            name,
            property_mappings: None,
            property_mappings_group: None,
            delegated_subject,
            credentials,
            scopes: None,
            exclude_users_service_account: None,
            filter_group: None,
            user_delete_action: None,
            group_delete_action: None,
            default_group_email_domain,
        }
    }
}
