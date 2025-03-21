# ScimProviderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**property_mappings_group** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Property mappings used for group creation/updating. | [optional]
**url** | **String** | Base URL to SCIM requests, usually ends in /v2 | 
**verify_certificates** | Option<**bool**> |  | [optional]
**token** | **String** | Authentication token | 
**compatibility_mode** | Option<[**models::CompatibilityModeEnum**](CompatibilityModeEnum.md)> | Alter authentik behavior for vendor-specific SCIM implementations. | [optional]
**exclude_users_service_account** | Option<**bool**> |  | [optional]
**filter_group** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**dry_run** | Option<**bool**> | When enabled, provider will not modify or create objects in the remote system. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


