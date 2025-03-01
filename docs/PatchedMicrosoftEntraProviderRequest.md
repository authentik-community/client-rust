# PatchedMicrosoftEntraProviderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**property_mappings_group** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Property mappings used for group creation/updating. | [optional]
**client_id** | Option<**String**> |  | [optional]
**client_secret** | Option<**String**> |  | [optional]
**tenant_id** | Option<**String**> |  | [optional]
**exclude_users_service_account** | Option<**bool**> |  | [optional]
**filter_group** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**user_delete_action** | Option<[**models::OutgoingSyncDeleteAction**](OutgoingSyncDeleteAction.md)> |  | [optional]
**group_delete_action** | Option<[**models::OutgoingSyncDeleteAction**](OutgoingSyncDeleteAction.md)> |  | [optional]
**dry_run** | Option<**bool**> | When enabled, provider will not modify or create objects in the remote system. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


