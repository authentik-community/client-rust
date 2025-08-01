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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AppEnum {
    #[serde(rename = "authentik.tenants")]
    AuthentikPeriodTenants,
    #[serde(rename = "authentik.tasks")]
    AuthentikPeriodTasks,
    #[serde(rename = "authentik.admin")]
    AuthentikPeriodAdmin,
    #[serde(rename = "authentik.api")]
    AuthentikPeriodApi,
    #[serde(rename = "authentik.crypto")]
    AuthentikPeriodCrypto,
    #[serde(rename = "authentik.events")]
    AuthentikPeriodEvents,
    #[serde(rename = "authentik.flows")]
    AuthentikPeriodFlows,
    #[serde(rename = "authentik.outposts")]
    AuthentikPeriodOutposts,
    #[serde(rename = "authentik.policies.dummy")]
    AuthentikPeriodPoliciesPeriodDummy,
    #[serde(rename = "authentik.policies.event_matcher")]
    AuthentikPeriodPoliciesPeriodEventMatcher,
    #[serde(rename = "authentik.policies.expiry")]
    AuthentikPeriodPoliciesPeriodExpiry,
    #[serde(rename = "authentik.policies.expression")]
    AuthentikPeriodPoliciesPeriodExpression,
    #[serde(rename = "authentik.policies.geoip")]
    AuthentikPeriodPoliciesPeriodGeoip,
    #[serde(rename = "authentik.policies.password")]
    AuthentikPeriodPoliciesPeriodPassword,
    #[serde(rename = "authentik.policies.reputation")]
    AuthentikPeriodPoliciesPeriodReputation,
    #[serde(rename = "authentik.policies")]
    AuthentikPeriodPolicies,
    #[serde(rename = "authentik.providers.ldap")]
    AuthentikPeriodProvidersPeriodLdap,
    #[serde(rename = "authentik.providers.oauth2")]
    AuthentikPeriodProvidersPeriodOauth2,
    #[serde(rename = "authentik.providers.proxy")]
    AuthentikPeriodProvidersPeriodProxy,
    #[serde(rename = "authentik.providers.rac")]
    AuthentikPeriodProvidersPeriodRac,
    #[serde(rename = "authentik.providers.radius")]
    AuthentikPeriodProvidersPeriodRadius,
    #[serde(rename = "authentik.providers.saml")]
    AuthentikPeriodProvidersPeriodSaml,
    #[serde(rename = "authentik.providers.scim")]
    AuthentikPeriodProvidersPeriodScim,
    #[serde(rename = "authentik.rbac")]
    AuthentikPeriodRbac,
    #[serde(rename = "authentik.recovery")]
    AuthentikPeriodRecovery,
    #[serde(rename = "authentik.sources.kerberos")]
    AuthentikPeriodSourcesPeriodKerberos,
    #[serde(rename = "authentik.sources.ldap")]
    AuthentikPeriodSourcesPeriodLdap,
    #[serde(rename = "authentik.sources.oauth")]
    AuthentikPeriodSourcesPeriodOauth,
    #[serde(rename = "authentik.sources.plex")]
    AuthentikPeriodSourcesPeriodPlex,
    #[serde(rename = "authentik.sources.saml")]
    AuthentikPeriodSourcesPeriodSaml,
    #[serde(rename = "authentik.sources.scim")]
    AuthentikPeriodSourcesPeriodScim,
    #[serde(rename = "authentik.stages.authenticator")]
    AuthentikPeriodStagesPeriodAuthenticator,
    #[serde(rename = "authentik.stages.authenticator_duo")]
    AuthentikPeriodStagesPeriodAuthenticatorDuo,
    #[serde(rename = "authentik.stages.authenticator_email")]
    AuthentikPeriodStagesPeriodAuthenticatorEmail,
    #[serde(rename = "authentik.stages.authenticator_sms")]
    AuthentikPeriodStagesPeriodAuthenticatorSms,
    #[serde(rename = "authentik.stages.authenticator_static")]
    AuthentikPeriodStagesPeriodAuthenticatorStatic,
    #[serde(rename = "authentik.stages.authenticator_totp")]
    AuthentikPeriodStagesPeriodAuthenticatorTotp,
    #[serde(rename = "authentik.stages.authenticator_validate")]
    AuthentikPeriodStagesPeriodAuthenticatorValidate,
    #[serde(rename = "authentik.stages.authenticator_webauthn")]
    AuthentikPeriodStagesPeriodAuthenticatorWebauthn,
    #[serde(rename = "authentik.stages.captcha")]
    AuthentikPeriodStagesPeriodCaptcha,
    #[serde(rename = "authentik.stages.consent")]
    AuthentikPeriodStagesPeriodConsent,
    #[serde(rename = "authentik.stages.deny")]
    AuthentikPeriodStagesPeriodDeny,
    #[serde(rename = "authentik.stages.dummy")]
    AuthentikPeriodStagesPeriodDummy,
    #[serde(rename = "authentik.stages.email")]
    AuthentikPeriodStagesPeriodEmail,
    #[serde(rename = "authentik.stages.identification")]
    AuthentikPeriodStagesPeriodIdentification,
    #[serde(rename = "authentik.stages.invitation")]
    AuthentikPeriodStagesPeriodInvitation,
    #[serde(rename = "authentik.stages.password")]
    AuthentikPeriodStagesPeriodPassword,
    #[serde(rename = "authentik.stages.prompt")]
    AuthentikPeriodStagesPeriodPrompt,
    #[serde(rename = "authentik.stages.redirect")]
    AuthentikPeriodStagesPeriodRedirect,
    #[serde(rename = "authentik.stages.user_delete")]
    AuthentikPeriodStagesPeriodUserDelete,
    #[serde(rename = "authentik.stages.user_login")]
    AuthentikPeriodStagesPeriodUserLogin,
    #[serde(rename = "authentik.stages.user_logout")]
    AuthentikPeriodStagesPeriodUserLogout,
    #[serde(rename = "authentik.stages.user_write")]
    AuthentikPeriodStagesPeriodUserWrite,
    #[serde(rename = "authentik.tasks.schedules")]
    AuthentikPeriodTasksPeriodSchedules,
    #[serde(rename = "authentik.brands")]
    AuthentikPeriodBrands,
    #[serde(rename = "authentik.blueprints")]
    AuthentikPeriodBlueprints,
    #[serde(rename = "authentik.core")]
    AuthentikPeriodCore,
    #[serde(rename = "authentik.enterprise")]
    AuthentikPeriodEnterprise,
    #[serde(rename = "authentik.enterprise.audit")]
    AuthentikPeriodEnterprisePeriodAudit,
    #[serde(rename = "authentik.enterprise.policies.unique_password")]
    AuthentikPeriodEnterprisePeriodPoliciesPeriodUniquePassword,
    #[serde(rename = "authentik.enterprise.providers.google_workspace")]
    AuthentikPeriodEnterprisePeriodProvidersPeriodGoogleWorkspace,
    #[serde(rename = "authentik.enterprise.providers.microsoft_entra")]
    AuthentikPeriodEnterprisePeriodProvidersPeriodMicrosoftEntra,
    #[serde(rename = "authentik.enterprise.providers.ssf")]
    AuthentikPeriodEnterprisePeriodProvidersPeriodSsf,
    #[serde(rename = "authentik.enterprise.search")]
    AuthentikPeriodEnterprisePeriodSearch,
    #[serde(rename = "authentik.enterprise.stages.authenticator_endpoint_gdtc")]
    AuthentikPeriodEnterprisePeriodStagesPeriodAuthenticatorEndpointGdtc,
    #[serde(rename = "authentik.enterprise.stages.mtls")]
    AuthentikPeriodEnterprisePeriodStagesPeriodMtls,
    #[serde(rename = "authentik.enterprise.stages.source")]
    AuthentikPeriodEnterprisePeriodStagesPeriodSource,
}

