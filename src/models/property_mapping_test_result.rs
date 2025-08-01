/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.6.4
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PropertyMappingTestResult : Result of a Property-mapping test
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyMappingTestResult {
    #[serde(rename = "result")]
    pub result: String,
    #[serde(rename = "successful")]
    pub successful: bool,
}

impl PropertyMappingTestResult {
    /// Result of a Property-mapping test
    pub fn new(result: String, successful: bool) -> PropertyMappingTestResult {
        PropertyMappingTestResult { result, successful }
    }
}
