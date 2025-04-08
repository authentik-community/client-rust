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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogLevelEnum {
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "exception")]
    Exception,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "notset")]
    Notset,
}

impl std::fmt::Display for LogLevelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Critical => write!(f, "critical"),
            Self::Exception => write!(f, "exception"),
            Self::Error => write!(f, "error"),
            Self::Warn => write!(f, "warn"),
            Self::Warning => write!(f, "warning"),
            Self::Info => write!(f, "info"),
            Self::Debug => write!(f, "debug"),
            Self::Notset => write!(f, "notset"),
        }
    }
}

impl Default for LogLevelEnum {
    fn default() -> LogLevelEnum {
        Self::Critical
    }
}
