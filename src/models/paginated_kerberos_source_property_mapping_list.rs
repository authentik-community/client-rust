/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.6.1
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginatedKerberosSourcePropertyMappingList {
    #[serde(rename = "pagination")]
    pub pagination: models::Pagination,
    #[serde(rename = "results")]
    pub results: Vec<models::KerberosSourcePropertyMapping>,
}

impl PaginatedKerberosSourcePropertyMappingList {
    pub fn new(
        pagination: models::Pagination,
        results: Vec<models::KerberosSourcePropertyMapping>,
    ) -> PaginatedKerberosSourcePropertyMappingList {
        PaginatedKerberosSourcePropertyMappingList { pagination, results }
    }
}
