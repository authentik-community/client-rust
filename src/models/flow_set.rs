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

/// FlowSet : Stripped down flow serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlowSet {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    #[serde(rename = "policybindingmodel_ptr_id")]
    pub policybindingmodel_ptr_id: uuid::Uuid,
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
    /// Get the URL to the background image. If the name is /static or starts with http it is returned as-is
    #[serde(rename = "background")]
    pub background: String,
    #[serde(rename = "policy_engine_mode", skip_serializing_if = "Option::is_none")]
    pub policy_engine_mode: Option<models::PolicyEngineMode>,
    /// Enable compatibility mode, increases compatibility with password managers on mobile devices.
    #[serde(rename = "compatibility_mode", skip_serializing_if = "Option::is_none")]
    pub compatibility_mode: Option<bool>,
    /// Get export URL for flow
    #[serde(rename = "export_url")]
    pub export_url: String,
    #[serde(rename = "layout", skip_serializing_if = "Option::is_none")]
    pub layout: Option<models::FlowLayoutEnum>,
    /// Configure what should happen when a flow denies access to a user.
    #[serde(rename = "denied_action", skip_serializing_if = "Option::is_none")]
    pub denied_action: Option<models::DeniedActionEnum>,
}

impl FlowSet {
    /// Stripped down flow serializer
    pub fn new(
        pk: uuid::Uuid,
        policybindingmodel_ptr_id: uuid::Uuid,
        name: String,
        slug: String,
        title: String,
        designation: models::FlowDesignationEnum,
        background: String,
        export_url: String,
    ) -> FlowSet {
        FlowSet {
            pk,
            policybindingmodel_ptr_id,
            name,
            slug,
            title,
            designation,
            background,
            policy_engine_mode: None,
            compatibility_mode: None,
            export_url,
            layout: None,
            denied_action: None,
        }
    }
}
