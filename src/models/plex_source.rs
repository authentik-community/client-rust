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

/// PlexSource : Plex Source Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlexSource {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    /// Source's display Name.
    #[serde(rename = "name")]
    pub name: String,
    /// Internal source name, used in URLs.
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Flow to use when authenticating existing users.
    #[serde(
        rename = "authentication_flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub authentication_flow: Option<Option<uuid::Uuid>>,
    /// Flow to use when enrolling new users.
    #[serde(
        rename = "enrollment_flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub enrollment_flow: Option<Option<uuid::Uuid>>,
    #[serde(rename = "user_property_mappings", skip_serializing_if = "Option::is_none")]
    pub user_property_mappings: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "group_property_mappings", skip_serializing_if = "Option::is_none")]
    pub group_property_mappings: Option<Vec<uuid::Uuid>>,
    /// Get object component so that we know how to edit the object
    #[serde(rename = "component")]
    pub component: String,
    /// Return object's verbose_name
    #[serde(rename = "verbose_name")]
    pub verbose_name: String,
    /// Return object's plural verbose_name
    #[serde(rename = "verbose_name_plural")]
    pub verbose_name_plural: String,
    /// Return internal model name
    #[serde(rename = "meta_model_name")]
    pub meta_model_name: String,
    #[serde(rename = "policy_engine_mode", skip_serializing_if = "Option::is_none")]
    pub policy_engine_mode: Option<models::PolicyEngineMode>,
    /// How the source determines if an existing user should be authenticated or a new user enrolled.
    #[serde(rename = "user_matching_mode", skip_serializing_if = "Option::is_none")]
    pub user_matching_mode: Option<models::UserMatchingModeEnum>,
    /// Objects that are managed by authentik. These objects are created and updated automatically. This flag only indicates that an object can be overwritten by migrations. You can still modify the objects via the API, but expect changes to be overwritten in a later update.
    #[serde(rename = "managed", deserialize_with = "Option::deserialize")]
    pub managed: Option<String>,
    #[serde(rename = "user_path_template", skip_serializing_if = "Option::is_none")]
    pub user_path_template: Option<String>,
    #[serde(rename = "icon")]
    pub icon: String,
    /// How the source determines if an existing group should be used or a new group created.
    #[serde(rename = "group_matching_mode", skip_serializing_if = "Option::is_none")]
    pub group_matching_mode: Option<models::GroupMatchingModeEnum>,
    /// Client identifier used to talk to Plex.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Which servers a user has to be a member of to be granted access. Empty list allows every server.
    #[serde(rename = "allowed_servers", skip_serializing_if = "Option::is_none")]
    pub allowed_servers: Option<Vec<String>>,
    /// Allow friends to authenticate, even if you don't share a server.
    #[serde(rename = "allow_friends", skip_serializing_if = "Option::is_none")]
    pub allow_friends: Option<bool>,
    /// Plex token used to check friends
    #[serde(rename = "plex_token")]
    pub plex_token: String,
}

impl PlexSource {
    /// Plex Source Serializer
    pub fn new(
        pk: uuid::Uuid,
        name: String,
        slug: String,
        component: String,
        verbose_name: String,
        verbose_name_plural: String,
        meta_model_name: String,
        managed: Option<String>,
        icon: String,
        plex_token: String,
    ) -> PlexSource {
        PlexSource {
            pk,
            name,
            slug,
            enabled: None,
            authentication_flow: None,
            enrollment_flow: None,
            user_property_mappings: None,
            group_property_mappings: None,
            component,
            verbose_name,
            verbose_name_plural,
            meta_model_name,
            policy_engine_mode: None,
            user_matching_mode: None,
            managed,
            user_path_template: None,
            icon,
            group_matching_mode: None,
            client_id: None,
            allowed_servers: None,
            allow_friends: None,
            plex_token,
        }
    }
}
