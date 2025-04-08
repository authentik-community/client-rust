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

/// PasswordPolicyRequest : Password Policy Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PasswordPolicyRequest {
    #[serde(rename = "name")]
    pub name: String,
    /// When this option is enabled, all executions of this policy will be logged. By default, only execution errors are logged.
    #[serde(rename = "execution_logging", skip_serializing_if = "Option::is_none")]
    pub execution_logging: Option<bool>,
    /// Field key to check, field keys defined in Prompt stages are available.
    #[serde(rename = "password_field", skip_serializing_if = "Option::is_none")]
    pub password_field: Option<String>,
    #[serde(rename = "amount_digits", skip_serializing_if = "Option::is_none")]
    pub amount_digits: Option<u32>,
    #[serde(rename = "amount_uppercase", skip_serializing_if = "Option::is_none")]
    pub amount_uppercase: Option<u32>,
    #[serde(rename = "amount_lowercase", skip_serializing_if = "Option::is_none")]
    pub amount_lowercase: Option<u32>,
    #[serde(rename = "amount_symbols", skip_serializing_if = "Option::is_none")]
    pub amount_symbols: Option<u32>,
    #[serde(rename = "length_min", skip_serializing_if = "Option::is_none")]
    pub length_min: Option<u32>,
    #[serde(rename = "symbol_charset", skip_serializing_if = "Option::is_none")]
    pub symbol_charset: Option<String>,
    #[serde(rename = "error_message", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "check_static_rules", skip_serializing_if = "Option::is_none")]
    pub check_static_rules: Option<bool>,
    #[serde(rename = "check_have_i_been_pwned", skip_serializing_if = "Option::is_none")]
    pub check_have_i_been_pwned: Option<bool>,
    #[serde(rename = "check_zxcvbn", skip_serializing_if = "Option::is_none")]
    pub check_zxcvbn: Option<bool>,
    /// How many times the password hash is allowed to be on haveibeenpwned
    #[serde(rename = "hibp_allowed_count", skip_serializing_if = "Option::is_none")]
    pub hibp_allowed_count: Option<u32>,
    /// If the zxcvbn score is equal or less than this value, the policy will fail.
    #[serde(rename = "zxcvbn_score_threshold", skip_serializing_if = "Option::is_none")]
    pub zxcvbn_score_threshold: Option<u32>,
}

impl PasswordPolicyRequest {
    /// Password Policy Serializer
    pub fn new(name: String) -> PasswordPolicyRequest {
        PasswordPolicyRequest {
            name,
            execution_logging: None,
            password_field: None,
            amount_digits: None,
            amount_uppercase: None,
            amount_lowercase: None,
            amount_symbols: None,
            length_min: None,
            symbol_charset: None,
            error_message: None,
            check_static_rules: None,
            check_have_i_been_pwned: None,
            check_zxcvbn: None,
            hibp_allowed_count: None,
            zxcvbn_score_threshold: None,
        }
    }
}
