# MicrosoftEntraProvider

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
**client_id** | **String** |  | 
**client_secret** | **String** |  | 
**tenant_id** | **String** |  | 
**exclude_users_service_account** | Option<**bool**> |  | [optional]
**filter_group** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**user_delete_action** | Option<[**models::OutgoingSyncDeleteAction**](OutgoingSyncDeleteAction.md)> |  | [optional]
**group_delete_action** | Option<[**models::OutgoingSyncDeleteAction**](OutgoingSyncDeleteAction.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


