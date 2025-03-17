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

/// PatchedAuthenticatorValidateStageRequest : AuthenticatorValidateStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedAuthenticatorValidateStageRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
    #[serde(rename = "not_configured_action", skip_serializing_if = "Option::is_none")]
    pub not_configured_action: Option<models::NotConfiguredActionEnum>,
    /// Device classes which can be used to authenticate
    #[serde(rename = "device_classes", skip_serializing_if = "Option::is_none")]
    pub device_classes: Option<Vec<models::DeviceClassesEnum>>,
    /// Stages used to configure Authenticator when user doesn't have any compatible devices. After this configuration Stage passes, the user is not prompted again.
    #[serde(rename = "configuration_stages", skip_serializing_if = "Option::is_none")]
    pub configuration_stages: Option<Vec<uuid::Uuid>>,
    /// If any of the user's device has been used within this threshold, this stage will be skipped
    #[serde(rename = "last_auth_threshold", skip_serializing_if = "Option::is_none")]
    pub last_auth_threshold: Option<String>,
    /// Enforce user verification for WebAuthn devices.
    #[serde(rename = "webauthn_user_verification", skip_serializing_if = "Option::is_none")]
    pub webauthn_user_verification: Option<models::UserVerificationEnum>,
    #[serde(rename = "webauthn_allowed_device_types", skip_serializing_if = "Option::is_none")]
    pub webauthn_allowed_device_types: Option<Vec<uuid::Uuid>>,
}

impl PatchedAuthenticatorValidateStageRequest {
    /// AuthenticatorValidateStage Serializer
    pub fn new() -> PatchedAuthenticatorValidateStageRequest {
        PatchedAuthenticatorValidateStageRequest {
            name: None,
            flow_set: None,
            not_configured_action: None,
            device_classes: None,
            configuration_stages: None,
            last_auth_threshold: None,
            webauthn_user_verification: None,
            webauthn_allowed_device_types: None,
        }
    }
}
