# UserGroupRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**is_superuser** | Option<**bool**> | Users added to this group will be superusers. | [optional]
**parent** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**attributes** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


