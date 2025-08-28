# PatchedOAuthSourceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Source's display Name. | [optional]
**slug** | Option<**String**> | Internal source name, used in URLs. | [optional]
**enabled** | Option<**bool**> |  | [optional]
**authentication_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow to use when authenticating existing users. | [optional]
**enrollment_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow to use when enrolling new users. | [optional]
**user_property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**group_property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**policy_engine_mode** | Option<[**models::PolicyEngineMode**](PolicyEngineMode.md)> |  | [optional]
**user_matching_mode** | Option<[**models::UserMatchingModeEnum**](UserMatchingModeEnum.md)> | How the source determines if an existing user should be authenticated or a new user enrolled. | [optional]
**user_path_template** | Option<**String**> |  | [optional]
**group_matching_mode** | Option<[**models::GroupMatchingModeEnum**](GroupMatchingModeEnum.md)> | How the source determines if an existing group should be used or a new group created. | [optional]
**provider_type** | Option<[**models::ProviderTypeEnum**](ProviderTypeEnum.md)> |  | [optional]
**request_token_url** | Option<**String**> | URL used to request the initial token. This URL is only required for OAuth 1. | [optional]
**authorization_url** | Option<**String**> | URL the user is redirect to to conest the flow. | [optional]
**access_token_url** | Option<**String**> | URL used by authentik to retrieve tokens. | [optional]
**profile_url** | Option<**String**> | URL used by authentik to get user information. | [optional]
**consumer_key** | Option<**String**> |  | [optional]
**consumer_secret** | Option<**String**> |  | [optional]
**additional_scopes** | Option<**String**> |  | [optional]
**oidc_well_known_url** | Option<**String**> |  | [optional]
**oidc_jwks_url** | Option<**String**> |  | [optional]
**oidc_jwks** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


