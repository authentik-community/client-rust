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
pub struct PaginatedTotpDeviceList {
    #[serde(rename = "pagination")]
    pub pagination: models::Pagination,
    #[serde(rename = "results")]
    pub results: Vec<models::TotpDevice>,
}

impl PaginatedTotpDeviceList {
    pub fn new(pagination: models::Pagination, results: Vec<models::TotpDevice>) -> PaginatedTotpDeviceList {
        PaginatedTotpDeviceList { pagination, results }
    }
}
