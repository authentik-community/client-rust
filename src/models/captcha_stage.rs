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

/// CaptchaStage : CaptchaStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CaptchaStage {
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
    /// Public key, acquired your captcha Provider.
    #[serde(rename = "public_key")]
    pub public_key: String,
    #[serde(rename = "js_url", skip_serializing_if = "Option::is_none")]
    pub js_url: Option<String>,
    #[serde(rename = "api_url", skip_serializing_if = "Option::is_none")]
    pub api_url: Option<String>,
    #[serde(rename = "interactive", skip_serializing_if = "Option::is_none")]
    pub interactive: Option<bool>,
    #[serde(rename = "score_min_threshold", skip_serializing_if = "Option::is_none")]
    pub score_min_threshold: Option<f64>,
    #[serde(rename = "score_max_threshold", skip_serializing_if = "Option::is_none")]
    pub score_max_threshold: Option<f64>,
    /// When enabled and the received captcha score is outside of the given threshold, the stage will show an error message. When not enabled, the flow will continue, but the data from the captcha will be available in the context for policy decisions
    #[serde(rename = "error_on_invalid_score", skip_serializing_if = "Option::is_none")]
    pub error_on_invalid_score: Option<bool>,
}

impl CaptchaStage {
    /// CaptchaStage Serializer
    pub fn new(
        pk: uuid::Uuid,
        name: String,
        component: String,
        verbose_name: String,
        verbose_name_plural: String,
        meta_model_name: String,
        public_key: String,
    ) -> CaptchaStage {
        CaptchaStage {
            pk,
            name,
            component,
            verbose_name,
            verbose_name_plural,
            meta_model_name,
            flow_set: None,
            public_key,
            js_url: None,
            api_url: None,
            interactive: None,
            score_min_threshold: None,
            score_max_threshold: None,
            error_on_invalid_score: None,
        }
    }
}
