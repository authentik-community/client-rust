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

/// AuthenticatedSessionGeoIp : Get GeoIP Data
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatedSessionGeoIp {
    #[serde(rename = "continent")]
    pub continent: String,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "lat")]
    pub lat: f64,
    #[serde(rename = "long")]
    pub long: f64,
    #[serde(rename = "city")]
    pub city: String,
}

impl AuthenticatedSessionGeoIp {
    /// Get GeoIP Data
    pub fn new(continent: String, country: String, lat: f64, long: f64, city: String) -> AuthenticatedSessionGeoIp {
        AuthenticatedSessionGeoIp {
            continent,
            country,
            lat,
            long,
            city,
        }
    }
}
