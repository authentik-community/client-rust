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

/// FilePathRequest : Serializer to upload file
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilePathRequest {
    #[serde(rename = "url")]
    pub url: String,
}

impl FilePathRequest {
    /// Serializer to upload file
    pub fn new(url: String) -> FilePathRequest {
        FilePathRequest { url }
    }
}
