/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.1
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UsedBy : A list of all objects referencing the queried object
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsedBy {
    #[serde(rename = "app")]
    pub app: String,
    #[serde(rename = "model_name")]
    pub model_name: String,
    #[serde(rename = "pk")]
    pub pk: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "action")]
    pub action: models::UsedByActionEnum,
}

impl UsedBy {
    /// A list of all objects referencing the queried object
    pub fn new(app: String, model_name: String, pk: String, name: String, action: models::UsedByActionEnum) -> UsedBy {
        UsedBy {
            app,
            model_name,
            pk,
            name,
            action,
        }
    }
}
