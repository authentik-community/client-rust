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

/// GroupKerberosSourceConnectionRequest : OAuth Group-Source connection Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupKerberosSourceConnectionRequest {
    #[serde(rename = "group")]
    pub group: uuid::Uuid,
    #[serde(rename = "source")]
    pub source: uuid::Uuid,
    #[serde(rename = "identifier")]
    pub identifier: String,
}

impl GroupKerberosSourceConnectionRequest {
    /// OAuth Group-Source connection Serializer
    pub fn new(group: uuid::Uuid, source: uuid::Uuid, identifier: String) -> GroupKerberosSourceConnectionRequest {
        GroupKerberosSourceConnectionRequest {
            group,
            source,
            identifier,
        }
    }
}