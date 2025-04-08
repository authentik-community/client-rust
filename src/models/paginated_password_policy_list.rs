/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.4
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginatedPasswordPolicyList {
    #[serde(rename = "pagination")]
    pub pagination: models::Pagination,
    #[serde(rename = "results")]
    pub results: Vec<models::PasswordPolicy>,
}

impl PaginatedPasswordPolicyList {
    pub fn new(pagination: models::Pagination, results: Vec<models::PasswordPolicy>) -> PaginatedPasswordPolicyList {
        PaginatedPasswordPolicyList { pagination, results }
    }
}
