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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LdapDebug {
    #[serde(rename = "user")]
    pub user: Vec<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "group")]
    pub group: Vec<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "membership")]
    pub membership: Vec<std::collections::HashMap<String, serde_json::Value>>,
}

impl LdapDebug {
    pub fn new(
        user: Vec<std::collections::HashMap<String, serde_json::Value>>,
        group: Vec<std::collections::HashMap<String, serde_json::Value>>,
        membership: Vec<std::collections::HashMap<String, serde_json::Value>>,
    ) -> LdapDebug {
        LdapDebug {
            user,
            group,
            membership,
        }
    }
}
