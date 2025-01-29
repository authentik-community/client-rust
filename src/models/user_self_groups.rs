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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSelfGroups {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "pk")]
    pub pk: String,
}

impl UserSelfGroups {
    pub fn new(name: String, pk: String) -> UserSelfGroups {
        UserSelfGroups { name, pk }
    }
}
