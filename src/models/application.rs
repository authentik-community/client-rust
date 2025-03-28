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

/// Application : Application Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    /// Application's display Name.
    #[serde(rename = "name")]
    pub name: String,
    /// Internal application name, used in URLs.
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(
        rename = "provider",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub provider: Option<Option<i32>>,
    #[serde(rename = "provider_obj")]
    pub provider_obj: models::Provider,
    #[serde(rename = "backchannel_providers", skip_serializing_if = "Option::is_none")]
    pub backchannel_providers: Option<Vec<i32>>,
    #[serde(rename = "backchannel_providers_obj")]
    pub backchannel_providers_obj: Vec<models::Provider>,
    /// Allow formatting of launch URL
    #[serde(rename = "launch_url", deserialize_with = "Option::deserialize")]
    pub launch_url: Option<String>,
    /// Open launch URL in a new browser tab or window.
    #[serde(rename = "open_in_new_tab", skip_serializing_if = "Option::is_none")]
    pub open_in_new_tab: Option<bool>,
    #[serde(rename = "meta_launch_url", skip_serializing_if = "Option::is_none")]
    pub meta_launch_url: Option<String>,
    /// Get the URL to the App Icon image. If the name is /static or starts with http it is returned as-is
    #[serde(rename = "meta_icon", deserialize_with = "Option::deserialize")]
    pub meta_icon: Option<String>,
    #[serde(rename = "meta_description", skip_serializing_if = "Option::is_none")]
    pub meta_description: Option<String>,
    #[serde(rename = "meta_publisher", skip_serializing_if = "Option::is_none")]
    pub meta_publisher: Option<String>,
    #[serde(rename = "policy_engine_mode", skip_serializing_if = "Option::is_none")]
    pub policy_engine_mode: Option<models::PolicyEngineMode>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

impl Application {
    /// Application Serializer
    pub fn new(
        pk: uuid::Uuid,
        name: String,
        slug: String,
        provider_obj: models::Provider,
        backchannel_providers_obj: Vec<models::Provider>,
        launch_url: Option<String>,
        meta_icon: Option<String>,
    ) -> Application {
        Application {
            pk,
            name,
            slug,
            provider: None,
            provider_obj,
            backchannel_providers: None,
            backchannel_providers_obj,
            launch_url,
            open_in_new_tab: None,
            meta_launch_url: None,
            meta_icon,
            meta_description: None,
            meta_publisher: None,
            policy_engine_mode: None,
            group: None,
        }
    }
}
