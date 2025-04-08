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

/// PatchedAuthenticatorDuoStageRequest : AuthenticatorDuoStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedAuthenticatorDuoStageRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
    /// Flow used by an authenticated user to configure this Stage. If empty, user will not be able to configure this stage.
    #[serde(
        rename = "configure_flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub configure_flow: Option<Option<uuid::Uuid>>,
    #[serde(
        rename = "friendly_name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub friendly_name: Option<Option<String>>,
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "client_secret", skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(rename = "api_hostname", skip_serializing_if = "Option::is_none")]
    pub api_hostname: Option<String>,
    #[serde(rename = "admin_integration_key", skip_serializing_if = "Option::is_none")]
    pub admin_integration_key: Option<String>,
    #[serde(rename = "admin_secret_key", skip_serializing_if = "Option::is_none")]
    pub admin_secret_key: Option<String>,
}

impl PatchedAuthenticatorDuoStageRequest {
    /// AuthenticatorDuoStage Serializer
    pub fn new() -> PatchedAuthenticatorDuoStageRequest {
        PatchedAuthenticatorDuoStageRequest {
            name: None,
            flow_set: None,
            configure_flow: None,
            friendly_name: None,
            client_id: None,
            client_secret: None,
            api_hostname: None,
            admin_integration_key: None,
            admin_secret_key: None,
        }
    }
}
