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
pub struct PaginatedPasswordStageList {
    #[serde(rename = "pagination")]
    pub pagination: models::Pagination,
    #[serde(rename = "results")]
    pub results: Vec<models::PasswordStage>,
    #[serde(rename = "autocomplete")]
    pub autocomplete: std::collections::HashMap<String, serde_json::Value>,
}

impl PaginatedPasswordStageList {
    pub fn new(
        pagination: models::Pagination,
        results: Vec<models::PasswordStage>,
        autocomplete: std::collections::HashMap<String, serde_json::Value>,
    ) -> PaginatedPasswordStageList {
        PaginatedPasswordStageList {
            pagination,
            results,
            autocomplete,
        }
    }
}
