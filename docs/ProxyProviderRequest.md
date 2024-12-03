# ProxyProviderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**authentication_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow used for authentication when the associated application is accessed by an un-authenticated user. | [optional]
**authorization_flow** | [**uuid::Uuid**](uuid::Uuid.md) | Flow used when authorizing this provider. | 
**invalidation_flow** | [**uuid::Uuid**](uuid::Uuid.md) | Flow used ending the session from a provider. | 
**property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
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
**cookie_domain** | Option<**String**> |  | [optional]
**jwt_federation_sources** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**jwt_federation_providers** | Option<**Vec<i32>**> |  | [optional]
**access_token_validity** | Option<**String**> | Tokens not valid on or after current time + this value (Format: hours=1;minutes=2;seconds=3). | [optional]
**refresh_token_validity** | Option<**String**> | Tokens not valid on or after current time + this value (Format: hours=1;minutes=2;seconds=3). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


