/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.2
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// SamlProvider : SAMLProvider Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SamlProvider {
    #[serde(rename = "pk")]
    pub pk: i32,
    #[serde(rename = "name")]
    pub name: String,
    /// Flow used for authentication when the associated application is accessed by an un-authenticated user.
    #[serde(
        rename = "authentication_flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub authentication_flow: Option<Option<uuid::Uuid>>,
    /// Flow used when authorizing this provider.
    #[serde(rename = "authorization_flow")]
    pub authorization_flow: uuid::Uuid,
    /// Flow used ending the session from a provider.
    #[serde(rename = "invalidation_flow")]
    pub invalidation_flow: uuid::Uuid,
    #[serde(rename = "property_mappings", skip_serializing_if = "Option::is_none")]
    pub property_mappings: Option<Vec<uuid::Uuid>>,
    /// Get object component so that we know how to edit the object
    #[serde(rename = "component")]
    pub component: String,
    /// Internal application name, used in URLs.
    #[serde(rename = "assigned_application_slug")]
    pub assigned_application_slug: String,
    /// Application's display Name.
    #[serde(rename = "assigned_application_name")]
    pub assigned_application_name: String,
    /// Internal application name, used in URLs.
    #[serde(rename = "assigned_backchannel_application_slug")]
    pub assigned_backchannel_application_slug: String,
    /// Application's display Name.
    #[serde(rename = "assigned_backchannel_application_name")]
    pub assigned_backchannel_application_name: String,
    /// Return object's verbose_name
    #[serde(rename = "verbose_name")]
    pub verbose_name: String,
    /// Return object's plural verbose_name
    #[serde(rename = "verbose_name_plural")]
    pub verbose_name_plural: String,
    /// Return internal model name
    #[serde(rename = "meta_model_name")]
    pub meta_model_name: String,
    #[serde(rename = "acs_url")]
    pub acs_url: String,
    /// Value of the audience restriction field of the assertion. When left empty, no audience restriction will be added.
    #[serde(rename = "audience", skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    /// Also known as EntityID
    #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// Assertion valid not before current time + this value (Format: hours=-1;minutes=-2;seconds=-3).
    #[serde(rename = "assertion_valid_not_before", skip_serializing_if = "Option::is_none")]
    pub assertion_valid_not_before: Option<String>,
    /// Assertion not valid on or after current time + this value (Format: hours=1;minutes=2;seconds=3).
    #[serde(rename = "assertion_valid_not_on_or_after", skip_serializing_if = "Option::is_none")]
    pub assertion_valid_not_on_or_after: Option<String>,
    /// Session not valid on or after current time + this value (Format: hours=1;minutes=2;seconds=3).
    #[serde(rename = "session_valid_not_on_or_after", skip_serializing_if = "Option::is_none")]
    pub session_valid_not_on_or_after: Option<String>,
    /// Configure how the NameID value will be created. When left empty, the NameIDPolicy of the incoming request will be considered
    #[serde(
        rename = "name_id_mapping",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub name_id_mapping: Option<Option<uuid::Uuid>>,
    #[serde(rename = "digest_algorithm", skip_serializing_if = "Option::is_none")]
    pub digest_algorithm: Option<models::DigestAlgorithmEnum>,
    #[serde(rename = "signature_algorithm", skip_serializing_if = "Option::is_none")]
    pub signature_algorithm: Option<models::SignatureAlgorithmEnum>,
    /// Keypair used to sign outgoing Responses going to the Service Provider.
    #[serde(
        rename = "signing_kp",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub signing_kp: Option<Option<uuid::Uuid>>,
    /// When selected, incoming assertion's Signatures will be validated against this certificate. To allow unsigned Requests, leave on default.
    #[serde(
        rename = "verification_kp",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub verification_kp: Option<Option<uuid::Uuid>>,
    /// When selected, incoming assertions are encrypted by the IdP using the public key of the encryption keypair. The assertion is decrypted by the SP using the the private key.
    #[serde(
        rename = "encryption_kp",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub encryption_kp: Option<Option<uuid::Uuid>>,
    #[serde(rename = "sign_assertion", skip_serializing_if = "Option::is_none")]
    pub sign_assertion: Option<bool>,
    #[serde(rename = "sign_response", skip_serializing_if = "Option::is_none")]
    pub sign_response: Option<bool>,
    /// This determines how authentik sends the response back to the Service Provider.
    #[serde(rename = "sp_binding", skip_serializing_if = "Option::is_none")]
    pub sp_binding: Option<models::SpBindingEnum>,
    /// Default relay_state value for IDP-initiated logins
    #[serde(rename = "default_relay_state", skip_serializing_if = "Option::is_none")]
    pub default_relay_state: Option<String>,
    /// Get metadata download URL
    #[serde(rename = "url_download_metadata")]
    pub url_download_metadata: String,
    /// Get SSO Post URL
    #[serde(rename = "url_sso_post")]
    pub url_sso_post: String,
    /// Get SSO Redirect URL
    #[serde(rename = "url_sso_redirect")]
    pub url_sso_redirect: String,
    /// Get SSO IDP-Initiated URL
    #[serde(rename = "url_sso_init")]
    pub url_sso_init: String,
    /// Get SLO POST URL
    #[serde(rename = "url_slo_post")]
    pub url_slo_post: String,
    /// Get SLO redirect URL
    #[serde(rename = "url_slo_redirect")]
    pub url_slo_redirect: String,
}

impl SamlProvider {
    /// SAMLProvider Serializer
    pub fn new(
        pk: i32,
        name: String,
        authorization_flow: uuid::Uuid,
        invalidation_flow: uuid::Uuid,
        component: String,
        assigned_application_slug: String,
        assigned_application_name: String,
        assigned_backchannel_application_slug: String,
        assigned_backchannel_application_name: String,
        verbose_name: String,
        verbose_name_plural: String,
        meta_model_name: String,
        acs_url: String,
        url_download_metadata: String,
        url_sso_post: String,
        url_sso_redirect: String,
        url_sso_init: String,
        url_slo_post: String,
        url_slo_redirect: String,
    ) -> SamlProvider {
        SamlProvider {
            pk,
            name,
            authentication_flow: None,
            authorization_flow,
            invalidation_flow,
            property_mappings: None,
            component,
            assigned_application_slug,
            assigned_application_name,
            assigned_backchannel_application_slug,
            assigned_backchannel_application_name,
            verbose_name,
            verbose_name_plural,
            meta_model_name,
            acs_url,
            audience: None,
            issuer: None,
            assertion_valid_not_before: None,
            assertion_valid_not_on_or_after: None,
            session_valid_not_on_or_after: None,
            name_id_mapping: None,
            digest_algorithm: None,
            signature_algorithm: None,
            signing_kp: None,
            verification_kp: None,
            encryption_kp: None,
            sign_assertion: None,
            sign_response: None,
            sp_binding: None,
            default_relay_state: None,
            url_download_metadata,
            url_sso_post,
            url_sso_redirect,
            url_sso_init,
            url_slo_post,
            url_slo_redirect,
        }
    }
}
