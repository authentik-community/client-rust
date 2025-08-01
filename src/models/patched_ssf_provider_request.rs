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

/// PatchedSsfProviderRequest : SSFProvider Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedSsfProviderRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Key used to sign the SSF Events.
    #[serde(rename = "signing_key", skip_serializing_if = "Option::is_none")]
    pub signing_key: Option<uuid::Uuid>,
    #[serde(rename = "oidc_auth_providers", skip_serializing_if = "Option::is_none")]
    pub oidc_auth_providers: Option<Vec<i32>>,
    #[serde(rename = "event_retention", skip_serializing_if = "Option::is_none")]
    pub event_retention: Option<String>,
}

impl PatchedSsfProviderRequest {
    /// SSFProvider Serializer
    pub fn new() -> PatchedSsfProviderRequest {
        PatchedSsfProviderRequest {
            name: None,
            signing_key: None,
            oidc_auth_providers: None,
            event_retention: None,
        }
    }
}
