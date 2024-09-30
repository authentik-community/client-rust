# PatchedUserRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**username** | Option<**String**> |  | [optional]
**name** | Option<**String**> | User's display name. | [optional]
**is_active** | Option<**bool**> | Designates whether this user should be treated as active. Unselect this instead of deleting accounts. | [optional]
**last_login** | Option<**String**> |  | [optional]
**groups** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**email** | Option<**String**> |  | [optional]
**attributes** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**path** | Option<**String**> |  | [optional]
**r#type** | Option<[**models::UserTypeEnum**](UserTypeEnum.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


