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

/// Coordinate : Coordinates for diagrams
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Coordinate {
    #[serde(rename = "x_cord")]
    pub x_cord: i32,
    #[serde(rename = "y_cord")]
    pub y_cord: i32,
}

impl Coordinate {
    /// Coordinates for diagrams
    pub fn new(x_cord: i32, y_cord: i32) -> Coordinate {
        Coordinate { x_cord, y_cord }
    }
}
