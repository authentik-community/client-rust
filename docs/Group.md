# Group

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**num_pk** | **i32** |  | [readonly]
**name** | **String** |  | 
**is_superuser** | Option<**bool**> | Users added to this group will be superusers. | [optional]
**parent** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**parent_name** | Option<**String**> |  | [readonly]
**users** | Option<**Vec<i32>**> |  | [optional]
**users_obj** | Option<[**Vec<models::PartialUser>**](PartialUser.md)> |  | [readonly]
**attributes** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**roles** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**roles_obj** | [**Vec<models::Role>**](Role.md) |  | [readonly]
**children** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**children_obj** | Option<[**Vec<models::GroupChild>**](GroupChild.md)> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


