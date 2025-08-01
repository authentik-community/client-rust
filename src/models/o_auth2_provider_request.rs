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

/// OAuth2ProviderRequest : OAuth2Provider Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuth2ProviderRequest {
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
    /// Confidential clients are capable of maintaining the confidentiality of their credentials. Public clients are incapable
    #[serde(rename = "client_type", skip_serializing_if = "Option::is_none")]
    pub client_type: Option<models::ClientTypeEnum>,
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "client_secret", skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// Access codes not valid on or after current time + this value (Format: hours=1;minutes=2;seconds=3).
    #[serde(rename = "access_code_validity", skip_serializing_if = "Option::is_none")]
    pub access_code_validity: Option<String>,
    /// Tokens not valid on or after current time + this value (Format: hours=1;minutes=2;seconds=3).
    #[serde(rename = "access_token_validity", skip_serializing_if = "Option::is_none")]
    pub access_token_validity: Option<String>,
    /// Tokens not valid on or after current time + this value (Format: hours=1;minutes=2;seconds=3).
    #[serde(rename = "refresh_token_validity", skip_serializing_if = "Option::is_none")]
    pub refresh_token_validity: Option<String>,
    /// Include User claims from scopes in the id_token, for applications that don't access the userinfo endpoint.
    #[serde(rename = "include_claims_in_id_token", skip_serializing_if = "Option::is_none")]
    pub include_claims_in_id_token: Option<bool>,
    /// Key used to sign the tokens.
    #[serde(
        rename = "signing_key",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub signing_key: Option<Option<uuid::Uuid>>,
    /// Key used to encrypt the tokens. When set, tokens will be encrypted and returned as JWEs.
    #[serde(
        rename = "encryption_key",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub encryption_key: Option<Option<uuid::Uuid>>,
    #[serde(rename = "redirect_uris")]
    pub redirect_uris: Vec<models::RedirectUriRequest>,
    /// Configure what data should be used as unique User Identifier. For most cases, the default should be fine.
    #[serde(rename = "sub_mode", skip_serializing_if = "Option::is_none")]
    pub sub_mode: Option<models::SubModeEnum>,
    /// Configure how the issuer field of the ID Token should be filled.
    #[serde(rename = "issuer_mode", skip_serializing_if = "Option::is_none")]
    pub issuer_mode: Option<models::IssuerModeEnum>,
    #[serde(rename = "jwt_federation_sources", skip_serializing_if = "Option::is_none")]
    pub jwt_federation_sources: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "jwt_federation_providers", skip_serializing_if = "Option::is_none")]
    pub jwt_federation_providers: Option<Vec<i32>>,
}

impl OAuth2ProviderRequest {
    /// OAuth2Provider Serializer
    pub fn new(
        name: String,
        authorization_flow: uuid::Uuid,
        invalidation_flow: uuid::Uuid,
        redirect_uris: Vec<models::RedirectUriRequest>,
    ) -> OAuth2ProviderRequest {
        OAuth2ProviderRequest {
            name,
            authentication_flow: None,
            authorization_flow,
            invalidation_flow,
            property_mappings: None,
            client_type: None,
            client_id: None,
            client_secret: None,
            access_code_validity: None,
            access_token_validity: None,
            refresh_token_validity: None,
            include_claims_in_id_token: None,
            signing_key: None,
            encryption_key: None,
            redirect_uris,
            sub_mode: None,
            issuer_mode: None,
            jwt_federation_sources: None,
            jwt_federation_providers: None,
        }
    }
}
