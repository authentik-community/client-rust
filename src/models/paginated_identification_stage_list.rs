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
pub struct PaginatedIdentificationStageList {
    #[serde(rename = "pagination")]
    pub pagination: models::Pagination,
    #[serde(rename = "results")]
    pub results: Vec<models::IdentificationStage>,
    #[serde(rename = "autocomplete")]
    pub autocomplete: std::collections::HashMap<String, serde_json::Value>,
}

impl PaginatedIdentificationStageList {
    pub fn new(
        pagination: models::Pagination,
        results: Vec<models::IdentificationStage>,
        autocomplete: std::collections::HashMap<String, serde_json::Value>,
    ) -> PaginatedIdentificationStageList {
        PaginatedIdentificationStageList {
            pagination,
            results,
            autocomplete,
        }
    }
}
