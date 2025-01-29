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

/// TypeCreate : Types of an object that can be created
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TypeCreate {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "component")]
    pub component: String,
    #[serde(rename = "model_name")]
    pub model_name: String,
    #[serde(rename = "icon_url", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "requires_enterprise", skip_serializing_if = "Option::is_none")]
    pub requires_enterprise: Option<bool>,
}

impl TypeCreate {
    /// Types of an object that can be created
    pub fn new(name: String, description: String, component: String, model_name: String) -> TypeCreate {
        TypeCreate {
            name,
            description,
            component,
            model_name,
            icon_url: None,
            requires_enterprise: None,
        }
    }
}
