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

/// KubernetesServiceConnection : KubernetesServiceConnection Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubernetesServiceConnection {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    /// If enabled, use the local connection. Required Docker socket/Kubernetes Integration
    #[serde(rename = "local", skip_serializing_if = "Option::is_none")]
    pub local: Option<bool>,
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
    /// Paste your kubeconfig here. authentik will automatically use the currently selected context.
    #[serde(rename = "kubeconfig", skip_serializing_if = "Option::is_none")]
    pub kubeconfig: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Verify SSL Certificates of the Kubernetes API endpoint
    #[serde(rename = "verify_ssl", skip_serializing_if = "Option::is_none")]
    pub verify_ssl: Option<bool>,
}

impl KubernetesServiceConnection {
    /// KubernetesServiceConnection Serializer
    pub fn new(
        pk: uuid::Uuid,
        name: String,
        component: String,
        verbose_name: String,
        verbose_name_plural: String,
        meta_model_name: String,
    ) -> KubernetesServiceConnection {
        KubernetesServiceConnection {
            pk,
            name,
            local: None,
            component,
            verbose_name,
            verbose_name_plural,
            meta_model_name,
            kubeconfig: None,
            verify_ssl: None,
        }
    }
}
