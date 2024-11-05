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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginatedDummyPolicyList {
    #[serde(rename = "pagination")]
    pub pagination: models::Pagination,
    #[serde(rename = "results")]
    pub results: Vec<models::DummyPolicy>,
}

impl PaginatedDummyPolicyList {
    pub fn new(pagination: models::Pagination, results: Vec<models::DummyPolicy>) -> PaginatedDummyPolicyList {
        PaginatedDummyPolicyList { pagination, results }
    }
}
