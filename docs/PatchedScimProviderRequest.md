# PatchedScimProviderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**property_mappings_group** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Property mappings used for group creation/updating. | [optional]
**url** | Option<**String**> | Base URL to SCIM requests, usually ends in /v2 | [optional]
**verify_certificates** | Option<**bool**> |  | [optional]
**token** | Option<**String**> | Authentication token | [optional]
**auth_mode** | Option<[**models::ScimAuthenticationModeEnum**](SCIMAuthenticationModeEnum.md)> |  | [optional]
**auth_oauth** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | OAuth Source used for authentication | [optional]
**auth_oauth_params** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Additional OAuth parameters, such as grant_type | [optional]
**compatibility_mode** | Option<[**models::CompatibilityModeEnum**](CompatibilityModeEnum.md)> | Alter authentik behavior for vendor-specific SCIM implementations. | [optional]
**exclude_users_service_account** | Option<**bool**> |  | [optional]
**filter_group** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**dry_run** | Option<**bool**> | When enabled, provider will not modify or create objects in the remote system. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


