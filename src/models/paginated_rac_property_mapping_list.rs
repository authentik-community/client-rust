/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.2
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginatedRacPropertyMappingList {
    #[serde(rename = "pagination")]
    pub pagination: models::Pagination,
    #[serde(rename = "results")]
    pub results: Vec<models::RacPropertyMapping>,
}

impl PaginatedRacPropertyMappingList {
    pub fn new(
        pagination: models::Pagination,
        results: Vec<models::RacPropertyMapping>,
    ) -> PaginatedRacPropertyMappingList {
        PaginatedRacPropertyMappingList { pagination, results }
    }
}
