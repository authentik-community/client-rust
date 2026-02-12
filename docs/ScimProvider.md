# ScimProvider

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**name** | **String** |  | 
**property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**property_mappings_group** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Property mappings used for group creation/updating. | [optional]
**component** | **String** | Get object component so that we know how to edit the object | [readonly]
**assigned_backchannel_application_slug** | Option<**String**> | Internal application name, used in URLs. | [readonly]
**assigned_backchannel_application_name** | Option<**String**> | Application's display Name. | [readonly]
**verbose_name** | **String** | Return object's verbose_name | [readonly]
**verbose_name_plural** | **String** | Return object's plural verbose_name | [readonly]
**meta_model_name** | **String** | Return internal model name | [readonly]
**url** | **String** | Base URL to SCIM requests, usually ends in /v2 | 
**verify_certificates** | Option<**bool**> |  | [optional]
**token** | **String** | Authentication token | 
**compatibility_mode** | Option<[**models::CompatibilityModeEnum**](CompatibilityModeEnum.md)> | Alter authentik behavior for vendor-specific SCIM implementations. | [optional]
**exclude_users_service_account** | Option<**bool**> |  | [optional]
**filter_group** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**dry_run** | Option<**bool**> | When enabled, provider will not modify or create objects in the remote system. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


