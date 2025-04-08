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

/// OAuthSourceRequest : OAuth Source Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuthSourceRequest {
    /// Source's display Name.
    #[serde(rename = "name")]
    pub name: String,
    /// Internal source name, used in URLs.
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Flow to use when authenticating existing users.
    #[serde(
        rename = "authentication_flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub authentication_flow: Option<Option<uuid::Uuid>>,
    /// Flow to use when enrolling new users.
    #[serde(
        rename = "enrollment_flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub enrollment_flow: Option<Option<uuid::Uuid>>,
    #[serde(rename = "user_property_mappings", skip_serializing_if = "Option::is_none")]
    pub user_property_mappings: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "group_property_mappings", skip_serializing_if = "Option::is_none")]
    pub group_property_mappings: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "policy_engine_mode", skip_serializing_if = "Option::is_none")]
    pub policy_engine_mode: Option<models::PolicyEngineMode>,
    /// How the source determines if an existing user should be authenticated or a new user enrolled.
    #[serde(rename = "user_matching_mode", skip_serializing_if = "Option::is_none")]
    pub user_matching_mode: Option<models::UserMatchingModeEnum>,
    #[serde(rename = "user_path_template", skip_serializing_if = "Option::is_none")]
    pub user_path_template: Option<String>,
    /// How the source determines if an existing group should be used or a new group created.
    #[serde(rename = "group_matching_mode", skip_serializing_if = "Option::is_none")]
    pub group_matching_mode: Option<models::GroupMatchingModeEnum>,
    #[serde(rename = "provider_type")]
    pub provider_type: models::ProviderTypeEnum,
    /// URL used to request the initial token. This URL is only required for OAuth 1.
    #[serde(
        rename = "request_token_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub request_token_url: Option<Option<String>>,
    /// URL the user is redirect to to conest the flow.
    #[serde(
        rename = "authorization_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_url: Option<Option<String>>,
    /// URL used by authentik to retrieve tokens.
    #[serde(
        rename = "access_token_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub access_token_url: Option<Option<String>>,
    /// URL used by authentik to get user information.
    #[serde(
        rename = "profile_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub profile_url: Option<Option<String>>,
    #[serde(rename = "consumer_key")]
    pub consumer_key: String,
    #[serde(rename = "consumer_secret")]
    pub consumer_secret: String,
    #[serde(rename = "additional_scopes", skip_serializing_if = "Option::is_none")]
    pub additional_scopes: Option<String>,
    #[serde(rename = "oidc_well_known_url", skip_serializing_if = "Option::is_none")]
    pub oidc_well_known_url: Option<String>,
    #[serde(rename = "oidc_jwks_url", skip_serializing_if = "Option::is_none")]
    pub oidc_jwks_url: Option<String>,
    #[serde(
        rename = "oidc_jwks",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub oidc_jwks: Option<Option<serde_json::Value>>,
}

impl OAuthSourceRequest {
    /// OAuth Source Serializer
    pub fn new(
        name: String,
        slug: String,
        provider_type: models::ProviderTypeEnum,
        consumer_key: String,
        consumer_secret: String,
    ) -> OAuthSourceRequest {
        OAuthSourceRequest {
            name,
            slug,
            enabled: None,
            authentication_flow: None,
            enrollment_flow: None,
            user_property_mappings: None,
            group_property_mappings: None,
            policy_engine_mode: None,
            user_matching_mode: None,
            user_path_template: None,
            group_matching_mode: None,
            provider_type,
            request_token_url: None,
            authorization_url: None,
            access_token_url: None,
            profile_url: None,
            consumer_key,
            consumer_secret,
            additional_scopes: None,
            oidc_well_known_url: None,
            oidc_jwks_url: None,
            oidc_jwks: None,
        }
    }
}
