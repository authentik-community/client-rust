/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PatchedRadiusProviderRequest : RadiusProvider Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedRadiusProviderRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Flow used for authentication when the associated application is accessed by an un-authenticated user.
    #[serde(
        rename = "authentication_flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub authentication_flow: Option<Option<uuid::Uuid>>,
    /// Flow used when authorizing this provider.
    #[serde(rename = "authorization_flow", skip_serializing_if = "Option::is_none")]
    pub authorization_flow: Option<uuid::Uuid>,
    /// Flow used ending the session from a provider.
    #[serde(rename = "invalidation_flow", skip_serializing_if = "Option::is_none")]
    pub invalidation_flow: Option<uuid::Uuid>,
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

impl PatchedRadiusProviderRequest {
    /// RadiusProvider Serializer
    pub fn new() -> PatchedRadiusProviderRequest {
        PatchedRadiusProviderRequest {
            name: None,
            authentication_flow: None,
            authorization_flow: None,
            invalidation_flow: None,
            property_mappings: None,
            client_networks: None,
            shared_secret: None,
            mfa_support: None,
        }
    }
}
