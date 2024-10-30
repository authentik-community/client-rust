/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.0
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`crypto_certificatekeypairs_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CryptoCertificatekeypairsCreateError {
    Status400(models::ValidationError),
    Status403(models::GenericError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`crypto_certificatekeypairs_destroy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CryptoCertificatekeypairsDestroyError {
    Status400(models::ValidationError),
    Status403(models::GenericError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`crypto_certificatekeypairs_generate_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CryptoCertificatekeypairsGenerateCreateError {
    Status400(),
    Status403(models::GenericError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`crypto_certificatekeypairs_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CryptoCertificatekeypairsListError {
    Status400(models::ValidationError),
    Status403(models::GenericError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`crypto_certificatekeypairs_partial_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CryptoCertificatekeypairsPartialUpdateError {
    Status400(models::ValidationError),
    Status403(models::GenericError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`crypto_certificatekeypairs_retrieve`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CryptoCertificatekeypairsRetrieveError {
    Status400(models::ValidationError),
    Status403(models::GenericError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`crypto_certificatekeypairs_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CryptoCertificatekeypairsUpdateError {
    Status400(models::ValidationError),
    Status403(models::GenericError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`crypto_certificatekeypairs_used_by_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CryptoCertificatekeypairsUsedByListError {
    Status400(models::ValidationError),
    Status403(models::GenericError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`crypto_certificatekeypairs_view_certificate_retrieve`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CryptoCertificatekeypairsViewCertificateRetrieveError {
    Status400(models::ValidationError),
    Status403(models::GenericError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`crypto_certificatekeypairs_view_private_key_retrieve`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CryptoCertificatekeypairsViewPrivateKeyRetrieveError {
    Status400(models::ValidationError),
    Status403(models::GenericError),
    UnknownValue(serde_json::Value),
}

/// CertificateKeyPair Viewset
pub async fn crypto_certificatekeypairs_create(
    configuration: &configuration::Configuration,
    certificate_key_pair_request: models::CertificateKeyPairRequest,
) -> Result<models::CertificateKeyPair, Error<CryptoCertificatekeypairsCreateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/crypto/certificatekeypairs/", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&certificate_key_pair_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CryptoCertificatekeypairsCreateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// CertificateKeyPair Viewset
pub async fn crypto_certificatekeypairs_destroy(
    configuration: &configuration::Configuration,
    kp_uuid: &str,
) -> Result<(), Error<CryptoCertificatekeypairsDestroyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/crypto/certificatekeypairs/{kp_uuid}/",
        local_var_configuration.base_path,
        kp_uuid = crate::apis::urlencode(kp_uuid)
    );
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CryptoCertificatekeypairsDestroyError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Generate a new, self-signed certificate-key pair
pub async fn crypto_certificatekeypairs_generate_create(
    configuration: &configuration::Configuration,
    certificate_generation_request: models::CertificateGenerationRequest,
) -> Result<models::CertificateKeyPair, Error<CryptoCertificatekeypairsGenerateCreateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/crypto/certificatekeypairs/generate/",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&certificate_generation_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CryptoCertificatekeypairsGenerateCreateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// CertificateKeyPair Viewset
pub async fn crypto_certificatekeypairs_list(
    configuration: &configuration::Configuration,
    has_key: Option<bool>,
    include_details: Option<bool>,
    managed: Option<&str>,
    name: Option<&str>,
    ordering: Option<&str>,
    page: Option<i32>,
    page_size: Option<i32>,
    search: Option<&str>,
) -> Result<models::PaginatedCertificateKeyPairList, Error<CryptoCertificatekeypairsListError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/crypto/certificatekeypairs/", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = has_key {
        local_var_req_builder = local_var_req_builder.query(&[("has_key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include_details {
        local_var_req_builder = local_var_req_builder.query(&[("include_details", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = managed {
        local_var_req_builder = local_var_req_builder.query(&[("managed", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name {
        local_var_req_builder = local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = ordering {
        local_var_req_builder = local_var_req_builder.query(&[("ordering", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("page_size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = search {
        local_var_req_builder = local_var_req_builder.query(&[("search", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CryptoCertificatekeypairsListError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// CertificateKeyPair Viewset
pub async fn crypto_certificatekeypairs_partial_update(
    configuration: &configuration::Configuration,
    kp_uuid: &str,
    patched_certificate_key_pair_request: Option<models::PatchedCertificateKeyPairRequest>,
) -> Result<models::CertificateKeyPair, Error<CryptoCertificatekeypairsPartialUpdateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/crypto/certificatekeypairs/{kp_uuid}/",
        local_var_configuration.base_path,
        kp_uuid = crate::apis::urlencode(kp_uuid)
    );
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&patched_certificate_key_pair_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CryptoCertificatekeypairsPartialUpdateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// CertificateKeyPair Viewset
pub async fn crypto_certificatekeypairs_retrieve(
    configuration: &configuration::Configuration,
    kp_uuid: &str,
) -> Result<models::CertificateKeyPair, Error<CryptoCertificatekeypairsRetrieveError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/crypto/certificatekeypairs/{kp_uuid}/",
        local_var_configuration.base_path,
        kp_uuid = crate::apis::urlencode(kp_uuid)
    );
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CryptoCertificatekeypairsRetrieveError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// CertificateKeyPair Viewset
pub async fn crypto_certificatekeypairs_update(
    configuration: &configuration::Configuration,
    kp_uuid: &str,
    certificate_key_pair_request: models::CertificateKeyPairRequest,
) -> Result<models::CertificateKeyPair, Error<CryptoCertificatekeypairsUpdateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/crypto/certificatekeypairs/{kp_uuid}/",
        local_var_configuration.base_path,
        kp_uuid = crate::apis::urlencode(kp_uuid)
    );
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&certificate_key_pair_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CryptoCertificatekeypairsUpdateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a list of all objects that use this object
pub async fn crypto_certificatekeypairs_used_by_list(
    configuration: &configuration::Configuration,
    kp_uuid: &str,
) -> Result<Vec<models::UsedBy>, Error<CryptoCertificatekeypairsUsedByListError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/crypto/certificatekeypairs/{kp_uuid}/used_by/",
        local_var_configuration.base_path,
        kp_uuid = crate::apis::urlencode(kp_uuid)
    );
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CryptoCertificatekeypairsUsedByListError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Return certificate-key pairs certificate and log access
pub async fn crypto_certificatekeypairs_view_certificate_retrieve(
    configuration: &configuration::Configuration,
    kp_uuid: &str,
    download: Option<bool>,
) -> Result<models::CertificateData, Error<CryptoCertificatekeypairsViewCertificateRetrieveError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/crypto/certificatekeypairs/{kp_uuid}/view_certificate/",
        local_var_configuration.base_path,
        kp_uuid = crate::apis::urlencode(kp_uuid)
    );
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = download {
        local_var_req_builder = local_var_req_builder.query(&[("download", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CryptoCertificatekeypairsViewCertificateRetrieveError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Return certificate-key pairs private key and log access
pub async fn crypto_certificatekeypairs_view_private_key_retrieve(
    configuration: &configuration::Configuration,
    kp_uuid: &str,
    download: Option<bool>,
) -> Result<models::CertificateData, Error<CryptoCertificatekeypairsViewPrivateKeyRetrieveError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/crypto/certificatekeypairs/{kp_uuid}/view_private_key/",
        local_var_configuration.base_path,
        kp_uuid = crate::apis::urlencode(kp_uuid)
    );
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = download {
        local_var_req_builder = local_var_req_builder.query(&[("download", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CryptoCertificatekeypairsViewPrivateKeyRetrieveError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
