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
pub struct DetailedCountry {
    #[serde(rename = "code")]
    pub code: models::CountryCodeEnum,
    #[serde(rename = "name")]
    pub name: String,
}

impl DetailedCountry {
    pub fn new(code: models::CountryCodeEnum, name: String) -> DetailedCountry {
        DetailedCountry { code, name }
    }
}
