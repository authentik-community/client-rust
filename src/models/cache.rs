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

/// Cache : Generic cache stats for an object
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cache {
    #[serde(rename = "count")]
    pub count: i32,
}

impl Cache {
    /// Generic cache stats for an object
    pub fn new(count: i32) -> Cache {
        Cache { count }
    }
}