impl std::fmt::Display for AppEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AuthentikPeriodTenants => write!(f, "authentik.tenants"),
            Self::AuthentikPeriodTasks => write!(f, "authentik.tasks"),
            Self::AuthentikPeriodAdmin => write!(f, "authentik.admin"),
            Self::AuthentikPeriodApi => write!(f, "authentik.api"),
            Self::AuthentikPeriodCrypto => write!(f, "authentik.crypto"),
            Self::AuthentikPeriodEvents => write!(f, "authentik.events"),
            Self::AuthentikPeriodFlows => write!(f, "authentik.flows"),
            Self::AuthentikPeriodOutposts => write!(f, "authentik.outposts"),
            Self::AuthentikPeriodPoliciesPeriodDummy => write!(f, "authentik.policies.dummy"),
            Self::AuthentikPeriodPoliciesPeriodEventMatcher => write!(f, "authentik.policies.event_matcher"),
            Self::AuthentikPeriodPoliciesPeriodExpiry => write!(f, "authentik.policies.expiry"),
            Self::AuthentikPeriodPoliciesPeriodExpression => write!(f, "authentik.policies.expression"),
            Self::AuthentikPeriodPoliciesPeriodGeoip => write!(f, "authentik.policies.geoip"),
            Self::AuthentikPeriodPoliciesPeriodPassword => write!(f, "authentik.policies.password"),
            Self::AuthentikPeriodPoliciesPeriodReputation => write!(f, "authentik.policies.reputation"),
            Self::AuthentikPeriodPolicies => write!(f, "authentik.policies"),
            Self::AuthentikPeriodProvidersPeriodLdap => write!(f, "authentik.providers.ldap"),
            Self::AuthentikPeriodProvidersPeriodOauth2 => write!(f, "authentik.providers.oauth2"),
            Self::AuthentikPeriodProvidersPeriodProxy => write!(f, "authentik.providers.proxy"),
            Self::AuthentikPeriodProvidersPeriodRac => write!(f, "authentik.providers.rac"),
            Self::AuthentikPeriodProvidersPeriodRadius => write!(f, "authentik.providers.radius"),
            Self::AuthentikPeriodProvidersPeriodSaml => write!(f, "authentik.providers.saml"),
            Self::AuthentikPeriodProvidersPeriodScim => write!(f, "authentik.providers.scim"),
            Self::AuthentikPeriodRbac => write!(f, "authentik.rbac"),
            Self::AuthentikPeriodRecovery => write!(f, "authentik.recovery"),
            Self::AuthentikPeriodSourcesPeriodKerberos => write!(f, "authentik.sources.kerberos"),
            Self::AuthentikPeriodSourcesPeriodLdap => write!(f, "authentik.sources.ldap"),
            Self::AuthentikPeriodSourcesPeriodOauth => write!(f, "authentik.sources.oauth"),
            Self::AuthentikPeriodSourcesPeriodPlex => write!(f, "authentik.sources.plex"),
            Self::AuthentikPeriodSourcesPeriodSaml => write!(f, "authentik.sources.saml"),
            Self::AuthentikPeriodSourcesPeriodScim => write!(f, "authentik.sources.scim"),
            Self::AuthentikPeriodStagesPeriodAuthenticator => write!(f, "authentik.stages.authenticator"),
            Self::AuthentikPeriodStagesPeriodAuthenticatorDuo => write!(f, "authentik.stages.authenticator_duo"),
            Self::AuthentikPeriodStagesPeriodAuthenticatorEmail => write!(f, "authentik.stages.authenticator_email"),
            Self::AuthentikPeriodStagesPeriodAuthenticatorSms => write!(f, "authentik.stages.authenticator_sms"),
            Self::AuthentikPeriodStagesPeriodAuthenticatorStatic => write!(f, "authentik.stages.authenticator_static"),
            Self::AuthentikPeriodStagesPeriodAuthenticatorTotp => write!(f, "authentik.stages.authenticator_totp"),
            Self::AuthentikPeriodStagesPeriodAuthenticatorValidate => {
                write!(f, "authentik.stages.authenticator_validate")
            }
            Self::AuthentikPeriodStagesPeriodAuthenticatorWebauthn => {
                write!(f, "authentik.stages.authenticator_webauthn")
            }
            Self::AuthentikPeriodStagesPeriodCaptcha => write!(f, "authentik.stages.captcha"),
            Self::AuthentikPeriodStagesPeriodConsent => write!(f, "authentik.stages.consent"),
            Self::AuthentikPeriodStagesPeriodDeny => write!(f, "authentik.stages.deny"),
            Self::AuthentikPeriodStagesPeriodDummy => write!(f, "authentik.stages.dummy"),
            Self::AuthentikPeriodStagesPeriodEmail => write!(f, "authentik.stages.email"),
            Self::AuthentikPeriodStagesPeriodIdentification => write!(f, "authentik.stages.identification"),
            Self::AuthentikPeriodStagesPeriodInvitation => write!(f, "authentik.stages.invitation"),
            Self::AuthentikPeriodStagesPeriodPassword => write!(f, "authentik.stages.password"),
            Self::AuthentikPeriodStagesPeriodPrompt => write!(f, "authentik.stages.prompt"),
            Self::AuthentikPeriodStagesPeriodRedirect => write!(f, "authentik.stages.redirect"),
            Self::AuthentikPeriodStagesPeriodUserDelete => write!(f, "authentik.stages.user_delete"),
            Self::AuthentikPeriodStagesPeriodUserLogin => write!(f, "authentik.stages.user_login"),
            Self::AuthentikPeriodStagesPeriodUserLogout => write!(f, "authentik.stages.user_logout"),
            Self::AuthentikPeriodStagesPeriodUserWrite => write!(f, "authentik.stages.user_write"),
            Self::AuthentikPeriodTasksPeriodSchedules => write!(f, "authentik.tasks.schedules"),
            Self::AuthentikPeriodBrands => write!(f, "authentik.brands"),
            Self::AuthentikPeriodBlueprints => write!(f, "authentik.blueprints"),
            Self::AuthentikPeriodCore => write!(f, "authentik.core"),
            Self::AuthentikPeriodEnterprise => write!(f, "authentik.enterprise"),
            Self::AuthentikPeriodEnterprisePeriodAudit => write!(f, "authentik.enterprise.audit"),
            Self::AuthentikPeriodEnterprisePeriodPoliciesPeriodUniquePassword => {
                write!(f, "authentik.enterprise.policies.unique_password")
            }
            Self::AuthentikPeriodEnterprisePeriodProvidersPeriodGoogleWorkspace => {
                write!(f, "authentik.enterprise.providers.google_workspace")
            }
            Self::AuthentikPeriodEnterprisePeriodProvidersPeriodMicrosoftEntra => {
                write!(f, "authentik.enterprise.providers.microsoft_entra")
            }
            Self::AuthentikPeriodEnterprisePeriodProvidersPeriodSsf => write!(f, "authentik.enterprise.providers.ssf"),
            Self::AuthentikPeriodEnterprisePeriodSearch => write!(f, "authentik.enterprise.search"),
            Self::AuthentikPeriodEnterprisePeriodStagesPeriodAuthenticatorEndpointGdtc => {
                write!(f, "authentik.enterprise.stages.authenticator_endpoint_gdtc")
            }
            Self::AuthentikPeriodEnterprisePeriodStagesPeriodMtls => write!(f, "authentik.enterprise.stages.mtls"),
            Self::AuthentikPeriodEnterprisePeriodStagesPeriodSource => write!(f, "authentik.enterprise.stages.source"),
        }
    }
}

impl Default for AppEnum {
    fn default() -> AppEnum {
        Self::AuthentikPeriodTenants
    }
}
