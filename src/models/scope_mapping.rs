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

/// ScopeMapping : ScopeMapping Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopeMapping {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    /// Objects that are managed by authentik. These objects are created and updated automatically. This flag only indicates that an object can be overwritten by migrations. You can still modify the objects via the API, but expect changes to be overwritten in a later update.
    #[serde(
        rename = "managed",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub managed: Option<Option<String>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "expression")]
    pub expression: String,
    /// Get object's component so that we know how to edit the object
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
    /// Scope name requested by the client
    #[serde(rename = "scope_name")]
    pub scope_name: String,
    /// Description shown to the user when consenting. If left empty, the user won't be informed.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ScopeMapping {
    /// ScopeMapping Serializer
    pub fn new(
        pk: uuid::Uuid,
        name: String,
        expression: String,
        component: String,
        verbose_name: String,
        verbose_name_plural: String,
        meta_model_name: String,
        scope_name: String,
    ) -> ScopeMapping {
        ScopeMapping {
            pk,
            managed: None,
            name,
            expression,
            component,
            verbose_name,
            verbose_name_plural,
            meta_model_name,
            scope_name,
            description: None,
        }
    }
}
