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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginatedUserLogoutStageList {
    #[serde(rename = "pagination")]
    pub pagination: models::Pagination,
    #[serde(rename = "results")]
    pub results: Vec<models::UserLogoutStage>,
    #[serde(rename = "autocomplete")]
    pub autocomplete: std::collections::HashMap<String, serde_json::Value>,
}

impl PaginatedUserLogoutStageList {
    pub fn new(
        pagination: models::Pagination,
        results: Vec<models::UserLogoutStage>,
        autocomplete: std::collections::HashMap<String, serde_json::Value>,
    ) -> PaginatedUserLogoutStageList {
        PaginatedUserLogoutStageList {
            pagination,
            results,
            autocomplete,
        }
    }
}
