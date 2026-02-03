# PatchedGoogleWorkspaceProviderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**property_mappings** | Option<**Vec<uuid::Uuid>**> |  | [optional]
**property_mappings_group** | Option<**Vec<uuid::Uuid>**> | Property mappings used for group creation/updating. | [optional]
**delegated_subject** | Option<**String**> |  | [optional]
**credentials** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**scopes** | Option<**String**> |  | [optional]
**exclude_users_service_account** | Option<**bool**> |  | [optional]
**filter_group** | Option<**uuid::Uuid**> |  | [optional]
**user_delete_action** | Option<[**models::OutgoingSyncDeleteAction**](OutgoingSyncDeleteAction.md)> |  | [optional]
**group_delete_action** | Option<[**models::OutgoingSyncDeleteAction**](OutgoingSyncDeleteAction.md)> |  | [optional]
**default_group_email_domain** | Option<**String**> |  | [optional]
**sync_page_size** | Option<**u32**> | Controls the number of objects synced in a single task | [optional]
**sync_page_timeout** | Option<**String**> | Timeout for synchronization of a single page | [optional]
**dry_run** | Option<**bool**> | When enabled, provider will not modify or create objects in the remote system. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


