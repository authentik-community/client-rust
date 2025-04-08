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

/// AuthenticatorSmsStageRequest : AuthenticatorSMSStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatorSmsStageRequest {
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
    #[serde(rename = "provider")]
    pub provider: models::ProviderEnum,
    #[serde(rename = "from_number")]
    pub from_number: String,
    #[serde(rename = "account_sid")]
    pub account_sid: String,
    #[serde(rename = "auth")]
    pub auth: String,
    #[serde(rename = "auth_password", skip_serializing_if = "Option::is_none")]
    pub auth_password: Option<String>,
    #[serde(rename = "auth_type", skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<models::AuthTypeEnum>,
    /// When enabled, the Phone number is only used during enrollment to verify the users authenticity. Only a hash of the phone number is saved to ensure it is not reused in the future.
    #[serde(rename = "verify_only", skip_serializing_if = "Option::is_none")]
    pub verify_only: Option<bool>,
    /// Optionally modify the payload being sent to custom providers.
    #[serde(
        rename = "mapping",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub mapping: Option<Option<uuid::Uuid>>,
}

impl AuthenticatorSmsStageRequest {
    /// AuthenticatorSMSStage Serializer
    pub fn new(
        name: String,
        provider: models::ProviderEnum,
        from_number: String,
        account_sid: String,
        auth: String,
    ) -> AuthenticatorSmsStageRequest {
        AuthenticatorSmsStageRequest {
            name,
            flow_set: None,
            configure_flow: None,
            friendly_name: None,
            provider,
            from_number,
            account_sid,
            auth,
            auth_password: None,
            auth_type: None,
            verify_only: None,
            mapping: None,
        }
    }
}
