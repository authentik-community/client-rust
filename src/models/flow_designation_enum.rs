/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.2
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FlowDesignationEnum {
    #[serde(rename = "authentication")]
    Authentication,
    #[serde(rename = "authorization")]
    Authorization,
    #[serde(rename = "invalidation")]
    Invalidation,
    #[serde(rename = "enrollment")]
    Enrollment,
    #[serde(rename = "unenrollment")]
    Unenrollment,
    #[serde(rename = "recovery")]
    Recovery,
    #[serde(rename = "stage_configuration")]
    StageConfiguration,
}

impl std::fmt::Display for FlowDesignationEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Authentication => write!(f, "authentication"),
            Self::Authorization => write!(f, "authorization"),
            Self::Invalidation => write!(f, "invalidation"),
            Self::Enrollment => write!(f, "enrollment"),
            Self::Unenrollment => write!(f, "unenrollment"),
            Self::Recovery => write!(f, "recovery"),
            Self::StageConfiguration => write!(f, "stage_configuration"),
        }
    }
}

impl Default for FlowDesignationEnum {
    fn default() -> FlowDesignationEnum {
        Self::Authentication
    }
}
