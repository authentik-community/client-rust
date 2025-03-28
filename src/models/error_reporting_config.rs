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

/// ErrorReportingConfig : Config for error reporting
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorReportingConfig {
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "sentry_dsn")]
    pub sentry_dsn: String,
    #[serde(rename = "environment")]
    pub environment: String,
    #[serde(rename = "send_pii")]
    pub send_pii: bool,
    #[serde(rename = "traces_sample_rate")]
    pub traces_sample_rate: f64,
}

impl ErrorReportingConfig {
    /// Config for error reporting
    pub fn new(
        enabled: bool,
        sentry_dsn: String,
        environment: String,
        send_pii: bool,
        traces_sample_rate: f64,
    ) -> ErrorReportingConfig {
        ErrorReportingConfig {
            enabled,
            sentry_dsn,
            environment,
            send_pii,
            traces_sample_rate,
        }
    }
}
