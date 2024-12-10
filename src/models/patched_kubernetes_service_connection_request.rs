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

/// PatchedKubernetesServiceConnectionRequest : KubernetesServiceConnection Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedKubernetesServiceConnectionRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// If enabled, use the local connection. Required Docker socket/Kubernetes Integration
    #[serde(rename = "local", skip_serializing_if = "Option::is_none")]
    pub local: Option<bool>,
    /// Paste your kubeconfig here. authentik will automatically use the currently selected context.
    #[serde(
        rename = "kubeconfig",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub kubeconfig: Option<Option<serde_json::Value>>,
    /// Verify SSL Certificates of the Kubernetes API endpoint
    #[serde(rename = "verify_ssl", skip_serializing_if = "Option::is_none")]
    pub verify_ssl: Option<bool>,
}

impl PatchedKubernetesServiceConnectionRequest {
    /// KubernetesServiceConnection Serializer
    pub fn new() -> PatchedKubernetesServiceConnectionRequest {
        PatchedKubernetesServiceConnectionRequest {
            name: None,
            local: None,
            kubeconfig: None,
            verify_ssl: None,
        }
    }
}
