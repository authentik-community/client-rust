# UserAssignedObjectPermission

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**username** | **String** | Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only. | 
**name** | **String** | User's display name. | 
**is_active** | Option<**bool**> | Designates whether this user should be treated as active. Unselect this instead of deleting accounts. | [optional]
**last_login** | Option<**String**> |  | [optional]
**email** | Option<**String**> |  | [optional]
**attributes** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**uid** | **String** |  | [readonly]
**permissions** | [**Vec<models::UserObjectPermission>**](UserObjectPermission.md) |  | 
**is_superuser** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


