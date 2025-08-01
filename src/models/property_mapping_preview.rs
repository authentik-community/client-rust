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

/// PropertyMappingPreview : Preview how the current user is mapped via the property mappings selected in a provider
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyMappingPreview {
    #[serde(rename = "preview")]
    pub preview: std::collections::HashMap<String, serde_json::Value>,
}

impl PropertyMappingPreview {
    /// Preview how the current user is mapped via the property mappings selected in a provider
    pub fn new(preview: std::collections::HashMap<String, serde_json::Value>) -> PropertyMappingPreview {
        PropertyMappingPreview { preview }
    }
}
