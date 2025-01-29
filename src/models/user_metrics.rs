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

/// UserMetrics : User Metrics
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserMetrics {
    #[serde(rename = "logins")]
    pub logins: Vec<models::Coordinate>,
    #[serde(rename = "logins_failed")]
    pub logins_failed: Vec<models::Coordinate>,
    #[serde(rename = "authorizations")]
    pub authorizations: Vec<models::Coordinate>,
}

impl UserMetrics {
    /// User Metrics
    pub fn new(
        logins: Vec<models::Coordinate>,
        logins_failed: Vec<models::Coordinate>,
        authorizations: Vec<models::Coordinate>,
    ) -> UserMetrics {
        UserMetrics {
            logins,
            logins_failed,
            authorizations,
        }
    }
}
