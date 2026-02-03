# ProxyOutpostConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**name** | **String** |  | 
**internal_host** | Option<**String**> |  | [optional]
**external_host** | **String** |  | 
**internal_host_ssl_validation** | Option<**bool**> | Validate SSL Certificates of upstream servers | [optional]
**client_id** | Option<**String**> |  | [optional]
**client_secret** | Option<**String**> |  | [optional]
**oidc_configuration** | [**models::OpenIdConnectConfiguration**](OpenIDConnectConfiguration.md) |  | [readonly]
**cookie_secret** | Option<**String**> |  | [optional]
**certificate** | Option<**uuid::Uuid**> |  | [optional]
**skip_path_regex** | Option<**String**> | Regular expressions for which authentication is not required. Each new line is interpreted as a new Regular Expression. | [optional]
**basic_auth_enabled** | Option<**bool**> | Set a custom HTTP-Basic Authentication header based on values from authentik. | [optional]
**basic_auth_password_attribute** | Option<**String**> | User/Group Attribute used for the password part of the HTTP-Basic Header. | [optional]
**basic_auth_user_attribute** | Option<**String**> | User/Group Attribute used for the user part of the HTTP-Basic Header. If not set, the user's Email address is used. | [optional]
**mode** | Option<[**models::ProxyMode**](ProxyMode.md)> | Enable support for forwardAuth in traefik and nginx auth_request. Exclusive with internal_host. | [optional]
**cookie_domain** | Option<**String**> |  | [optional]
**access_token_validity** | Option<**f64**> | Get token validity as second count | [readonly]
**intercept_header_auth** | Option<**bool**> | When enabled, this provider will intercept the authorization header and authenticate requests based on its value. | [optional]
**scopes_to_request** | **Vec<String>** | Get all the scope names the outpost should request, including custom-defined ones | [readonly]
**assigned_application_slug** | **String** | Internal application name, used in URLs. | [readonly]
**assigned_application_name** | **String** | Application's display Name. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


