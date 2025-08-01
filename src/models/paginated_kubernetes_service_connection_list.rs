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
pub struct PaginatedKubernetesServiceConnectionList {
    #[serde(rename = "pagination")]
    pub pagination: models::Pagination,
    #[serde(rename = "results")]
    pub results: Vec<models::KubernetesServiceConnection>,
    #[serde(rename = "autocomplete")]
    pub autocomplete: std::collections::HashMap<String, serde_json::Value>,
}

impl PaginatedKubernetesServiceConnectionList {
    pub fn new(
        pagination: models::Pagination,
        results: Vec<models::KubernetesServiceConnection>,
        autocomplete: std::collections::HashMap<String, serde_json::Value>,
    ) -> PaginatedKubernetesServiceConnectionList {
        PaginatedKubernetesServiceConnectionList {
            pagination,
            results,
            autocomplete,
        }
    }
}
