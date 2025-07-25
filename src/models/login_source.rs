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

/// LoginSource : Serializer for Login buttons of sources
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginSource {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(
        rename = "icon_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_url: Option<Option<String>>,
    #[serde(rename = "challenge")]
    pub challenge: models::LoginChallengeTypes,
}

impl LoginSource {
    /// Serializer for Login buttons of sources
    pub fn new(name: String, challenge: models::LoginChallengeTypes) -> LoginSource {
        LoginSource {
            name,
            icon_url: None,
            challenge,
        }
    }
}
