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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "component")]
pub enum LoginChallengeTypes {
    #[serde(rename = "xak-flow-redirect")]
    XakFlowRedirect(models::RedirectChallenge),
    #[serde(rename = "ak-source-oauth-apple")]
    AkSourceOauthApple(models::AppleLoginChallenge),
    #[serde(rename = "ak-source-plex")]
    AkSourcePlex(models::PlexAuthenticationChallenge),
}

impl Default for LoginChallengeTypes {
    fn default() -> Self {
        Self::XakFlowRedirect(Default::default())
    }
}
