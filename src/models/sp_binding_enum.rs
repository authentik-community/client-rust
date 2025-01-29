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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SpBindingEnum {
    #[serde(rename = "redirect")]
    Redirect,
    #[serde(rename = "post")]
    Post,
}

impl std::fmt::Display for SpBindingEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Redirect => write!(f, "redirect"),
            Self::Post => write!(f, "post"),
        }
    }
}

impl Default for SpBindingEnum {
    fn default() -> SpBindingEnum {
        Self::Redirect
    }
}
