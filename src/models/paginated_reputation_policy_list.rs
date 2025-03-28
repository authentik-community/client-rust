/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginatedReputationPolicyList {
    #[serde(rename = "pagination")]
    pub pagination: models::Pagination,
    #[serde(rename = "results")]
    pub results: Vec<models::ReputationPolicy>,
}

impl PaginatedReputationPolicyList {
    pub fn new(
        pagination: models::Pagination,
        results: Vec<models::ReputationPolicy>,
    ) -> PaginatedReputationPolicyList {
        PaginatedReputationPolicyList { pagination, results }
    }
}
