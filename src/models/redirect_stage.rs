/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// RedirectStage : RedirectStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedirectStage {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    /// Get object type so that we know how to edit the object
    #[serde(rename = "component")]
    pub component: String,
    /// Return object's verbose_name
    #[serde(rename = "verbose_name")]
    pub verbose_name: String,
    /// Return object's plural verbose_name
    #[serde(rename = "verbose_name_plural")]
    pub verbose_name_plural: String,
    /// Return internal model name
    #[serde(rename = "meta_model_name")]
    pub meta_model_name: String,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSet>>,
    #[serde(rename = "keep_context", skip_serializing_if = "Option::is_none")]
    pub keep_context: Option<bool>,
    #[serde(rename = "mode")]
    pub mode: models::RedirectStageModeEnum,
    #[serde(rename = "target_static", skip_serializing_if = "Option::is_none")]
    pub target_static: Option<String>,
    #[serde(
        rename = "target_flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_flow: Option<Option<uuid::Uuid>>,
}

impl RedirectStage {
    /// RedirectStage Serializer
    pub fn new(
        pk: uuid::Uuid,
        name: String,
        component: String,
        verbose_name: String,
        verbose_name_plural: String,
        meta_model_name: String,
        mode: models::RedirectStageModeEnum,
    ) -> RedirectStage {
        RedirectStage {
            pk,
            name,
            component,
            verbose_name,
            verbose_name_plural,
            meta_model_name,
            flow_set: None,
            keep_context: None,
            mode,
            target_static: None,
            target_flow: None,
        }
    }
}
