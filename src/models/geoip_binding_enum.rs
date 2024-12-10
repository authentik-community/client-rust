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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GeoipBindingEnum {
    #[serde(rename = "no_binding")]
    NoBinding,
    #[serde(rename = "bind_continent")]
    BindContinent,
    #[serde(rename = "bind_continent_country")]
    BindContinentCountry,
    #[serde(rename = "bind_continent_country_city")]
    BindContinentCountryCity,
}

impl std::fmt::Display for GeoipBindingEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NoBinding => write!(f, "no_binding"),
            Self::BindContinent => write!(f, "bind_continent"),
            Self::BindContinentCountry => write!(f, "bind_continent_country"),
            Self::BindContinentCountryCity => write!(f, "bind_continent_country_city"),
        }
    }
}

impl Default for GeoipBindingEnum {
    fn default() -> GeoipBindingEnum {
        Self::NoBinding
    }
}
