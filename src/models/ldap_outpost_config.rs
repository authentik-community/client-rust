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

/// LdapOutpostConfig : LDAPProvider Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LdapOutpostConfig {
    #[serde(rename = "pk")]
    pub pk: i32,
    #[serde(rename = "name")]
    pub name: String,
    /// DN under which objects are accessible.
    #[serde(rename = "base_dn", skip_serializing_if = "Option::is_none")]
    pub base_dn: Option<String>,
    #[serde(rename = "bind_flow_slug")]
    pub bind_flow_slug: String,
    /// Get slug for unbind flow, defaulting to brand's default flow.
    #[serde(rename = "unbind_flow_slug", deserialize_with = "Option::deserialize")]
    pub unbind_flow_slug: Option<String>,
    /// Prioritise backchannel slug over direct application slug
    #[serde(rename = "application_slug")]
    pub application_slug: String,
    #[serde(
        rename = "certificate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub certificate: Option<Option<uuid::Uuid>>,
    #[serde(rename = "tls_server_name", skip_serializing_if = "Option::is_none")]
    pub tls_server_name: Option<String>,
    /// The start for uidNumbers, this number is added to the user.pk to make sure that the numbers aren't too low for POSIX users. Default is 2000 to ensure that we don't collide with local users uidNumber
    #[serde(rename = "uid_start_number", skip_serializing_if = "Option::is_none")]
    pub uid_start_number: Option<i32>,
    /// The start for gidNumbers, this number is added to a number generated from the group.pk to make sure that the numbers aren't too low for POSIX groups. Default is 4000 to ensure that we don't collide with local groups or users primary groups gidNumber
    #[serde(rename = "gid_start_number", skip_serializing_if = "Option::is_none")]
    pub gid_start_number: Option<i32>,
    #[serde(rename = "search_mode", skip_serializing_if = "Option::is_none")]
    pub search_mode: Option<models::LdapapiAccessMode>,
    #[serde(rename = "bind_mode", skip_serializing_if = "Option::is_none")]
    pub bind_mode: Option<models::LdapapiAccessMode>,
    /// When enabled, code-based multi-factor authentication can be used by appending a semicolon and the TOTP code to the password. This should only be enabled if all users that will bind to this provider have a TOTP device configured, as otherwise a password may incorrectly be rejected if it contains a semicolon.
    #[serde(rename = "mfa_support", skip_serializing_if = "Option::is_none")]
    pub mfa_support: Option<bool>,
}

impl LdapOutpostConfig {
    /// LDAPProvider Serializer
    pub fn new(
        pk: i32,
        name: String,
        bind_flow_slug: String,
        unbind_flow_slug: Option<String>,
        application_slug: String,
    ) -> LdapOutpostConfig {
        LdapOutpostConfig {
            pk,
            name,
            base_dn: None,
            bind_flow_slug,
            unbind_flow_slug,
            application_slug,
            certificate: None,
            tls_server_name: None,
            uid_start_number: None,
            gid_start_number: None,
            search_mode: None,
            bind_mode: None,
            mfa_support: None,
        }
    }
}
