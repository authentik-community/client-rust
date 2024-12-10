/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.5
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pagination {
    #[serde(rename = "next")]
    pub next: f64,
    #[serde(rename = "previous")]
    pub previous: f64,
    #[serde(rename = "count")]
    pub count: f64,
    #[serde(rename = "current")]
    pub current: f64,
    #[serde(rename = "total_pages")]
    pub total_pages: f64,
    #[serde(rename = "start_index")]
    pub start_index: f64,
    #[serde(rename = "end_index")]
    pub end_index: f64,
}

impl Pagination {
    pub fn new(
        next: f64,
        previous: f64,
        count: f64,
        current: f64,
        total_pages: f64,
        start_index: f64,
        end_index: f64,
    ) -> Pagination {
        Pagination {
            next,
            previous,
            count,
            current,
            total_pages,
            start_index,
            end_index,
        }
    }
}
