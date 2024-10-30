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

/// PasswordStage : PasswordStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PasswordStage {
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
    /// Selection of backends to test the password against.
    #[serde(rename = "backends")]
    pub backends: Vec<models::BackendsEnum>,
    /// Flow used by an authenticated user to configure this Stage. If empty, user will not be able to configure this stage.
    #[serde(
        rename = "configure_flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub configure_flow: Option<Option<uuid::Uuid>>,
    /// How many attempts a user has before the flow is canceled. To lock the user out, use a reputation policy and a user_write stage.
    #[serde(rename = "failed_attempts_before_cancel", skip_serializing_if = "Option::is_none")]
    pub failed_attempts_before_cancel: Option<i32>,
    /// When enabled, provides a 'show password' button with the password input field.
    #[serde(rename = "allow_show_password", skip_serializing_if = "Option::is_none")]
    pub allow_show_password: Option<bool>,
}

impl PasswordStage {
    /// PasswordStage Serializer
    pub fn new(
        pk: uuid::Uuid,
        name: String,
        component: String,
        verbose_name: String,
        verbose_name_plural: String,
        meta_model_name: String,
        backends: Vec<models::BackendsEnum>,
    ) -> PasswordStage {
        PasswordStage {
            pk,
            name,
            component,
            verbose_name,
            verbose_name_plural,
            meta_model_name,
            flow_set: None,
            backends,
            configure_flow: None,
            failed_attempts_before_cancel: None,
            allow_show_password: None,
        }
    }
}
