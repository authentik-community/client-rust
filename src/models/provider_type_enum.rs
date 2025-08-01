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
pub enum ProviderTypeEnum {
    #[serde(rename = "apple")]
    Apple,
    #[serde(rename = "openidconnect")]
    Openidconnect,
    #[serde(rename = "entraid")]
    Entraid,
    #[serde(rename = "azuread")]
    Azuread,
    #[serde(rename = "discord")]
    Discord,
    #[serde(rename = "facebook")]
    Facebook,
    #[serde(rename = "github")]
    Github,
    #[serde(rename = "gitlab")]
    Gitlab,
    #[serde(rename = "google")]
    Google,
    #[serde(rename = "mailcow")]
    Mailcow,
    #[serde(rename = "okta")]
    Okta,
    #[serde(rename = "patreon")]
    Patreon,
    #[serde(rename = "reddit")]
    Reddit,
    #[serde(rename = "twitch")]
    Twitch,
    #[serde(rename = "twitter")]
    Twitter,
}

impl std::fmt::Display for ProviderTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Apple => write!(f, "apple"),
            Self::Openidconnect => write!(f, "openidconnect"),
            Self::Entraid => write!(f, "entraid"),
            Self::Azuread => write!(f, "azuread"),
            Self::Discord => write!(f, "discord"),
            Self::Facebook => write!(f, "facebook"),
            Self::Github => write!(f, "github"),
            Self::Gitlab => write!(f, "gitlab"),
            Self::Google => write!(f, "google"),
            Self::Mailcow => write!(f, "mailcow"),
            Self::Okta => write!(f, "okta"),
            Self::Patreon => write!(f, "patreon"),
            Self::Reddit => write!(f, "reddit"),
            Self::Twitch => write!(f, "twitch"),
            Self::Twitter => write!(f, "twitter"),
        }
    }
}

impl Default for ProviderTypeEnum {
    fn default() -> ProviderTypeEnum {
        Self::Apple
    }
}
