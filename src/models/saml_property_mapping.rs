/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.4
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// SamlPropertyMapping : SAMLPropertyMapping Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SamlPropertyMapping {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    /// Objects that are managed by authentik. These objects are created and updated automatically. This flag only indicates that an object can be overwritten by migrations. You can still modify the objects via the API, but expect changes to be overwritten in a later update.
    #[serde(
        rename = "managed",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub managed: Option<Option<String>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "expression")]
    pub expression: String,
    /// Get object's component so that we know how to edit the object
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
    #[serde(rename = "saml_name")]
    pub saml_name: String,
    #[serde(
        rename = "friendly_name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub friendly_name: Option<Option<String>>,
}

impl SamlPropertyMapping {
    /// SAMLPropertyMapping Serializer
    pub fn new(
        pk: uuid::Uuid,
        name: String,
        expression: String,
        component: String,
        verbose_name: String,
        verbose_name_plural: String,
        meta_model_name: String,
        saml_name: String,
    ) -> SamlPropertyMapping {
        SamlPropertyMapping {
            pk,
            managed: None,
            name,
            expression,
            component,
            verbose_name,
            verbose_name_plural,
            meta_model_name,
            saml_name,
            friendly_name: None,
        }
    }
}
