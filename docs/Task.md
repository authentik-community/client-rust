# Task

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**queue_name** | Option<**String**> | Queue name | [optional]
**actor_name** | **String** | Dramatiq actor name | 
**state** | Option<[**models::StateEnum**](StateEnum.md)> | Task status | [optional]
**mtime** | Option<**String**> | Task last modified time | [optional]
**retries** | Option<**u64**> | Number of retries | [optional]
**eta** | Option<**String**> | Planned execution time | [optional]
**rel_obj_app_label** | **String** |  | [readonly]
**rel_obj_model** | **String** |  | [readonly]
**rel_obj_id** | Option<**String**> |  | [optional]
**uid** | **String** |  | [readonly]
**messages** | [**Vec<models::LogEvent>**](LogEvent.md) |  | 
**previous_messages** | [**Vec<models::LogEvent>**](LogEvent.md) |  | 
**aggregated_status** | [**models::TaskAggregatedStatusEnum**](TaskAggregatedStatusEnum.md) |  | 
**description** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


