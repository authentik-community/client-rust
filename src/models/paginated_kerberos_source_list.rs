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
pub struct PaginatedKerberosSourceList {
    #[serde(rename = "pagination")]
    pub pagination: models::Pagination,
    #[serde(rename = "results")]
    pub results: Vec<models::KerberosSource>,
}

impl PaginatedKerberosSourceList {
    pub fn new(pagination: models::Pagination, results: Vec<models::KerberosSource>) -> PaginatedKerberosSourceList {
        PaginatedKerberosSourceList { pagination, results }
    }
}
