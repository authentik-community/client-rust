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

/// SyncObjectResult : Result of a single object sync
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncObjectResult {
    #[serde(rename = "messages")]
    pub messages: Vec<models::LogEvent>,
}

impl SyncObjectResult {
    /// Result of a single object sync
    pub fn new(messages: Vec<models::LogEvent>) -> SyncObjectResult {
        SyncObjectResult { messages }
    }
}
