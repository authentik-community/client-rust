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

/// Brand : Brand Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Brand {
    #[serde(rename = "brand_uuid")]
    pub brand_uuid: uuid::Uuid,
    /// Domain that activates this brand. Can be a superset, i.e. `a.b` for `aa.b` and `ba.b`
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[serde(rename = "branding_title", skip_serializing_if = "Option::is_none")]
    pub branding_title: Option<String>,
    #[serde(rename = "branding_logo", skip_serializing_if = "Option::is_none")]
    pub branding_logo: Option<String>,
    #[serde(rename = "branding_favicon", skip_serializing_if = "Option::is_none")]
    pub branding_favicon: Option<String>,
    #[serde(rename = "branding_custom_css", skip_serializing_if = "Option::is_none")]
    pub branding_custom_css: Option<String>,
    #[serde(rename = "branding_default_flow_background", skip_serializing_if = "Option::is_none")]
    pub branding_default_flow_background: Option<String>,
    #[serde(
        rename = "flow_authentication",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub flow_authentication: Option<Option<uuid::Uuid>>,
    #[serde(
        rename = "flow_invalidation",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub flow_invalidation: Option<Option<uuid::Uuid>>,
    #[serde(
        rename = "flow_recovery",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub flow_recovery: Option<Option<uuid::Uuid>>,
    #[serde(
        rename = "flow_unenrollment",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub flow_unenrollment: Option<Option<uuid::Uuid>>,
    #[serde(
        rename = "flow_user_settings",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub flow_user_settings: Option<Option<uuid::Uuid>>,
    #[serde(
        rename = "flow_device_code",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub flow_device_code: Option<Option<uuid::Uuid>>,
    /// When set, external users will be redirected to this application after authenticating.
    #[serde(
        rename = "default_application",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_application: Option<Option<uuid::Uuid>>,
    /// Web Certificate used by the authentik Core webserver.
    #[serde(
        rename = "web_certificate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub web_certificate: Option<Option<uuid::Uuid>>,
    /// Certificates used for client authentication.
    #[serde(rename = "client_certificates", skip_serializing_if = "Option::is_none")]
    pub client_certificates: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl Brand {
    /// Brand Serializer
    pub fn new(brand_uuid: uuid::Uuid, domain: String) -> Brand {
        Brand {
            brand_uuid,
            domain,
            default: None,
            branding_title: None,
            branding_logo: None,
            branding_favicon: None,
            branding_custom_css: None,
            branding_default_flow_background: None,
            flow_authentication: None,
            flow_invalidation: None,
            flow_recovery: None,
            flow_unenrollment: None,
            flow_user_settings: None,
            flow_device_code: None,
            default_application: None,
            web_certificate: None,
            client_certificates: None,
            attributes: None,
        }
    }
}
