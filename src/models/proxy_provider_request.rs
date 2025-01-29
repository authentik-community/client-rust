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

/// ProxyProviderRequest : ProxyProvider Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyProviderRequest {
    #[serde(rename = "name")]
    pub name: String,
    /// Flow used for authentication when the associated application is accessed by an un-authenticated user.
    #[serde(
        rename = "authentication_flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub authentication_flow: Option<Option<uuid::Uuid>>,
    /// Flow used when authorizing this provider.
    #[serde(rename = "authorization_flow")]
    pub authorization_flow: uuid::Uuid,
    /// Flow used ending the session from a provider.
    #[serde(rename = "invalidation_flow")]
    pub invalidation_flow: uuid::Uuid,
    #[serde(rename = "property_mappings", skip_serializing_if = "Option::is_none")]
    pub property_mappings: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "internal_host", skip_serializing_if = "Option::is_none")]
    pub internal_host: Option<String>,
    #[serde(rename = "external_host")]
    pub external_host: String,
    /// Validate SSL Certificates of upstream servers
    #[serde(rename = "internal_host_ssl_validation", skip_serializing_if = "Option::is_none")]
    pub internal_host_ssl_validation: Option<bool>,
    #[serde(
        rename = "certificate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub certificate: Option<Option<uuid::Uuid>>,
    /// Regular expressions for which authentication is not required. Each new line is interpreted as a new Regular Expression.
    #[serde(rename = "skip_path_regex", skip_serializing_if = "Option::is_none")]
    pub skip_path_regex: Option<String>,
    /// Set a custom HTTP-Basic Authentication header based on values from authentik.
    #[serde(rename = "basic_auth_enabled", skip_serializing_if = "Option::is_none")]
    pub basic_auth_enabled: Option<bool>,
    /// User/Group Attribute used for the password part of the HTTP-Basic Header.
    #[serde(rename = "basic_auth_password_attribute", skip_serializing_if = "Option::is_none")]
    pub basic_auth_password_attribute: Option<String>,
    /// User/Group Attribute used for the user part of the HTTP-Basic Header. If not set, the user's Email address is used.
    #[serde(rename = "basic_auth_user_attribute", skip_serializing_if = "Option::is_none")]
    pub basic_auth_user_attribute: Option<String>,
    /// Enable support for forwardAuth in traefik and nginx auth_request. Exclusive with internal_host.
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<models::ProxyMode>,
    /// When enabled, this provider will intercept the authorization header and authenticate requests based on its value.
    #[serde(rename = "intercept_header_auth", skip_serializing_if = "Option::is_none")]
    pub intercept_header_auth: Option<bool>,
    #[serde(rename = "cookie_domain", skip_serializing_if = "Option::is_none")]
    pub cookie_domain: Option<String>,
    #[serde(rename = "jwt_federation_sources", skip_serializing_if = "Option::is_none")]
    pub jwt_federation_sources: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "jwt_federation_providers", skip_serializing_if = "Option::is_none")]
    pub jwt_federation_providers: Option<Vec<i32>>,
    /// Tokens not valid on or after current time + this value (Format: hours=1;minutes=2;seconds=3).
    #[serde(rename = "access_token_validity", skip_serializing_if = "Option::is_none")]
    pub access_token_validity: Option<String>,
    /// Tokens not valid on or after current time + this value (Format: hours=1;minutes=2;seconds=3).
    #[serde(rename = "refresh_token_validity", skip_serializing_if = "Option::is_none")]
    pub refresh_token_validity: Option<String>,
}

impl ProxyProviderRequest {
    /// ProxyProvider Serializer
    pub fn new(
        name: String,
        authorization_flow: uuid::Uuid,
        invalidation_flow: uuid::Uuid,
        external_host: String,
    ) -> ProxyProviderRequest {
        ProxyProviderRequest {
            name,
            authentication_flow: None,
            authorization_flow,
            invalidation_flow,
            property_mappings: None,
            internal_host: None,
            external_host,
            internal_host_ssl_validation: None,
            certificate: None,
            skip_path_regex: None,
            basic_auth_enabled: None,
            basic_auth_password_attribute: None,
            basic_auth_user_attribute: None,
            mode: None,
            intercept_header_auth: None,
            cookie_domain: None,
            jwt_federation_sources: None,
            jwt_federation_providers: None,
            access_token_validity: None,
            refresh_token_validity: None,
        }
    }
}
