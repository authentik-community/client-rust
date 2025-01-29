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

/// UserLoginStage : UserLoginStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserLoginStage {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    /// Get object type so that we know how to edit the object
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
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSet>>,
    /// Determines how long a session lasts. Default of 0 means that the sessions lasts until the browser is closed. (Format: hours=-1;minutes=-2;seconds=-3)
    #[serde(rename = "session_duration", skip_serializing_if = "Option::is_none")]
    pub session_duration: Option<String>,
    /// Terminate all other sessions of the user logging in.
    #[serde(rename = "terminate_other_sessions", skip_serializing_if = "Option::is_none")]
    pub terminate_other_sessions: Option<bool>,
    /// Offset the session will be extended by when the user picks the remember me option. Default of 0 means that the remember me option will not be shown. (Format: hours=-1;minutes=-2;seconds=-3)
    #[serde(rename = "remember_me_offset", skip_serializing_if = "Option::is_none")]
    pub remember_me_offset: Option<String>,
    /// Bind sessions created by this stage to the configured network
    #[serde(rename = "network_binding", skip_serializing_if = "Option::is_none")]
    pub network_binding: Option<models::NetworkBindingEnum>,
    /// Bind sessions created by this stage to the configured GeoIP location
    #[serde(rename = "geoip_binding", skip_serializing_if = "Option::is_none")]
    pub geoip_binding: Option<models::GeoipBindingEnum>,
}

impl UserLoginStage {
    /// UserLoginStage Serializer
    pub fn new(
        pk: uuid::Uuid,
        name: String,
        component: String,
        verbose_name: String,
        verbose_name_plural: String,
        meta_model_name: String,
    ) -> UserLoginStage {
        UserLoginStage {
            pk,
            name,
            component,
            verbose_name,
            verbose_name_plural,
            meta_model_name,
            flow_set: None,
            session_duration: None,
            terminate_other_sessions: None,
            remember_me_offset: None,
            network_binding: None,
            geoip_binding: None,
        }
    }
}
