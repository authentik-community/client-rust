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

/// SourceType : Serializer for SourceType
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceType {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "verbose_name")]
    pub verbose_name: String,
    #[serde(rename = "urls_customizable")]
    pub urls_customizable: bool,
    #[serde(rename = "request_token_url", deserialize_with = "Option::deserialize")]
    pub request_token_url: Option<String>,
    #[serde(rename = "authorization_url", deserialize_with = "Option::deserialize")]
    pub authorization_url: Option<String>,
    #[serde(rename = "access_token_url", deserialize_with = "Option::deserialize")]
    pub access_token_url: Option<String>,
    #[serde(rename = "profile_url", deserialize_with = "Option::deserialize")]
    pub profile_url: Option<String>,
    #[serde(rename = "oidc_well_known_url", deserialize_with = "Option::deserialize")]
    pub oidc_well_known_url: Option<String>,
    #[serde(rename = "oidc_jwks_url", deserialize_with = "Option::deserialize")]
    pub oidc_jwks_url: Option<String>,
}

impl SourceType {
    /// Serializer for SourceType
    pub fn new(
        name: String,
        verbose_name: String,
        urls_customizable: bool,
        request_token_url: Option<String>,
        authorization_url: Option<String>,
        access_token_url: Option<String>,
        profile_url: Option<String>,
        oidc_well_known_url: Option<String>,
        oidc_jwks_url: Option<String>,
    ) -> SourceType {
        SourceType {
            name,
            verbose_name,
            urls_customizable,
            request_token_url,
            authorization_url,
            access_token_url,
            profile_url,
            oidc_well_known_url,
            oidc_jwks_url,
        }
    }
}
