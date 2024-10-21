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
**exclude_users_service_account** | Option<**bool**> |  | [optional]
**filter_group** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


