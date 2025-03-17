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

/// SamlSource : SAMLSource Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SamlSource {
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
    /// Flow used before authentication.
    #[serde(rename = "pre_authentication_flow")]
    pub pre_authentication_flow: uuid::Uuid,
    /// Also known as Entity ID. Defaults the Metadata URL.
    #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// URL that the initial Login request is sent to.
    #[serde(rename = "sso_url")]
    pub sso_url: String,
    /// Optional URL if your IDP supports Single-Logout.
    #[serde(
        rename = "slo_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub slo_url: Option<Option<String>>,
    /// Allows authentication flows initiated by the IdP. This can be a security risk, as no validation of the request ID is done.
    #[serde(rename = "allow_idp_initiated", skip_serializing_if = "Option::is_none")]
    pub allow_idp_initiated: Option<bool>,
    /// NameID Policy sent to the IdP. Can be unset, in which case no Policy is sent.
    #[serde(rename = "name_id_policy", skip_serializing_if = "Option::is_none")]
    pub name_id_policy: Option<models::NameIdPolicyEnum>,
    #[serde(rename = "binding_type", skip_serializing_if = "Option::is_none")]
    pub binding_type: Option<models::BindingTypeEnum>,
    /// When selected, incoming assertion's Signatures will be validated against this certificate. To allow unsigned Requests, leave on default.
    #[serde(
        rename = "verification_kp",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub verification_kp: Option<Option<uuid::Uuid>>,
    /// Keypair used to sign outgoing Responses going to the Identity Provider.
    #[serde(
        rename = "signing_kp",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub signing_kp: Option<Option<uuid::Uuid>>,
    #[serde(rename = "digest_algorithm", skip_serializing_if = "Option::is_none")]
    pub digest_algorithm: Option<models::DigestAlgorithmEnum>,
    #[serde(rename = "signature_algorithm", skip_serializing_if = "Option::is_none")]
    pub signature_algorithm: Option<models::SignatureAlgorithmEnum>,
    /// Time offset when temporary users should be deleted. This only applies if your IDP uses the NameID Format 'transient', and the user doesn't log out manually. (Format: hours=1;minutes=2;seconds=3).
    #[serde(rename = "temporary_user_delete_after", skip_serializing_if = "Option::is_none")]
    pub temporary_user_delete_after: Option<String>,
    /// When selected, incoming assertions are encrypted by the IdP using the public key of the encryption keypair. The assertion is decrypted by the SP using the the private key.
    #[serde(
        rename = "encryption_kp",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub encryption_kp: Option<Option<uuid::Uuid>>,
}

impl SamlSource {
    /// SAMLSource Serializer
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
        pre_authentication_flow: uuid::Uuid,
        sso_url: String,
    ) -> SamlSource {
        SamlSource {
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
            pre_authentication_flow,
            issuer: None,
            sso_url,
            slo_url: None,
            allow_idp_initiated: None,
            name_id_policy: None,
            binding_type: None,
            verification_kp: None,
            signing_kp: None,
            digest_algorithm: None,
            signature_algorithm: None,
            temporary_user_delete_after: None,
            encryption_kp: None,
        }
    }
}
