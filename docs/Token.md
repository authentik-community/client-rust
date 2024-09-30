# Token

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**managed** | Option<**String**> | Objects that are managed by authentik. These objects are created and updated automatically. This flag only indicates that an object can be overwritten by migrations. You can still modify the objects via the API, but expect changes to be overwritten in a later update. | [optional]
**identifier** | **String** |  | 
**intent** | Option<[**models::IntentEnum**](IntentEnum.md)> |  | [optional]
**user** | Option<**i32**> |  | [optional]
**user_obj** | [**models::User**](User.md) |  | [readonly]
**description** | Option<**String**> |  | [optional]
**expires** | Option<**String**> |  | [optional]
**expiring** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


