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

/// PatchedSettingsRequest : Settings Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedSettingsRequest {
    /// Configure how authentik should show avatars for users.
    #[serde(rename = "avatars", skip_serializing_if = "Option::is_none")]
    pub avatars: Option<String>,
    /// Enable the ability for users to change their name.
    #[serde(rename = "default_user_change_name", skip_serializing_if = "Option::is_none")]
    pub default_user_change_name: Option<bool>,
    /// Enable the ability for users to change their email address.
    #[serde(rename = "default_user_change_email", skip_serializing_if = "Option::is_none")]
    pub default_user_change_email: Option<bool>,
    /// Enable the ability for users to change their username.
    #[serde(rename = "default_user_change_username", skip_serializing_if = "Option::is_none")]
    pub default_user_change_username: Option<bool>,
    /// Events will be deleted after this duration.(Format: weeks=3;days=2;hours=3,seconds=2).
    #[serde(rename = "event_retention", skip_serializing_if = "Option::is_none")]
    pub event_retention: Option<String>,
    /// The option configures the footer links on the flow executor pages.
    #[serde(
        rename = "footer_links",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub footer_links: Option<Option<serde_json::Value>>,
    /// When enabled, all the events caused by a user will be deleted upon the user's deletion.
    #[serde(rename = "gdpr_compliance", skip_serializing_if = "Option::is_none")]
    pub gdpr_compliance: Option<bool>,
    /// Globally enable/disable impersonation.
    #[serde(rename = "impersonation", skip_serializing_if = "Option::is_none")]
    pub impersonation: Option<bool>,
    /// Default token duration
    #[serde(rename = "default_token_duration", skip_serializing_if = "Option::is_none")]
    pub default_token_duration: Option<String>,
    /// Default token length
    #[serde(rename = "default_token_length", skip_serializing_if = "Option::is_none")]
    pub default_token_length: Option<u32>,
}

impl PatchedSettingsRequest {
    /// Settings Serializer
    pub fn new() -> PatchedSettingsRequest {
        PatchedSettingsRequest {
            avatars: None,
            default_user_change_name: None,
            default_user_change_email: None,
            default_user_change_username: None,
            event_retention: None,
            footer_links: None,
            gdpr_compliance: None,
            impersonation: None,
            default_token_duration: None,
            default_token_length: None,
        }
    }
}
