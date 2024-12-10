/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.5
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// LdapSourcePropertyMappingRequest : LDAP PropertyMapping Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LdapSourcePropertyMappingRequest {
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
}

impl LdapSourcePropertyMappingRequest {
    /// LDAP PropertyMapping Serializer
    pub fn new(name: String, expression: String) -> LdapSourcePropertyMappingRequest {
        LdapSourcePropertyMappingRequest {
            managed: None,
            name,
            expression,
        }
    }
}
