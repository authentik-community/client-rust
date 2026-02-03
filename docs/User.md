# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**username** | **String** |  | 
**name** | **String** | User's display name. | 
**is_active** | Option<**bool**> | Designates whether this user should be treated as active. Unselect this instead of deleting accounts. | [optional]
**last_login** | Option<**String**> |  | [optional]
**date_joined** | **String** |  | [readonly]
**is_superuser** | **bool** |  | [readonly]
**groups** | Option<**Vec<uuid::Uuid>**> |  | [optional]
**groups_obj** | Option<[**Vec<models::PartialGroup>**](PartialGroup.md)> |  | [readonly]
**roles** | Option<**Vec<uuid::Uuid>**> |  | [optional]
**roles_obj** | Option<[**Vec<models::Role>**](Role.md)> |  | [readonly]
**email** | Option<**String**> |  | [optional]
**avatar** | **String** | User's avatar, either a http/https URL or a data URI | [readonly]
**attributes** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**uid** | **String** |  | [readonly]
**path** | Option<**String**> |  | [optional]
**r#type** | Option<[**models::UserTypeEnum**](UserTypeEnum.md)> |  | [optional]
**uuid** | **uuid::Uuid** |  | [readonly]
**password_change_date** | **String** |  | [readonly]
**last_updated** | **String** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


