/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.1
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DigestAlgorithmEnum {
    #[serde(rename = "http://www.w3.org/2000/09/xmldsig#sha1")]
    Variant2000Slash09SlashXmldsigHashSha1,
    #[serde(rename = "http://www.w3.org/2001/04/xmlenc#sha256")]
    Variant2001Slash04SlashXmlencHashSha256,
    #[serde(rename = "http://www.w3.org/2001/04/xmldsig-more#sha384")]
    Variant2001Slash04SlashXmldsigMoreHashSha384,
    #[serde(rename = "http://www.w3.org/2001/04/xmlenc#sha512")]
    Variant2001Slash04SlashXmlencHashSha512,
}

impl std::fmt::Display for DigestAlgorithmEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant2000Slash09SlashXmldsigHashSha1 => write!(f, "http://www.w3.org/2000/09/xmldsig#sha1"),
            Self::Variant2001Slash04SlashXmlencHashSha256 => write!(f, "http://www.w3.org/2001/04/xmlenc#sha256"),
            Self::Variant2001Slash04SlashXmldsigMoreHashSha384 => {
                write!(f, "http://www.w3.org/2001/04/xmldsig-more#sha384")
            }
            Self::Variant2001Slash04SlashXmlencHashSha512 => write!(f, "http://www.w3.org/2001/04/xmlenc#sha512"),
        }
    }
}

impl Default for DigestAlgorithmEnum {
    fn default() -> DigestAlgorithmEnum {
        Self::Variant2000Slash09SlashXmldsigHashSha1
    }
}
