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
pub struct ScheduleRequest {
    #[serde(
        rename = "rel_obj_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub rel_obj_id: Option<Option<String>>,
    /// When to schedule tasks
    #[serde(rename = "crontab")]
    pub crontab: String,
    /// Pause this schedule
    #[serde(rename = "paused", skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
}

impl ScheduleRequest {
    pub fn new(crontab: String) -> ScheduleRequest {
        ScheduleRequest {
            rel_obj_id: None,
            crontab,
            paused: None,
        }
    }
}
