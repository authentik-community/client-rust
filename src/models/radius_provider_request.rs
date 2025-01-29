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

/// RadiusProviderRequest : RadiusProvider Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RadiusProviderRequest {
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
    /// List of CIDRs (comma-separated) that clients can connect from. A more specific CIDR will match before a looser one. Clients connecting from a non-specified CIDR will be dropped.
    #[serde(rename = "client_networks", skip_serializing_if = "Option::is_none")]
    pub client_networks: Option<String>,
    /// Shared secret between clients and server to hash packets.
    #[serde(rename = "shared_secret", skip_serializing_if = "Option::is_none")]
    pub shared_secret: Option<String>,
    /// When enabled, code-based multi-factor authentication can be used by appending a semicolon and the TOTP code to the password. This should only be enabled if all users that will bind to this provider have a TOTP device configured, as otherwise a password may incorrectly be rejected if it contains a semicolon.
    #[serde(rename = "mfa_support", skip_serializing_if = "Option::is_none")]
    pub mfa_support: Option<bool>,
}

impl RadiusProviderRequest {
    /// RadiusProvider Serializer
    pub fn new(name: String, authorization_flow: uuid::Uuid, invalidation_flow: uuid::Uuid) -> RadiusProviderRequest {
        RadiusProviderRequest {
            name,
            authentication_flow: None,
            authorization_flow,
            invalidation_flow,
            property_mappings: None,
            client_networks: None,
            shared_secret: None,
            mfa_support: None,
        }
    }
}
