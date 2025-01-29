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

/// FlowSetRequest : Stripped down flow serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlowSetRequest {
    #[serde(rename = "name")]
    pub name: String,
    /// Visible in the URL.
    #[serde(rename = "slug")]
    pub slug: String,
    /// Shown as the Title in Flow pages.
    #[serde(rename = "title")]
    pub title: String,
    /// Decides what this Flow is used for. For example, the Authentication flow is redirect to when an un-authenticated user visits authentik.
    #[serde(rename = "designation")]
    pub designation: models::FlowDesignationEnum,
    #[serde(rename = "policy_engine_mode", skip_serializing_if = "Option::is_none")]
    pub policy_engine_mode: Option<models::PolicyEngineMode>,
    /// Enable compatibility mode, increases compatibility with password managers on mobile devices.
    #[serde(rename = "compatibility_mode", skip_serializing_if = "Option::is_none")]
    pub compatibility_mode: Option<bool>,
    #[serde(rename = "layout", skip_serializing_if = "Option::is_none")]
    pub layout: Option<models::FlowLayoutEnum>,
    /// Configure what should happen when a flow denies access to a user.
    #[serde(rename = "denied_action", skip_serializing_if = "Option::is_none")]
    pub denied_action: Option<models::DeniedActionEnum>,
}

impl FlowSetRequest {
    /// Stripped down flow serializer
    pub fn new(name: String, slug: String, title: String, designation: models::FlowDesignationEnum) -> FlowSetRequest {
        FlowSetRequest {
            name,
            slug,
            title,
            designation,
            policy_engine_mode: None,
            compatibility_mode: None,
            layout: None,
            denied_action: None,
        }
    }
}
