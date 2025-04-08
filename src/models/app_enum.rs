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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AppEnum {
    #[serde(rename = "authentik.tenants")]
    Tenants,
    #[serde(rename = "authentik.admin")]
    Admin,
    #[serde(rename = "authentik.api")]
    Api,
    #[serde(rename = "authentik.crypto")]
    Crypto,
    #[serde(rename = "authentik.flows")]
    Flows,
    #[serde(rename = "authentik.outposts")]
    Outposts,
    #[serde(rename = "authentik.policies.dummy")]
    PoliciesPeriodDummy,
    #[serde(rename = "authentik.policies.event_matcher")]
    PoliciesPeriodEventMatcher,
    #[serde(rename = "authentik.policies.expiry")]
    PoliciesPeriodExpiry,
    #[serde(rename = "authentik.policies.expression")]
    PoliciesPeriodExpression,
    #[serde(rename = "authentik.policies.geoip")]
    PoliciesPeriodGeoip,
    #[serde(rename = "authentik.policies.password")]
    PoliciesPeriodPassword,
    #[serde(rename = "authentik.policies.reputation")]
    PoliciesPeriodReputation,
    #[serde(rename = "authentik.policies")]
    Policies,
    #[serde(rename = "authentik.providers.ldap")]
    ProvidersPeriodLdap,
    #[serde(rename = "authentik.providers.oauth2")]
    ProvidersPeriodOauth2,
    #[serde(rename = "authentik.providers.proxy")]
    ProvidersPeriodProxy,
    #[serde(rename = "authentik.providers.rac")]
    ProvidersPeriodRac,
    #[serde(rename = "authentik.providers.radius")]
    ProvidersPeriodRadius,
    #[serde(rename = "authentik.providers.saml")]
    ProvidersPeriodSaml,
    #[serde(rename = "authentik.providers.scim")]
    ProvidersPeriodScim,
    #[serde(rename = "authentik.rbac")]
    Rbac,
    #[serde(rename = "authentik.recovery")]
    Recovery,
    #[serde(rename = "authentik.sources.kerberos")]
    SourcesPeriodKerberos,
    #[serde(rename = "authentik.sources.ldap")]
    SourcesPeriodLdap,
    #[serde(rename = "authentik.sources.oauth")]
    SourcesPeriodOauth,
    #[serde(rename = "authentik.sources.plex")]
    SourcesPeriodPlex,
    #[serde(rename = "authentik.sources.saml")]
    SourcesPeriodSaml,
    #[serde(rename = "authentik.sources.scim")]
    SourcesPeriodScim,
    #[serde(rename = "authentik.stages.authenticator")]
    StagesPeriodAuthenticator,
    #[serde(rename = "authentik.stages.authenticator_duo")]
    StagesPeriodAuthenticatorDuo,
    #[serde(rename = "authentik.stages.authenticator_email")]
    StagesPeriodAuthenticatorEmail,
    #[serde(rename = "authentik.stages.authenticator_sms")]
    StagesPeriodAuthenticatorSms,
    #[serde(rename = "authentik.stages.authenticator_static")]
    StagesPeriodAuthenticatorStatic,
    #[serde(rename = "authentik.stages.authenticator_totp")]
    StagesPeriodAuthenticatorTotp,
    #[serde(rename = "authentik.stages.authenticator_validate")]
    StagesPeriodAuthenticatorValidate,
    #[serde(rename = "authentik.stages.authenticator_webauthn")]
    StagesPeriodAuthenticatorWebauthn,
    #[serde(rename = "authentik.stages.captcha")]
    StagesPeriodCaptcha,
    #[serde(rename = "authentik.stages.consent")]
    StagesPeriodConsent,
    #[serde(rename = "authentik.stages.deny")]
    StagesPeriodDeny,
    #[serde(rename = "authentik.stages.dummy")]
    StagesPeriodDummy,
    #[serde(rename = "authentik.stages.email")]
    StagesPeriodEmail,
    #[serde(rename = "authentik.stages.identification")]
    StagesPeriodIdentification,
    #[serde(rename = "authentik.stages.invitation")]
    StagesPeriodInvitation,
    #[serde(rename = "authentik.stages.password")]
    StagesPeriodPassword,
    #[serde(rename = "authentik.stages.prompt")]
    StagesPeriodPrompt,
    #[serde(rename = "authentik.stages.redirect")]
    StagesPeriodRedirect,
    #[serde(rename = "authentik.stages.user_delete")]
    StagesPeriodUserDelete,
    #[serde(rename = "authentik.stages.user_login")]
    StagesPeriodUserLogin,
    #[serde(rename = "authentik.stages.user_logout")]
    StagesPeriodUserLogout,
    #[serde(rename = "authentik.stages.user_write")]
    StagesPeriodUserWrite,
    #[serde(rename = "authentik.brands")]
    Brands,
    #[serde(rename = "authentik.blueprints")]
    Blueprints,
    #[serde(rename = "authentik.core")]
    Core,
    #[serde(rename = "authentik.enterprise")]
    Enterprise,
    #[serde(rename = "authentik.enterprise.audit")]
    EnterprisePeriodAudit,
    #[serde(rename = "authentik.enterprise.providers.google_workspace")]
    EnterprisePeriodProvidersPeriodGoogleWorkspace,
    #[serde(rename = "authentik.enterprise.providers.microsoft_entra")]
    EnterprisePeriodProvidersPeriodMicrosoftEntra,
    #[serde(rename = "authentik.enterprise.providers.ssf")]
    EnterprisePeriodProvidersPeriodSsf,
    #[serde(rename = "authentik.enterprise.stages.authenticator_endpoint_gdtc")]
    EnterprisePeriodStagesPeriodAuthenticatorEndpointGdtc,
    #[serde(rename = "authentik.enterprise.stages.source")]
    EnterprisePeriodStagesPeriodSource,
    #[serde(rename = "authentik.events")]
    Events,
}

