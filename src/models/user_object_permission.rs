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

/// UserObjectPermission : User-bound object level permission
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserObjectPermission {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "codename")]
    pub codename: String,
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "app_label")]
    pub app_label: String,
    #[serde(rename = "object_pk")]
    pub object_pk: String,
    #[serde(rename = "name")]
    pub name: String,
}

impl UserObjectPermission {
    /// User-bound object level permission
    pub fn new(
        id: i32,
        codename: String,
        model: String,
        app_label: String,
        object_pk: String,
        name: String,
    ) -> UserObjectPermission {
        UserObjectPermission {
            id,
            codename,
            model,
            app_label,
            object_pk,
            name,
        }
    }
}
