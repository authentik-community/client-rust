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
pub enum ProxyMode {
    #[serde(rename = "proxy")]
    Proxy,
    #[serde(rename = "forward_single")]
    ForwardSingle,
    #[serde(rename = "forward_domain")]
    ForwardDomain,
}

impl std::fmt::Display for ProxyMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Proxy => write!(f, "proxy"),
            Self::ForwardSingle => write!(f, "forward_single"),
            Self::ForwardDomain => write!(f, "forward_domain"),
        }
    }
}

impl Default for ProxyMode {
    fn default() -> ProxyMode {
        Self::Proxy
    }
}
