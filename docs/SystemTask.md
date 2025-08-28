# SystemTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uuid** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**name** | **String** |  | 
**full_name** | **String** | Get full name with UID | [readonly]
**uid** | Option<**String**> |  | [optional]
**description** | **String** |  | 
**start_timestamp** | **String** |  | [readonly]
**finish_timestamp** | **String** |  | [readonly]
**duration** | **f64** |  | [readonly]
**status** | [**models::SystemTaskStatusEnum**](SystemTaskStatusEnum.md) |  | 
**messages** | [**Vec<models::LogEvent>**](LogEvent.md) |  | 
**expires** | Option<**String**> |  | [optional]
**expiring** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


