/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.0
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// EmailStageRequest : EmailStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailStageRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
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
    /// Time in minutes the token sent is valid.
    #[serde(rename = "token_expiry", skip_serializing_if = "Option::is_none")]
    pub token_expiry: Option<i32>,
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    /// Activate users upon completion of stage.
    #[serde(rename = "activate_user_on_success", skip_serializing_if = "Option::is_none")]
    pub activate_user_on_success: Option<bool>,
}

impl EmailStageRequest {
    /// EmailStage Serializer
    pub fn new(name: String) -> EmailStageRequest {
        EmailStageRequest {
            name,
            flow_set: None,
            use_global_settings: None,
            host: None,
            port: None,
            username: None,
            password: None,
            use_tls: None,
            use_ssl: None,
            timeout: None,
            from_address: None,
            token_expiry: None,
            subject: None,
            template: None,
            activate_user_on_success: None,
        }
    }
}
