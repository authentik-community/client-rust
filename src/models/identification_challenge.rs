/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.5
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// IdentificationChallenge : Identification challenges with all UI elements
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentificationChallenge {
    #[serde(rename = "flow_info", skip_serializing_if = "Option::is_none")]
    pub flow_info: Option<models::ContextualFlowInfo>,
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "response_errors", skip_serializing_if = "Option::is_none")]
    pub response_errors: Option<std::collections::HashMap<String, Vec<models::ErrorDetail>>>,
    #[serde(rename = "user_fields", deserialize_with = "Option::deserialize")]
    pub user_fields: Option<Vec<String>>,
    #[serde(rename = "password_fields")]
    pub password_fields: bool,
    #[serde(rename = "allow_show_password", skip_serializing_if = "Option::is_none")]
    pub allow_show_password: Option<bool>,
    #[serde(rename = "application_pre", skip_serializing_if = "Option::is_none")]
    pub application_pre: Option<String>,
    #[serde(rename = "flow_designation")]
    pub flow_designation: models::FlowDesignationEnum,
    #[serde(rename = "captcha_stage", skip_serializing_if = "Option::is_none")]
    pub captcha_stage: Option<models::CaptchaChallenge>,
    #[serde(rename = "enroll_url", skip_serializing_if = "Option::is_none")]
    pub enroll_url: Option<String>,
    #[serde(rename = "recovery_url", skip_serializing_if = "Option::is_none")]
    pub recovery_url: Option<String>,
    #[serde(rename = "passwordless_url", skip_serializing_if = "Option::is_none")]
    pub passwordless_url: Option<String>,
    #[serde(rename = "primary_action")]
    pub primary_action: String,
    #[serde(rename = "sources", skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<models::LoginSource>>,
    #[serde(rename = "show_source_labels")]
    pub show_source_labels: bool,
}

impl IdentificationChallenge {
    /// Identification challenges with all UI elements
    pub fn new(
        user_fields: Option<Vec<String>>,
        password_fields: bool,
        flow_designation: models::FlowDesignationEnum,
        primary_action: String,
        show_source_labels: bool,
    ) -> IdentificationChallenge {
        IdentificationChallenge {
            flow_info: None,
            component: None,
            response_errors: None,
            user_fields,
            password_fields,
            allow_show_password: None,
            application_pre: None,
            flow_designation,
            captcha_stage: None,
            enroll_url: None,
            recovery_url: None,
            passwordless_url: None,
            primary_action,
            sources: None,
            show_source_labels,
        }
    }
}
