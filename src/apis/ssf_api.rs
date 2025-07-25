/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.6.4
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for typed errors of method [`ssf_streams_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SsfStreamsListError {
    Status400(models::ValidationError),
    Status403(models::GenericError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ssf_streams_retrieve`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SsfStreamsRetrieveError {
    Status400(models::ValidationError),
    Status403(models::GenericError),
    UnknownValue(serde_json::Value),
}

/// SSFStream Viewset
pub async fn ssf_streams_list(
    configuration: &configuration::Configuration,
    delivery_method: Option<&str>,
    endpoint_url: Option<&str>,
    ordering: Option<&str>,
    page: Option<i32>,
    page_size: Option<i32>,
    provider: Option<i32>,
    search: Option<&str>,
) -> Result<models::PaginatedSsfStreamList, Error<SsfStreamsListError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_delivery_method = delivery_method;
    let p_endpoint_url = endpoint_url;
    let p_ordering = ordering;
    let p_page = page;
    let p_page_size = page_size;
    let p_provider = provider;
    let p_search = search;

    let uri_str = format!("{}/ssf/streams/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_delivery_method {
        req_builder = req_builder.query(&[("delivery_method", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_endpoint_url {
        req_builder = req_builder.query(&[("endpoint_url", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_ordering {
        req_builder = req_builder.query(&[("ordering", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page_size {
        req_builder = req_builder.query(&[("page_size", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_provider {
        req_builder = req_builder.query(&[("provider", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_search {
        req_builder = req_builder.query(&[("search", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PaginatedSsfStreamList`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PaginatedSsfStreamList`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SsfStreamsListError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// SSFStream Viewset
pub async fn ssf_streams_retrieve(
    configuration: &configuration::Configuration,
    uuid: &str,
) -> Result<models::SsfStream, Error<SsfStreamsRetrieveError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_uuid = uuid;

    let uri_str = format!(
        "{}/ssf/streams/{uuid}/",
        configuration.base_path,
        uuid = crate::apis::urlencode(p_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `models::SsfStream`",
                )))
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::SsfStream`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SsfStreamsRetrieveError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
