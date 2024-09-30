# UserSelf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**username** | **String** | Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only. | 
**name** | **String** | User's display name. | 
**is_active** | **bool** | Designates whether this user should be treated as active. Unselect this instead of deleting accounts. | [readonly]
**is_superuser** | **bool** |  | [readonly]
**groups** | [**Vec<models::UserSelfGroups>**](UserSelfGroups.md) |  | [readonly]
**email** | Option<**String**> |  | [optional]
**avatar** | **String** | User's avatar, either a http/https URL or a data URI | [readonly]
**uid** | **String** |  | [readonly]
**settings** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | Get user settings with brand and group settings applied | [readonly]
**r#type** | Option<[**models::UserTypeEnum**](UserTypeEnum.md)> |  | [optional]
**system_permissions** | **Vec<String>** | Get all system permissions assigned to the user | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


