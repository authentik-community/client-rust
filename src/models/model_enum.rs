/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.6.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ModelEnum {
    #[serde(rename = "authentik_tenants.domain")]
    TenantsPeriodDomain,
    #[serde(rename = "authentik_crypto.certificatekeypair")]
    CryptoPeriodCertificatekeypair,
    #[serde(rename = "authentik_flows.flow")]
    FlowsPeriodFlow,
    #[serde(rename = "authentik_flows.flowstagebinding")]
    FlowsPeriodFlowstagebinding,
    #[serde(rename = "authentik_outposts.dockerserviceconnection")]
    OutpostsPeriodDockerserviceconnection,
    #[serde(rename = "authentik_outposts.kubernetesserviceconnection")]
    OutpostsPeriodKubernetesserviceconnection,
    #[serde(rename = "authentik_outposts.outpost")]
    OutpostsPeriodOutpost,
    #[serde(rename = "authentik_policies_dummy.dummypolicy")]
    PoliciesDummyPeriodDummypolicy,
    #[serde(rename = "authentik_policies_event_matcher.eventmatcherpolicy")]
    PoliciesEventMatcherPeriodEventmatcherpolicy,
    #[serde(rename = "authentik_policies_expiry.passwordexpirypolicy")]
    PoliciesExpiryPeriodPasswordexpirypolicy,
    #[serde(rename = "authentik_policies_expression.expressionpolicy")]
    PoliciesExpressionPeriodExpressionpolicy,
    #[serde(rename = "authentik_policies_geoip.geoippolicy")]
    PoliciesGeoipPeriodGeoippolicy,
    #[serde(rename = "authentik_policies_password.passwordpolicy")]
    PoliciesPasswordPeriodPasswordpolicy,
    #[serde(rename = "authentik_policies_reputation.reputationpolicy")]
    PoliciesReputationPeriodReputationpolicy,
    #[serde(rename = "authentik_policies.policybinding")]
    PoliciesPeriodPolicybinding,
    #[serde(rename = "authentik_providers_ldap.ldapprovider")]
    ProvidersLdapPeriodLdapprovider,
    #[serde(rename = "authentik_providers_oauth2.scopemapping")]
    ProvidersOauth2PeriodScopemapping,
    #[serde(rename = "authentik_providers_oauth2.oauth2provider")]
    ProvidersOauth2PeriodOauth2provider,
    #[serde(rename = "authentik_providers_proxy.proxyprovider")]
    ProvidersProxyPeriodProxyprovider,
    #[serde(rename = "authentik_providers_rac.racprovider")]
    ProvidersRacPeriodRacprovider,
    #[serde(rename = "authentik_providers_rac.endpoint")]
    ProvidersRacPeriodEndpoint,
    #[serde(rename = "authentik_providers_rac.racpropertymapping")]
    ProvidersRacPeriodRacpropertymapping,
    #[serde(rename = "authentik_providers_radius.radiusprovider")]
    ProvidersRadiusPeriodRadiusprovider,
    #[serde(rename = "authentik_providers_radius.radiusproviderpropertymapping")]
    ProvidersRadiusPeriodRadiusproviderpropertymapping,
    #[serde(rename = "authentik_providers_saml.samlprovider")]
    ProvidersSamlPeriodSamlprovider,
    #[serde(rename = "authentik_providers_saml.samlpropertymapping")]
    ProvidersSamlPeriodSamlpropertymapping,
    #[serde(rename = "authentik_providers_scim.scimprovider")]
    ProvidersScimPeriodScimprovider,
    #[serde(rename = "authentik_providers_scim.scimmapping")]
    ProvidersScimPeriodScimmapping,
    #[serde(rename = "authentik_rbac.role")]
    RbacPeriodRole,
    #[serde(rename = "authentik_rbac.initialpermissions")]
    RbacPeriodInitialpermissions,
    #[serde(rename = "authentik_sources_kerberos.kerberossource")]
    SourcesKerberosPeriodKerberossource,
    #[serde(rename = "authentik_sources_kerberos.kerberossourcepropertymapping")]
    SourcesKerberosPeriodKerberossourcepropertymapping,
    #[serde(rename = "authentik_sources_kerberos.userkerberossourceconnection")]
    SourcesKerberosPeriodUserkerberossourceconnection,
    #[serde(rename = "authentik_sources_kerberos.groupkerberossourceconnection")]
    SourcesKerberosPeriodGroupkerberossourceconnection,
    #[serde(rename = "authentik_sources_ldap.ldapsource")]
    SourcesLdapPeriodLdapsource,
    #[serde(rename = "authentik_sources_ldap.ldapsourcepropertymapping")]
    SourcesLdapPeriodLdapsourcepropertymapping,
    #[serde(rename = "authentik_sources_ldap.userldapsourceconnection")]
    SourcesLdapPeriodUserldapsourceconnection,
    #[serde(rename = "authentik_sources_ldap.groupldapsourceconnection")]
    SourcesLdapPeriodGroupldapsourceconnection,
    #[serde(rename = "authentik_sources_oauth.oauthsource")]
    SourcesOauthPeriodOauthsource,
    #[serde(rename = "authentik_sources_oauth.oauthsourcepropertymapping")]
    SourcesOauthPeriodOauthsourcepropertymapping,
    #[serde(rename = "authentik_sources_oauth.useroauthsourceconnection")]
    SourcesOauthPeriodUseroauthsourceconnection,
    #[serde(rename = "authentik_sources_oauth.groupoauthsourceconnection")]
    SourcesOauthPeriodGroupoauthsourceconnection,
    #[serde(rename = "authentik_sources_plex.plexsource")]
    SourcesPlexPeriodPlexsource,
    #[serde(rename = "authentik_sources_plex.plexsourcepropertymapping")]
    SourcesPlexPeriodPlexsourcepropertymapping,
    #[serde(rename = "authentik_sources_plex.userplexsourceconnection")]
    SourcesPlexPeriodUserplexsourceconnection,
    #[serde(rename = "authentik_sources_plex.groupplexsourceconnection")]
    SourcesPlexPeriodGroupplexsourceconnection,
    #[serde(rename = "authentik_sources_saml.samlsource")]
    SourcesSamlPeriodSamlsource,
    #[serde(rename = "authentik_sources_saml.samlsourcepropertymapping")]
    SourcesSamlPeriodSamlsourcepropertymapping,
    #[serde(rename = "authentik_sources_saml.usersamlsourceconnection")]
    SourcesSamlPeriodUsersamlsourceconnection,
    #[serde(rename = "authentik_sources_saml.groupsamlsourceconnection")]
    SourcesSamlPeriodGroupsamlsourceconnection,
    #[serde(rename = "authentik_sources_scim.scimsource")]
    SourcesScimPeriodScimsource,
    #[serde(rename = "authentik_sources_scim.scimsourcepropertymapping")]
    SourcesScimPeriodScimsourcepropertymapping,
    #[serde(rename = "authentik_stages_authenticator_duo.authenticatorduostage")]
    StagesAuthenticatorDuoPeriodAuthenticatorduostage,
    #[serde(rename = "authentik_stages_authenticator_duo.duodevice")]
    StagesAuthenticatorDuoPeriodDuodevice,
    #[serde(rename = "authentik_stages_authenticator_email.authenticatoremailstage")]
    StagesAuthenticatorEmailPeriodAuthenticatoremailstage,
    #[serde(rename = "authentik_stages_authenticator_email.emaildevice")]
    StagesAuthenticatorEmailPeriodEmaildevice,
    #[serde(rename = "authentik_stages_authenticator_sms.authenticatorsmsstage")]
    StagesAuthenticatorSmsPeriodAuthenticatorsmsstage,
    #[serde(rename = "authentik_stages_authenticator_sms.smsdevice")]
    StagesAuthenticatorSmsPeriodSmsdevice,
    #[serde(rename = "authentik_stages_authenticator_static.authenticatorstaticstage")]
    StagesAuthenticatorStaticPeriodAuthenticatorstaticstage,
    #[serde(rename = "authentik_stages_authenticator_static.staticdevice")]
    StagesAuthenticatorStaticPeriodStaticdevice,
    #[serde(rename = "authentik_stages_authenticator_totp.authenticatortotpstage")]
    StagesAuthenticatorTotpPeriodAuthenticatortotpstage,
    #[serde(rename = "authentik_stages_authenticator_totp.totpdevice")]
    StagesAuthenticatorTotpPeriodTotpdevice,
    #[serde(rename = "authentik_stages_authenticator_validate.authenticatorvalidatestage")]
    StagesAuthenticatorValidatePeriodAuthenticatorvalidatestage,
    #[serde(rename = "authentik_stages_authenticator_webauthn.authenticatorwebauthnstage")]
    StagesAuthenticatorWebauthnPeriodAuthenticatorwebauthnstage,
    #[serde(rename = "authentik_stages_authenticator_webauthn.webauthndevice")]
    StagesAuthenticatorWebauthnPeriodWebauthndevice,
    #[serde(rename = "authentik_stages_captcha.captchastage")]
    StagesCaptchaPeriodCaptchastage,
    #[serde(rename = "authentik_stages_consent.consentstage")]
    StagesConsentPeriodConsentstage,
    #[serde(rename = "authentik_stages_consent.userconsent")]
    StagesConsentPeriodUserconsent,
    #[serde(rename = "authentik_stages_deny.denystage")]
    StagesDenyPeriodDenystage,
    #[serde(rename = "authentik_stages_dummy.dummystage")]
    StagesDummyPeriodDummystage,
    #[serde(rename = "authentik_stages_email.emailstage")]
    StagesEmailPeriodEmailstage,
    #[serde(rename = "authentik_stages_identification.identificationstage")]
    StagesIdentificationPeriodIdentificationstage,
    #[serde(rename = "authentik_stages_invitation.invitationstage")]
    StagesInvitationPeriodInvitationstage,
    #[serde(rename = "authentik_stages_invitation.invitation")]
    StagesInvitationPeriodInvitation,
    #[serde(rename = "authentik_stages_password.passwordstage")]
    StagesPasswordPeriodPasswordstage,
    #[serde(rename = "authentik_stages_prompt.prompt")]
    StagesPromptPeriodPrompt,
    #[serde(rename = "authentik_stages_prompt.promptstage")]
    StagesPromptPeriodPromptstage,
    #[serde(rename = "authentik_stages_redirect.redirectstage")]
    StagesRedirectPeriodRedirectstage,
    #[serde(rename = "authentik_stages_user_delete.userdeletestage")]
    StagesUserDeletePeriodUserdeletestage,
    #[serde(rename = "authentik_stages_user_login.userloginstage")]
    StagesUserLoginPeriodUserloginstage,
    #[serde(rename = "authentik_stages_user_logout.userlogoutstage")]
    StagesUserLogoutPeriodUserlogoutstage,
    #[serde(rename = "authentik_stages_user_write.userwritestage")]
    StagesUserWritePeriodUserwritestage,
    #[serde(rename = "authentik_brands.brand")]
    BrandsPeriodBrand,
    #[serde(rename = "authentik_blueprints.blueprintinstance")]
    BlueprintsPeriodBlueprintinstance,
    #[serde(rename = "authentik_core.group")]
    CorePeriodGroup,
    #[serde(rename = "authentik_core.user")]
    CorePeriodUser,
    #[serde(rename = "authentik_core.application")]
    CorePeriodApplication,
    #[serde(rename = "authentik_core.applicationentitlement")]
    CorePeriodApplicationentitlement,
    #[serde(rename = "authentik_core.token")]
    CorePeriodToken,
    #[serde(rename = "authentik_enterprise.license")]
    EnterprisePeriodLicense,
    #[serde(rename = "authentik_policies_unique_password.uniquepasswordpolicy")]
    PoliciesUniquePasswordPeriodUniquepasswordpolicy,
    #[serde(rename = "authentik_providers_google_workspace.googleworkspaceprovider")]
    ProvidersGoogleWorkspacePeriodGoogleworkspaceprovider,
    #[serde(rename = "authentik_providers_google_workspace.googleworkspaceprovidermapping")]
    ProvidersGoogleWorkspacePeriodGoogleworkspaceprovidermapping,
    #[serde(rename = "authentik_providers_microsoft_entra.microsoftentraprovider")]
    ProvidersMicrosoftEntraPeriodMicrosoftentraprovider,
    #[serde(rename = "authentik_providers_microsoft_entra.microsoftentraprovidermapping")]
    ProvidersMicrosoftEntraPeriodMicrosoftentraprovidermapping,
    #[serde(rename = "authentik_providers_ssf.ssfprovider")]
    ProvidersSsfPeriodSsfprovider,
    #[serde(rename = "authentik_stages_authenticator_endpoint_gdtc.authenticatorendpointgdtcstage")]
    StagesAuthenticatorEndpointGdtcPeriodAuthenticatorendpointgdtcstage,
    #[serde(rename = "authentik_stages_mtls.mutualtlsstage")]
    StagesMtlsPeriodMutualtlsstage,
    #[serde(rename = "authentik_stages_source.sourcestage")]
    StagesSourcePeriodSourcestage,
    #[serde(rename = "authentik_events.event")]
    EventsPeriodEvent,
    #[serde(rename = "authentik_events.notificationtransport")]
    EventsPeriodNotificationtransport,
    #[serde(rename = "authentik_events.notification")]
    EventsPeriodNotification,
    #[serde(rename = "authentik_events.notificationrule")]
    EventsPeriodNotificationrule,
    #[serde(rename = "authentik_events.notificationwebhookmapping")]
    EventsPeriodNotificationwebhookmapping,
}

