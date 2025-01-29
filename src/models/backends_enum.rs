/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BackendsEnum {
    #[serde(rename = "authentik.core.auth.InbuiltBackend")]
    CorePeriodAuthPeriodInbuiltBackend,
    #[serde(rename = "authentik.core.auth.TokenBackend")]
    CorePeriodAuthPeriodTokenBackend,
    #[serde(rename = "authentik.sources.ldap.auth.LDAPBackend")]
    SourcesPeriodLdapPeriodAuthPeriodLdapBackend,
    #[serde(rename = "authentik.sources.kerberos.auth.KerberosBackend")]
    SourcesPeriodKerberosPeriodAuthPeriodKerberosBackend,
}

impl std::fmt::Display for BackendsEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CorePeriodAuthPeriodInbuiltBackend => write!(f, "authentik.core.auth.InbuiltBackend"),
            Self::CorePeriodAuthPeriodTokenBackend => write!(f, "authentik.core.auth.TokenBackend"),
            Self::SourcesPeriodLdapPeriodAuthPeriodLdapBackend => write!(f, "authentik.sources.ldap.auth.LDAPBackend"),
            Self::SourcesPeriodKerberosPeriodAuthPeriodKerberosBackend => {
                write!(f, "authentik.sources.kerberos.auth.KerberosBackend")
            }
        }
    }
}

impl Default for BackendsEnum {
    fn default() -> BackendsEnum {
        Self::CorePeriodAuthPeriodInbuiltBackend
    }
}