impl std::fmt::Display for AppEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Tenants => write!(f, "authentik.tenants"),
            Self::Admin => write!(f, "authentik.admin"),
            Self::Api => write!(f, "authentik.api"),
            Self::Crypto => write!(f, "authentik.crypto"),
            Self::Flows => write!(f, "authentik.flows"),
            Self::Outposts => write!(f, "authentik.outposts"),
            Self::PoliciesPeriodDummy => write!(f, "authentik.policies.dummy"),
            Self::PoliciesPeriodEventMatcher => write!(f, "authentik.policies.event_matcher"),
            Self::PoliciesPeriodExpiry => write!(f, "authentik.policies.expiry"),
            Self::PoliciesPeriodExpression => write!(f, "authentik.policies.expression"),
            Self::PoliciesPeriodGeoip => write!(f, "authentik.policies.geoip"),
            Self::PoliciesPeriodPassword => write!(f, "authentik.policies.password"),
            Self::PoliciesPeriodReputation => write!(f, "authentik.policies.reputation"),
            Self::Policies => write!(f, "authentik.policies"),
            Self::ProvidersPeriodLdap => write!(f, "authentik.providers.ldap"),
            Self::ProvidersPeriodOauth2 => write!(f, "authentik.providers.oauth2"),
            Self::ProvidersPeriodProxy => write!(f, "authentik.providers.proxy"),
            Self::ProvidersPeriodRac => write!(f, "authentik.providers.rac"),
            Self::ProvidersPeriodRadius => write!(f, "authentik.providers.radius"),
            Self::ProvidersPeriodSaml => write!(f, "authentik.providers.saml"),
            Self::ProvidersPeriodScim => write!(f, "authentik.providers.scim"),
            Self::Rbac => write!(f, "authentik.rbac"),
            Self::Recovery => write!(f, "authentik.recovery"),
            Self::SourcesPeriodKerberos => write!(f, "authentik.sources.kerberos"),
            Self::SourcesPeriodLdap => write!(f, "authentik.sources.ldap"),
            Self::SourcesPeriodOauth => write!(f, "authentik.sources.oauth"),
            Self::SourcesPeriodPlex => write!(f, "authentik.sources.plex"),
            Self::SourcesPeriodSaml => write!(f, "authentik.sources.saml"),
            Self::SourcesPeriodScim => write!(f, "authentik.sources.scim"),
            Self::StagesPeriodAuthenticator => write!(f, "authentik.stages.authenticator"),
            Self::StagesPeriodAuthenticatorDuo => write!(f, "authentik.stages.authenticator_duo"),
            Self::StagesPeriodAuthenticatorEmail => write!(f, "authentik.stages.authenticator_email"),
            Self::StagesPeriodAuthenticatorSms => write!(f, "authentik.stages.authenticator_sms"),
            Self::StagesPeriodAuthenticatorStatic => write!(f, "authentik.stages.authenticator_static"),
            Self::StagesPeriodAuthenticatorTotp => write!(f, "authentik.stages.authenticator_totp"),
            Self::StagesPeriodAuthenticatorValidate => write!(f, "authentik.stages.authenticator_validate"),
            Self::StagesPeriodAuthenticatorWebauthn => write!(f, "authentik.stages.authenticator_webauthn"),
            Self::StagesPeriodCaptcha => write!(f, "authentik.stages.captcha"),
            Self::StagesPeriodConsent => write!(f, "authentik.stages.consent"),
            Self::StagesPeriodDeny => write!(f, "authentik.stages.deny"),
            Self::StagesPeriodDummy => write!(f, "authentik.stages.dummy"),
            Self::StagesPeriodEmail => write!(f, "authentik.stages.email"),
            Self::StagesPeriodIdentification => write!(f, "authentik.stages.identification"),
            Self::StagesPeriodInvitation => write!(f, "authentik.stages.invitation"),
            Self::StagesPeriodPassword => write!(f, "authentik.stages.password"),
            Self::StagesPeriodPrompt => write!(f, "authentik.stages.prompt"),
            Self::StagesPeriodRedirect => write!(f, "authentik.stages.redirect"),
            Self::StagesPeriodUserDelete => write!(f, "authentik.stages.user_delete"),
            Self::StagesPeriodUserLogin => write!(f, "authentik.stages.user_login"),
            Self::StagesPeriodUserLogout => write!(f, "authentik.stages.user_logout"),
            Self::StagesPeriodUserWrite => write!(f, "authentik.stages.user_write"),
            Self::Brands => write!(f, "authentik.brands"),
            Self::Blueprints => write!(f, "authentik.blueprints"),
            Self::Core => write!(f, "authentik.core"),
            Self::Enterprise => write!(f, "authentik.enterprise"),
            Self::EnterprisePeriodAudit => write!(f, "authentik.enterprise.audit"),
            Self::EnterprisePeriodProvidersPeriodGoogleWorkspace => {
                write!(f, "authentik.enterprise.providers.google_workspace")
            }
            Self::EnterprisePeriodProvidersPeriodMicrosoftEntra => {
                write!(f, "authentik.enterprise.providers.microsoft_entra")
            }
            Self::EnterprisePeriodProvidersPeriodSsf => write!(f, "authentik.enterprise.providers.ssf"),
            Self::EnterprisePeriodStagesPeriodAuthenticatorEndpointGdtc => {
                write!(f, "authentik.enterprise.stages.authenticator_endpoint_gdtc")
            }
            Self::EnterprisePeriodStagesPeriodSource => write!(f, "authentik.enterprise.stages.source"),
            Self::Events => write!(f, "authentik.events"),
        }
    }
}

impl Default for AppEnum {
    fn default() -> AppEnum {
        Self::Tenants
    }
}
