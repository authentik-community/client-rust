# ScimProviderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**property_mappings** | Option<**Vec<uuid::Uuid>**> |  | [optional]
**property_mappings_group** | Option<**Vec<uuid::Uuid>**> | Property mappings used for group creation/updating. | [optional]
**url** | **String** | Base URL to SCIM requests, usually ends in /v2 | 
**verify_certificates** | Option<**bool**> |  | [optional]
**token** | Option<**String**> | Authentication token | [optional]
**auth_mode** | Option<[**models::ScimAuthenticationModeEnum**](SCIMAuthenticationModeEnum.md)> |  | [optional]
**auth_oauth** | Option<**uuid::Uuid**> | OAuth Source used for authentication | [optional]
**auth_oauth_params** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Additional OAuth parameters, such as grant_type | [optional]
**compatibility_mode** | Option<[**models::CompatibilityModeEnum**](CompatibilityModeEnum.md)> | Alter authentik behavior for vendor-specific SCIM implementations. | [optional]
**service_provider_config_cache_timeout** | Option<**String**> | Cache duration for ServiceProviderConfig responses. Set minutes=0 to disable. | [optional]
**exclude_users_service_account** | Option<**bool**> |  | [optional]
**sync_page_size** | Option<**u32**> | Controls the number of objects synced in a single task | [optional]
**sync_page_timeout** | Option<**String**> | Timeout for synchronization of a single page | [optional]
**group_filters** | Option<**Vec<uuid::Uuid>**> | Group filters used to define sync-scope for groups. | [optional]
**dry_run** | Option<**bool**> | When enabled, provider will not modify or create objects in the remote system. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


