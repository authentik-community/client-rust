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

/// ContextualFlowInfo : Contextual flow information for a challenge
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContextualFlowInfo {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "background", skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(rename = "cancel_url")]
    pub cancel_url: String,
    #[serde(rename = "layout")]
    pub layout: models::ContextualFlowInfoLayoutEnum,
}

impl ContextualFlowInfo {
    /// Contextual flow information for a challenge
    pub fn new(cancel_url: String, layout: models::ContextualFlowInfoLayoutEnum) -> ContextualFlowInfo {
        ContextualFlowInfo {
            title: None,
            background: None,
            cancel_url,
            layout,
        }
    }
}
