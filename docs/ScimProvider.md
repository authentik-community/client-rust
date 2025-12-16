# ScimProvider

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**name** | **String** |  | 
**property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**property_mappings_group** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Property mappings used for group creation/updating. | [optional]
**component** | **String** | Get object component so that we know how to edit the object | [readonly]
**assigned_backchannel_application_slug** | **String** | Internal application name, used in URLs. | [readonly]
**assigned_backchannel_application_name** | **String** | Application's display Name. | [readonly]
**verbose_name** | **String** | Return object's verbose_name | [readonly]
**verbose_name_plural** | **String** | Return object's plural verbose_name | [readonly]
**meta_model_name** | **String** | Return internal model name | [readonly]
**url** | **String** | Base URL to SCIM requests, usually ends in /v2 | 
**verify_certificates** | Option<**bool**> |  | [optional]
**token** | Option<**String**> | Authentication token | [optional]
**auth_mode** | Option<[**models::ScimAuthenticationModeEnum**](SCIMAuthenticationModeEnum.md)> |  | [optional]
**auth_oauth** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | OAuth Source used for authentication | [optional]
**auth_oauth_params** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Additional OAuth parameters, such as grant_type | [optional]
**compatibility_mode** | Option<[**models::CompatibilityModeEnum**](CompatibilityModeEnum.md)> | Alter authentik behavior for vendor-specific SCIM implementations. | [optional]
**service_provider_config_cache_timeout** | Option<**String**> | Cache duration for ServiceProviderConfig responses. Set minutes=0 to disable. | [optional]
**exclude_users_service_account** | Option<**bool**> |  | [optional]
**filter_group** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**sync_page_size** | Option<**u32**> | Controls the number of objects synced in a single task | [optional]
**sync_page_timeout** | Option<**String**> | Timeout for synchronization of a single page | [optional]
**dry_run** | Option<**bool**> | When enabled, provider will not modify or create objects in the remote system. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


