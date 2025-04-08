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

/// OpenIdConnectConfiguration : rest_framework Serializer for OIDC Configuration
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpenIdConnectConfiguration {
    #[serde(rename = "issuer")]
    pub issuer: String,
    #[serde(rename = "authorization_endpoint")]
    pub authorization_endpoint: String,
    #[serde(rename = "token_endpoint")]
    pub token_endpoint: String,
    #[serde(rename = "userinfo_endpoint")]
    pub userinfo_endpoint: String,
    #[serde(rename = "end_session_endpoint")]
    pub end_session_endpoint: String,
    #[serde(rename = "introspection_endpoint")]
    pub introspection_endpoint: String,
    #[serde(rename = "jwks_uri")]
    pub jwks_uri: String,
    #[serde(rename = "response_types_supported")]
    pub response_types_supported: Vec<String>,
    #[serde(rename = "id_token_signing_alg_values_supported")]
    pub id_token_signing_alg_values_supported: Vec<String>,
    #[serde(rename = "subject_types_supported")]
    pub subject_types_supported: Vec<String>,
    #[serde(rename = "token_endpoint_auth_methods_supported")]
    pub token_endpoint_auth_methods_supported: Vec<String>,
}

impl OpenIdConnectConfiguration {
    /// rest_framework Serializer for OIDC Configuration
    pub fn new(
        issuer: String,
        authorization_endpoint: String,
        token_endpoint: String,
        userinfo_endpoint: String,
        end_session_endpoint: String,
        introspection_endpoint: String,
        jwks_uri: String,
        response_types_supported: Vec<String>,
        id_token_signing_alg_values_supported: Vec<String>,
        subject_types_supported: Vec<String>,
        token_endpoint_auth_methods_supported: Vec<String>,
    ) -> OpenIdConnectConfiguration {
        OpenIdConnectConfiguration {
            issuer,
            authorization_endpoint,
            token_endpoint,
            userinfo_endpoint,
            end_session_endpoint,
            introspection_endpoint,
            jwks_uri,
            response_types_supported,
            id_token_signing_alg_values_supported,
            subject_types_supported,
            token_endpoint_auth_methods_supported,
        }
    }
}
