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

/// ProxyOutpostConfig : Proxy provider serializer for outposts
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyOutpostConfig {
    #[serde(rename = "pk")]
    pub pk: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "internal_host", skip_serializing_if = "Option::is_none")]
    pub internal_host: Option<String>,
    #[serde(rename = "external_host")]
    pub external_host: String,
    /// Validate SSL Certificates of upstream servers
    #[serde(rename = "internal_host_ssl_validation", skip_serializing_if = "Option::is_none")]
    pub internal_host_ssl_validation: Option<bool>,
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "client_secret", skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(rename = "oidc_configuration")]
    pub oidc_configuration: models::OpenIdConnectConfiguration,
    #[serde(rename = "cookie_secret", skip_serializing_if = "Option::is_none")]
    pub cookie_secret: Option<String>,
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
    #[serde(rename = "cookie_domain", skip_serializing_if = "Option::is_none")]
    pub cookie_domain: Option<String>,
    /// Get token validity as second count
    #[serde(rename = "access_token_validity", deserialize_with = "Option::deserialize")]
    pub access_token_validity: Option<f64>,
    /// When enabled, this provider will intercept the authorization header and authenticate requests based on its value.
    #[serde(rename = "intercept_header_auth", skip_serializing_if = "Option::is_none")]
    pub intercept_header_auth: Option<bool>,
    /// Get all the scope names the outpost should request, including custom-defined ones
    #[serde(rename = "scopes_to_request")]
    pub scopes_to_request: Vec<String>,
    /// Internal application name, used in URLs.
    #[serde(rename = "assigned_application_slug")]
    pub assigned_application_slug: String,
    /// Application's display Name.
    #[serde(rename = "assigned_application_name")]
    pub assigned_application_name: String,
}

impl ProxyOutpostConfig {
    /// Proxy provider serializer for outposts
    pub fn new(
        pk: i32,
        name: String,
        external_host: String,
        oidc_configuration: models::OpenIdConnectConfiguration,
        access_token_validity: Option<f64>,
        scopes_to_request: Vec<String>,
        assigned_application_slug: String,
        assigned_application_name: String,
    ) -> ProxyOutpostConfig {
        ProxyOutpostConfig {
            pk,
            name,
            internal_host: None,
            external_host,
            internal_host_ssl_validation: None,
            client_id: None,
            client_secret: None,
            oidc_configuration,
            cookie_secret: None,
            certificate: None,
            skip_path_regex: None,
            basic_auth_enabled: None,
            basic_auth_password_attribute: None,
            basic_auth_user_attribute: None,
            mode: None,
            cookie_domain: None,
            access_token_validity,
            intercept_header_auth: None,
            scopes_to_request,
            assigned_application_slug,
            assigned_application_name,
        }
    }
}
