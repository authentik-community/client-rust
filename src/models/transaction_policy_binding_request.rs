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

/// TransactionPolicyBindingRequest : PolicyBindingSerializer which does not require target as target is set implicitly
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionPolicyBindingRequest {
    #[serde(
        rename = "policy",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub policy: Option<Option<uuid::Uuid>>,
    #[serde(
        rename = "group",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub group: Option<Option<uuid::Uuid>>,
    #[serde(
        rename = "user",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub user: Option<Option<i32>>,
    /// Negates the outcome of the policy. Messages are unaffected.
    #[serde(rename = "negate", skip_serializing_if = "Option::is_none")]
    pub negate: Option<bool>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "order")]
    pub order: i32,
    /// Timeout after which Policy execution is terminated.
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
    /// Result if the Policy execution fails.
    #[serde(rename = "failure_result", skip_serializing_if = "Option::is_none")]
    pub failure_result: Option<bool>,
}

impl TransactionPolicyBindingRequest {
    /// PolicyBindingSerializer which does not require target as target is set implicitly
    pub fn new(order: i32) -> TransactionPolicyBindingRequest {
        TransactionPolicyBindingRequest {
            policy: None,
            group: None,
            user: None,
            negate: None,
            enabled: None,
            order,
            timeout: None,
            failure_result: None,
        }
    }
}
