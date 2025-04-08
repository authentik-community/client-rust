/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.4
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// TokenView : Show token's current key
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenView {
    #[serde(rename = "key")]
    pub key: String,
}

impl TokenView {
    /// Show token's current key
    pub fn new(key: String) -> TokenView {
        TokenView { key }
    }
}
