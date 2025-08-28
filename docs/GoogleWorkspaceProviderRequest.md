# GoogleWorkspaceProviderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**property_mappings_group** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Property mappings used for group creation/updating. | [optional]
**delegated_subject** | **String** |  | 
**credentials** | Option<[**serde_json::Value**](.md)> |  | 
**scopes** | Option<**String**> |  | [optional]
**exclude_users_service_account** | Option<**bool**> |  | [optional]
**filter_group** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**user_delete_action** | Option<[**models::OutgoingSyncDeleteAction**](OutgoingSyncDeleteAction.md)> |  | [optional]
**group_delete_action** | Option<[**models::OutgoingSyncDeleteAction**](OutgoingSyncDeleteAction.md)> |  | [optional]
**default_group_email_domain** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


