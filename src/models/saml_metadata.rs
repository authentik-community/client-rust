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

/// SamlMetadata : SAML Provider Metadata serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SamlMetadata {
    #[serde(rename = "metadata")]
    pub metadata: String,
    #[serde(rename = "download_url")]
    pub download_url: String,
}

impl SamlMetadata {
    /// SAML Provider Metadata serializer
    pub fn new(metadata: String, download_url: String) -> SamlMetadata {
        SamlMetadata { metadata, download_url }
    }
}
