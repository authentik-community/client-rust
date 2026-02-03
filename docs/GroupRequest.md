# GroupRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**is_superuser** | Option<**bool**> | Users added to this group will be superusers. | [optional]
**parents** | Option<**Vec<uuid::Uuid>**> |  | [optional]
**users** | Option<**Vec<i32>**> |  | [optional]
**attributes** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**roles** | Option<**Vec<uuid::Uuid>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


