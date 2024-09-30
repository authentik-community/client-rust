# ScopeMappingRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**managed** | Option<**String**> | Objects that are managed by authentik. These objects are created and updated automatically. This flag only indicates that an object can be overwritten by migrations. You can still modify the objects via the API, but expect changes to be overwritten in a later update. | [optional]
**name** | **String** |  | 
**expression** | **String** |  | 
**scope_name** | **String** | Scope name requested by the client | 
**description** | Option<**String**> | Description shown to the user when consenting. If left empty, the user won't be informed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


