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

/// Outpost : Outpost Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Outpost {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: models::OutpostTypeEnum,
    #[serde(rename = "providers")]
    pub providers: Vec<i32>,
    #[serde(rename = "providers_obj")]
    pub providers_obj: Vec<models::Provider>,
    /// Select Service-Connection authentik should use to manage this outpost. Leave empty if authentik should not handle the deployment.
    #[serde(
        rename = "service_connection",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_connection: Option<Option<uuid::Uuid>>,
    #[serde(rename = "service_connection_obj")]
    pub service_connection_obj: models::ServiceConnection,
    #[serde(rename = "refresh_interval_s")]
    pub refresh_interval_s: i32,
    /// Get Token identifier
    #[serde(rename = "token_identifier")]
    pub token_identifier: String,
    #[serde(rename = "config")]
    pub config: std::collections::HashMap<String, serde_json::Value>,
    /// Objects that are managed by authentik. These objects are created and updated automatically. This flag only indicates that an object can be overwritten by migrations. You can still modify the objects via the API, but expect changes to be overwritten in a later update.
    #[serde(
        rename = "managed",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub managed: Option<Option<String>>,
}

impl Outpost {
    /// Outpost Serializer
    pub fn new(
        pk: uuid::Uuid,
        name: String,
        r#type: models::OutpostTypeEnum,
        providers: Vec<i32>,
        providers_obj: Vec<models::Provider>,
        service_connection_obj: models::ServiceConnection,
        refresh_interval_s: i32,
        token_identifier: String,
        config: std::collections::HashMap<String, serde_json::Value>,
    ) -> Outpost {
        Outpost {
            pk,
            name,
            r#type,
            providers,
            providers_obj,
            service_connection: None,
            service_connection_obj,
            refresh_interval_s,
            token_identifier,
            config,
            managed: None,
        }
    }
}
