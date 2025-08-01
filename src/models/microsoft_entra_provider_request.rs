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

/// MicrosoftEntraProviderRequest : MicrosoftEntraProvider Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftEntraProviderRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "property_mappings", skip_serializing_if = "Option::is_none")]
    pub property_mappings: Option<Vec<uuid::Uuid>>,
    /// Property mappings used for group creation/updating.
    #[serde(rename = "property_mappings_group", skip_serializing_if = "Option::is_none")]
    pub property_mappings_group: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "client_secret")]
    pub client_secret: String,
    #[serde(rename = "tenant_id")]
    pub tenant_id: String,
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

impl MicrosoftEntraProviderRequest {
    /// MicrosoftEntraProvider Serializer
    pub fn new(
        name: String,
        client_id: String,
        client_secret: String,
        tenant_id: String,
    ) -> MicrosoftEntraProviderRequest {
        MicrosoftEntraProviderRequest {
            name,
            property_mappings: None,
            property_mappings_group: None,
            client_id,
            client_secret,
            tenant_id,
            exclude_users_service_account: None,
            filter_group: None,
            user_delete_action: None,
            group_delete_action: None,
            dry_run: None,
        }
    }
}
