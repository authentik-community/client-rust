# MicrosoftEntraProvider

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**name** | **String** |  | 
**property_mappings** | Option<**Vec<uuid::Uuid>**> |  | [optional]
**property_mappings_group** | Option<**Vec<uuid::Uuid>**> | Property mappings used for group creation/updating. | [optional]
**component** | **String** | Get object component so that we know how to edit the object | [readonly]
**assigned_backchannel_application_slug** | Option<**String**> | Internal application name, used in URLs. | [readonly]
**assigned_backchannel_application_name** | Option<**String**> | Application's display Name. | [readonly]
**verbose_name** | **String** | Return object's verbose_name | [readonly]
**verbose_name_plural** | **String** | Return object's plural verbose_name | [readonly]
**meta_model_name** | **String** | Return internal model name | [readonly]
**client_id** | **String** |  | 
**client_secret** | **String** |  | 
**tenant_id** | **String** |  | 
**exclude_users_service_account** | Option<**bool**> |  | [optional]
**filter_group** | Option<**uuid::Uuid**> |  | [optional]
**user_delete_action** | Option<[**models::OutgoingSyncDeleteAction**](OutgoingSyncDeleteAction.md)> |  | [optional]
**group_delete_action** | Option<[**models::OutgoingSyncDeleteAction**](OutgoingSyncDeleteAction.md)> |  | [optional]
**sync_page_size** | Option<**u32**> | Controls the number of objects synced in a single task | [optional]
**sync_page_timeout** | Option<**String**> | Timeout for synchronization of a single page | [optional]
**dry_run** | Option<**bool**> | When enabled, provider will not modify or create objects in the remote system. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


