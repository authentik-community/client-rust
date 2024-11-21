# ProxyProvider

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**name** | **String** |  | 
**authentication_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow used for authentication when the associated application is accessed by an un-authenticated user. | [optional]
**authorization_flow** | [**uuid::Uuid**](uuid::Uuid.md) | Flow used when authorizing this provider. | 
**invalidation_flow** | [**uuid::Uuid**](uuid::Uuid.md) | Flow used ending the session from a provider. | 
**property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**component** | **String** | Get object component so that we know how to edit the object | [readonly]
**assigned_application_slug** | **String** | Internal application name, used in URLs. | [readonly]
**assigned_application_name** | **String** | Application's display Name. | [readonly]
**assigned_backchannel_application_slug** | **String** | Internal application name, used in URLs. | [readonly]
**assigned_backchannel_application_name** | **String** | Application's display Name. | [readonly]
**verbose_name** | **String** | Return object's verbose_name | [readonly]
**verbose_name_plural** | **String** | Return object's plural verbose_name | [readonly]
**meta_model_name** | **String** | Return internal model name | [readonly]
**client_id** | **String** |  | [readonly]
**internal_host** | Option<**String**> |  | [optional]
**external_host** | **String** |  | 
**internal_host_ssl_validation** | Option<**bool**> | Validate SSL Certificates of upstream servers | [optional]
**certificate** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**skip_path_regex** | Option<**String**> | Regular expressions for which authentication is not required. Each new line is interpreted as a new Regular Expression. | [optional]
**basic_auth_enabled** | Option<**bool**> | Set a custom HTTP-Basic Authentication header based on values from authentik. | [optional]
**basic_auth_password_attribute** | Option<**String**> | User/Group Attribute used for the password part of the HTTP-Basic Header. | [optional]
**basic_auth_user_attribute** | Option<**String**> | User/Group Attribute used for the user part of the HTTP-Basic Header. If not set, the user's Email address is used. | [optional]
**mode** | Option<[**models::ProxyMode**](ProxyMode.md)> | Enable support for forwardAuth in traefik and nginx auth_request. Exclusive with internal_host. | [optional]
**intercept_header_auth** | Option<**bool**> | When enabled, this provider will intercept the authorization header and authenticate requests based on its value. | [optional]
**redirect_uris** | [**Vec<models::RedirectUri>**](RedirectURI.md) |  | [readonly]
**cookie_domain** | Option<**String**> |  | [optional]
**jwks_sources** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**access_token_validity** | Option<**String**> | Tokens not valid on or after current time + this value (Format: hours=1;minutes=2;seconds=3). | [optional]
**refresh_token_validity** | Option<**String**> | Tokens not valid on or after current time + this value (Format: hours=1;minutes=2;seconds=3). | [optional]
**outpost_set** | **Vec<String>** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