impl std::fmt::Display for ModelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::TenantsPeriodDomain => write!(f, "authentik_tenants.domain"),
            Self::CryptoPeriodCertificatekeypair => write!(f, "authentik_crypto.certificatekeypair"),
            Self::FlowsPeriodFlow => write!(f, "authentik_flows.flow"),
            Self::FlowsPeriodFlowstagebinding => write!(f, "authentik_flows.flowstagebinding"),
            Self::OutpostsPeriodDockerserviceconnection => write!(f, "authentik_outposts.dockerserviceconnection"),
            Self::OutpostsPeriodKubernetesserviceconnection => {
                write!(f, "authentik_outposts.kubernetesserviceconnection")
            }
            Self::OutpostsPeriodOutpost => write!(f, "authentik_outposts.outpost"),
            Self::PoliciesDummyPeriodDummypolicy => write!(f, "authentik_policies_dummy.dummypolicy"),
            Self::PoliciesEventMatcherPeriodEventmatcherpolicy => {
                write!(f, "authentik_policies_event_matcher.eventmatcherpolicy")
            }
            Self::PoliciesExpiryPeriodPasswordexpirypolicy => {
                write!(f, "authentik_policies_expiry.passwordexpirypolicy")
            }
            Self::PoliciesExpressionPeriodExpressionpolicy => {
                write!(f, "authentik_policies_expression.expressionpolicy")
            }
            Self::PoliciesGeoipPeriodGeoippolicy => write!(f, "authentik_policies_geoip.geoippolicy"),
            Self::PoliciesPasswordPeriodPasswordpolicy => write!(f, "authentik_policies_password.passwordpolicy"),
            Self::PoliciesReputationPeriodReputationpolicy => {
                write!(f, "authentik_policies_reputation.reputationpolicy")
            }
            Self::PoliciesPeriodPolicybinding => write!(f, "authentik_policies.policybinding"),
            Self::ProvidersLdapPeriodLdapprovider => write!(f, "authentik_providers_ldap.ldapprovider"),
            Self::ProvidersOauth2PeriodScopemapping => write!(f, "authentik_providers_oauth2.scopemapping"),
            Self::ProvidersOauth2PeriodOauth2provider => write!(f, "authentik_providers_oauth2.oauth2provider"),
            Self::ProvidersProxyPeriodProxyprovider => write!(f, "authentik_providers_proxy.proxyprovider"),
            Self::ProvidersRacPeriodRacprovider => write!(f, "authentik_providers_rac.racprovider"),
            Self::ProvidersRacPeriodEndpoint => write!(f, "authentik_providers_rac.endpoint"),
            Self::ProvidersRacPeriodRacpropertymapping => write!(f, "authentik_providers_rac.racpropertymapping"),
            Self::ProvidersRadiusPeriodRadiusprovider => write!(f, "authentik_providers_radius.radiusprovider"),
            Self::ProvidersRadiusPeriodRadiusproviderpropertymapping => {
                write!(f, "authentik_providers_radius.radiusproviderpropertymapping")
            }
            Self::ProvidersSamlPeriodSamlprovider => write!(f, "authentik_providers_saml.samlprovider"),
            Self::ProvidersSamlPeriodSamlpropertymapping => write!(f, "authentik_providers_saml.samlpropertymapping"),
            Self::ProvidersScimPeriodScimprovider => write!(f, "authentik_providers_scim.scimprovider"),
            Self::ProvidersScimPeriodScimmapping => write!(f, "authentik_providers_scim.scimmapping"),
            Self::RbacPeriodRole => write!(f, "authentik_rbac.role"),
            Self::RbacPeriodInitialpermissions => write!(f, "authentik_rbac.initialpermissions"),
            Self::SourcesKerberosPeriodKerberossource => write!(f, "authentik_sources_kerberos.kerberossource"),
            Self::SourcesKerberosPeriodKerberossourcepropertymapping => {
                write!(f, "authentik_sources_kerberos.kerberossourcepropertymapping")
            }
            Self::SourcesKerberosPeriodUserkerberossourceconnection => {
                write!(f, "authentik_sources_kerberos.userkerberossourceconnection")
            }
            Self::SourcesKerberosPeriodGroupkerberossourceconnection => {
                write!(f, "authentik_sources_kerberos.groupkerberossourceconnection")
            }
            Self::SourcesLdapPeriodLdapsource => write!(f, "authentik_sources_ldap.ldapsource"),
            Self::SourcesLdapPeriodLdapsourcepropertymapping => {
                write!(f, "authentik_sources_ldap.ldapsourcepropertymapping")
            }
            Self::SourcesLdapPeriodUserldapsourceconnection => {
                write!(f, "authentik_sources_ldap.userldapsourceconnection")
            }
            Self::SourcesLdapPeriodGroupldapsourceconnection => {
                write!(f, "authentik_sources_ldap.groupldapsourceconnection")
            }
            Self::SourcesOauthPeriodOauthsource => write!(f, "authentik_sources_oauth.oauthsource"),
            Self::SourcesOauthPeriodOauthsourcepropertymapping => {
                write!(f, "authentik_sources_oauth.oauthsourcepropertymapping")
            }
            Self::SourcesOauthPeriodUseroauthsourceconnection => {
                write!(f, "authentik_sources_oauth.useroauthsourceconnection")
            }
            Self::SourcesOauthPeriodGroupoauthsourceconnection => {
                write!(f, "authentik_sources_oauth.groupoauthsourceconnection")
            }
            Self::SourcesPlexPeriodPlexsource => write!(f, "authentik_sources_plex.plexsource"),
            Self::SourcesPlexPeriodPlexsourcepropertymapping => {
                write!(f, "authentik_sources_plex.plexsourcepropertymapping")
            }
            Self::SourcesPlexPeriodUserplexsourceconnection => {
                write!(f, "authentik_sources_plex.userplexsourceconnection")
            }
            Self::SourcesPlexPeriodGroupplexsourceconnection => {
                write!(f, "authentik_sources_plex.groupplexsourceconnection")
            }
            Self::SourcesSamlPeriodSamlsource => write!(f, "authentik_sources_saml.samlsource"),
            Self::SourcesSamlPeriodSamlsourcepropertymapping => {
                write!(f, "authentik_sources_saml.samlsourcepropertymapping")
            }
            Self::SourcesSamlPeriodUsersamlsourceconnection => {
                write!(f, "authentik_sources_saml.usersamlsourceconnection")
            }
            Self::SourcesSamlPeriodGroupsamlsourceconnection => {
                write!(f, "authentik_sources_saml.groupsamlsourceconnection")
            }
            Self::SourcesScimPeriodScimsource => write!(f, "authentik_sources_scim.scimsource"),
            Self::SourcesScimPeriodScimsourcepropertymapping => {
                write!(f, "authentik_sources_scim.scimsourcepropertymapping")
            }
            Self::StagesAuthenticatorDuoPeriodAuthenticatorduostage => {
                write!(f, "authentik_stages_authenticator_duo.authenticatorduostage")
            }
            Self::StagesAuthenticatorDuoPeriodDuodevice => write!(f, "authentik_stages_authenticator_duo.duodevice"),
            Self::StagesAuthenticatorEmailPeriodAuthenticatoremailstage => {
                write!(f, "authentik_stages_authenticator_email.authenticatoremailstage")
            }
            Self::StagesAuthenticatorEmailPeriodEmaildevice => {
                write!(f, "authentik_stages_authenticator_email.emaildevice")
            }
            Self::StagesAuthenticatorSmsPeriodAuthenticatorsmsstage => {
                write!(f, "authentik_stages_authenticator_sms.authenticatorsmsstage")
            }
            Self::StagesAuthenticatorSmsPeriodSmsdevice => write!(f, "authentik_stages_authenticator_sms.smsdevice"),
            Self::StagesAuthenticatorStaticPeriodAuthenticatorstaticstage => {
                write!(f, "authentik_stages_authenticator_static.authenticatorstaticstage")
            }
            Self::StagesAuthenticatorStaticPeriodStaticdevice => {
                write!(f, "authentik_stages_authenticator_static.staticdevice")
            }
            Self::StagesAuthenticatorTotpPeriodAuthenticatortotpstage => {
                write!(f, "authentik_stages_authenticator_totp.authenticatortotpstage")
            }
            Self::StagesAuthenticatorTotpPeriodTotpdevice => {
                write!(f, "authentik_stages_authenticator_totp.totpdevice")
            }
            Self::StagesAuthenticatorValidatePeriodAuthenticatorvalidatestage => {
                write!(f, "authentik_stages_authenticator_validate.authenticatorvalidatestage")
            }
            Self::StagesAuthenticatorWebauthnPeriodAuthenticatorwebauthnstage => {
                write!(f, "authentik_stages_authenticator_webauthn.authenticatorwebauthnstage")
            }
            Self::StagesAuthenticatorWebauthnPeriodWebauthndevice => {
                write!(f, "authentik_stages_authenticator_webauthn.webauthndevice")
            }
            Self::StagesCaptchaPeriodCaptchastage => write!(f, "authentik_stages_captcha.captchastage"),
            Self::StagesConsentPeriodConsentstage => write!(f, "authentik_stages_consent.consentstage"),
            Self::StagesConsentPeriodUserconsent => write!(f, "authentik_stages_consent.userconsent"),
            Self::StagesDenyPeriodDenystage => write!(f, "authentik_stages_deny.denystage"),
            Self::StagesDummyPeriodDummystage => write!(f, "authentik_stages_dummy.dummystage"),
            Self::StagesEmailPeriodEmailstage => write!(f, "authentik_stages_email.emailstage"),
            Self::StagesIdentificationPeriodIdentificationstage => {
                write!(f, "authentik_stages_identification.identificationstage")
            }
            Self::StagesInvitationPeriodInvitationstage => write!(f, "authentik_stages_invitation.invitationstage"),
            Self::StagesInvitationPeriodInvitation => write!(f, "authentik_stages_invitation.invitation"),
            Self::StagesPasswordPeriodPasswordstage => write!(f, "authentik_stages_password.passwordstage"),
            Self::StagesPromptPeriodPrompt => write!(f, "authentik_stages_prompt.prompt"),
            Self::StagesPromptPeriodPromptstage => write!(f, "authentik_stages_prompt.promptstage"),
            Self::StagesRedirectPeriodRedirectstage => write!(f, "authentik_stages_redirect.redirectstage"),
            Self::StagesUserDeletePeriodUserdeletestage => write!(f, "authentik_stages_user_delete.userdeletestage"),
            Self::StagesUserLoginPeriodUserloginstage => write!(f, "authentik_stages_user_login.userloginstage"),
            Self::StagesUserLogoutPeriodUserlogoutstage => write!(f, "authentik_stages_user_logout.userlogoutstage"),
            Self::StagesUserWritePeriodUserwritestage => write!(f, "authentik_stages_user_write.userwritestage"),
            Self::BrandsPeriodBrand => write!(f, "authentik_brands.brand"),
            Self::BlueprintsPeriodBlueprintinstance => write!(f, "authentik_blueprints.blueprintinstance"),
            Self::CorePeriodGroup => write!(f, "authentik_core.group"),
            Self::CorePeriodUser => write!(f, "authentik_core.user"),
            Self::CorePeriodApplication => write!(f, "authentik_core.application"),
            Self::CorePeriodApplicationentitlement => write!(f, "authentik_core.applicationentitlement"),
            Self::CorePeriodToken => write!(f, "authentik_core.token"),
            Self::EnterprisePeriodLicense => write!(f, "authentik_enterprise.license"),
            Self::PoliciesUniquePasswordPeriodUniquepasswordpolicy => {
                write!(f, "authentik_policies_unique_password.uniquepasswordpolicy")
            }
            Self::ProvidersGoogleWorkspacePeriodGoogleworkspaceprovider => {
                write!(f, "authentik_providers_google_workspace.googleworkspaceprovider")
            }
            Self::ProvidersGoogleWorkspacePeriodGoogleworkspaceprovidermapping => {
                write!(f, "authentik_providers_google_workspace.googleworkspaceprovidermapping")
            }
            Self::ProvidersMicrosoftEntraPeriodMicrosoftentraprovider => {
                write!(f, "authentik_providers_microsoft_entra.microsoftentraprovider")
            }
            Self::ProvidersMicrosoftEntraPeriodMicrosoftentraprovidermapping => {
                write!(f, "authentik_providers_microsoft_entra.microsoftentraprovidermapping")
            }
            Self::ProvidersSsfPeriodSsfprovider => write!(f, "authentik_providers_ssf.ssfprovider"),
            Self::StagesAuthenticatorEndpointGdtcPeriodAuthenticatorendpointgdtcstage => write!(
                f,
                "authentik_stages_authenticator_endpoint_gdtc.authenticatorendpointgdtcstage"
            ),
            Self::StagesMtlsPeriodMutualtlsstage => write!(f, "authentik_stages_mtls.mutualtlsstage"),
            Self::StagesSourcePeriodSourcestage => write!(f, "authentik_stages_source.sourcestage"),
            Self::EventsPeriodEvent => write!(f, "authentik_events.event"),
            Self::EventsPeriodNotificationtransport => write!(f, "authentik_events.notificationtransport"),
            Self::EventsPeriodNotification => write!(f, "authentik_events.notification"),
            Self::EventsPeriodNotificationrule => write!(f, "authentik_events.notificationrule"),
            Self::EventsPeriodNotificationwebhookmapping => write!(f, "authentik_events.notificationwebhookmapping"),
        }
    }
}

impl Default for ModelEnum {
    fn default() -> ModelEnum {
        Self::TenantsPeriodDomain
    }
}
