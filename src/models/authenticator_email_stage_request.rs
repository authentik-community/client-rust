/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.2
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// AuthenticatorEmailStageRequest : AuthenticatorEmailStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatorEmailStageRequest {
    #[serde(rename = "name")]
    pub name: String,
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
    /// When enabled, global Email connection settings will be used and connection settings below will be ignored.
    #[serde(rename = "use_global_settings", skip_serializing_if = "Option::is_none")]
    pub use_global_settings: Option<bool>,
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "use_tls", skip_serializing_if = "Option::is_none")]
    pub use_tls: Option<bool>,
    #[serde(rename = "use_ssl", skip_serializing_if = "Option::is_none")]
    pub use_ssl: Option<bool>,
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "from_address", skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Time the token sent is valid (Format: hours=3,minutes=17,seconds=300).
    #[serde(rename = "token_expiry", skip_serializing_if = "Option::is_none")]
    pub token_expiry: Option<String>,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

impl AuthenticatorEmailStageRequest {
    /// AuthenticatorEmailStage Serializer
    pub fn new(name: String) -> AuthenticatorEmailStageRequest {
        AuthenticatorEmailStageRequest {
            name,
            flow_set: None,
            configure_flow: None,
            friendly_name: None,
            use_global_settings: None,
            host: None,
            port: None,
            username: None,
            password: None,
            use_tls: None,
            use_ssl: None,
            timeout: None,
            from_address: None,
            subject: None,
            token_expiry: None,
            template: None,
        }
    }
}
