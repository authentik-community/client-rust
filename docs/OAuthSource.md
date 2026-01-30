# OAuthSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**name** | **String** | Source's display Name. | 
**slug** | **String** | Internal source name, used in URLs. | 
**enabled** | Option<**bool**> |  | [optional]
**promoted** | Option<**bool**> | When enabled, this source will be displayed as a prominent button on the login page, instead of a small icon. | [optional]
**authentication_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow to use when authenticating existing users. | [optional]
**enrollment_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow to use when enrolling new users. | [optional]
**user_property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**group_property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**component** | **String** | Get object component so that we know how to edit the object | [readonly]
**verbose_name** | **String** | Return object's verbose_name | [readonly]
**verbose_name_plural** | **String** | Return object's plural verbose_name | [readonly]
**meta_model_name** | **String** | Return internal model name | [readonly]
**policy_engine_mode** | Option<[**models::PolicyEngineMode**](PolicyEngineMode.md)> |  | [optional]
**user_matching_mode** | Option<[**models::UserMatchingModeEnum**](UserMatchingModeEnum.md)> | How the source determines if an existing user should be authenticated or a new user enrolled. | [optional]
**managed** | Option<**String**> | Objects that are managed by authentik. These objects are created and updated automatically. This flag only indicates that an object can be overwritten by migrations. You can still modify the objects via the API, but expect changes to be overwritten in a later update. | [readonly]
**user_path_template** | Option<**String**> |  | [optional]
**icon** | Option<**String**> |  | [optional]
**icon_url** | Option<**String**> |  | [readonly]
**icon_themed_urls** | Option<[**models::ThemedUrls**](ThemedUrls.md)> |  | [readonly]
**group_matching_mode** | Option<[**models::GroupMatchingModeEnum**](GroupMatchingModeEnum.md)> | How the source determines if an existing group should be used or a new group created. | [optional]
**provider_type** | [**models::ProviderTypeEnum**](ProviderTypeEnum.md) |  | 
**request_token_url** | Option<**String**> | URL used to request the initial token. This URL is only required for OAuth 1. | [optional]
**authorization_url** | Option<**String**> | URL the user is redirect to to conest the flow. | [optional]
**access_token_url** | Option<**String**> | URL used by authentik to retrieve tokens. | [optional]
**profile_url** | Option<**String**> | URL used by authentik to get user information. | [optional]
**pkce** | Option<[**models::PkceMethodEnum**](PKCEMethodEnum.md)> |  | [optional]
**consumer_key** | **String** |  | 
**callback_url** | **String** | Get OAuth Callback URL | [readonly]
**additional_scopes** | Option<**String**> |  | [optional]
**r#type** | [**models::SourceType**](SourceType.md) |  | [readonly]
**oidc_well_known_url** | Option<**String**> |  | [optional]
**oidc_jwks_url** | Option<**String**> |  | [optional]
**oidc_jwks** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**authorization_code_auth_method** | Option<[**models::AuthorizationCodeAuthMethodEnum**](AuthorizationCodeAuthMethodEnum.md)> | How to perform authentication during an authorization_code token request flow | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


