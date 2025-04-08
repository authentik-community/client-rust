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
pub enum EventActions {
    #[serde(rename = "login")]
    Login,
    #[serde(rename = "login_failed")]
    LoginFailed,
    #[serde(rename = "logout")]
    Logout,
    #[serde(rename = "user_write")]
    UserWrite,
    #[serde(rename = "suspicious_request")]
    SuspiciousRequest,
    #[serde(rename = "password_set")]
    PasswordSet,
    #[serde(rename = "secret_view")]
    SecretView,
    #[serde(rename = "secret_rotate")]
    SecretRotate,
    #[serde(rename = "invitation_used")]
    InvitationUsed,
    #[serde(rename = "authorize_application")]
    AuthorizeApplication,
    #[serde(rename = "source_linked")]
    SourceLinked,
    #[serde(rename = "impersonation_started")]
    ImpersonationStarted,
    #[serde(rename = "impersonation_ended")]
    ImpersonationEnded,
    #[serde(rename = "flow_execution")]
    FlowExecution,
    #[serde(rename = "policy_execution")]
    PolicyExecution,
    #[serde(rename = "policy_exception")]
    PolicyException,
    #[serde(rename = "property_mapping_exception")]
    PropertyMappingException,
    #[serde(rename = "system_task_execution")]
    SystemTaskExecution,
    #[serde(rename = "system_task_exception")]
    SystemTaskException,
    #[serde(rename = "system_exception")]
    SystemException,
    #[serde(rename = "configuration_error")]
    ConfigurationError,
    #[serde(rename = "model_created")]
    ModelCreated,
    #[serde(rename = "model_updated")]
    ModelUpdated,
    #[serde(rename = "model_deleted")]
    ModelDeleted,
    #[serde(rename = "email_sent")]
    EmailSent,
    #[serde(rename = "update_available")]
    UpdateAvailable,
    #[serde(rename = "custom_")]
    Custom,
}

impl std::fmt::Display for EventActions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Login => write!(f, "login"),
            Self::LoginFailed => write!(f, "login_failed"),
            Self::Logout => write!(f, "logout"),
            Self::UserWrite => write!(f, "user_write"),
            Self::SuspiciousRequest => write!(f, "suspicious_request"),
            Self::PasswordSet => write!(f, "password_set"),
            Self::SecretView => write!(f, "secret_view"),
            Self::SecretRotate => write!(f, "secret_rotate"),
            Self::InvitationUsed => write!(f, "invitation_used"),
            Self::AuthorizeApplication => write!(f, "authorize_application"),
            Self::SourceLinked => write!(f, "source_linked"),
            Self::ImpersonationStarted => write!(f, "impersonation_started"),
            Self::ImpersonationEnded => write!(f, "impersonation_ended"),
            Self::FlowExecution => write!(f, "flow_execution"),
            Self::PolicyExecution => write!(f, "policy_execution"),
            Self::PolicyException => write!(f, "policy_exception"),
            Self::PropertyMappingException => write!(f, "property_mapping_exception"),
            Self::SystemTaskExecution => write!(f, "system_task_execution"),
            Self::SystemTaskException => write!(f, "system_task_exception"),
            Self::SystemException => write!(f, "system_exception"),
            Self::ConfigurationError => write!(f, "configuration_error"),
            Self::ModelCreated => write!(f, "model_created"),
            Self::ModelUpdated => write!(f, "model_updated"),
            Self::ModelDeleted => write!(f, "model_deleted"),
            Self::EmailSent => write!(f, "email_sent"),
            Self::UpdateAvailable => write!(f, "update_available"),
            Self::Custom => write!(f, "custom_"),
        }
    }
}

impl Default for EventActions {
    fn default() -> EventActions {
        Self::Login
    }
}
