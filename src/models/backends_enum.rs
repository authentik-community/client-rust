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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BackendsEnum {
    #[serde(rename = "authentik.core.auth.InbuiltBackend")]
    AuthentikPeriodCorePeriodAuthPeriodInbuiltBackend,
    #[serde(rename = "authentik.core.auth.TokenBackend")]
    AuthentikPeriodCorePeriodAuthPeriodTokenBackend,
    #[serde(rename = "authentik.sources.ldap.auth.LDAPBackend")]
    AuthentikPeriodSourcesPeriodLdapPeriodAuthPeriodLdapBackend,
    #[serde(rename = "authentik.sources.kerberos.auth.KerberosBackend")]
    AuthentikPeriodSourcesPeriodKerberosPeriodAuthPeriodKerberosBackend,
}

impl std::fmt::Display for BackendsEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AuthentikPeriodCorePeriodAuthPeriodInbuiltBackend => write!(f, "authentik.core.auth.InbuiltBackend"),
            Self::AuthentikPeriodCorePeriodAuthPeriodTokenBackend => write!(f, "authentik.core.auth.TokenBackend"),
            Self::AuthentikPeriodSourcesPeriodLdapPeriodAuthPeriodLdapBackend => {
                write!(f, "authentik.sources.ldap.auth.LDAPBackend")
            }
            Self::AuthentikPeriodSourcesPeriodKerberosPeriodAuthPeriodKerberosBackend => {
                write!(f, "authentik.sources.kerberos.auth.KerberosBackend")
            }
        }
    }
}

impl Default for BackendsEnum {
    fn default() -> BackendsEnum {
        Self::AuthentikPeriodCorePeriodAuthPeriodInbuiltBackend
    }
}
