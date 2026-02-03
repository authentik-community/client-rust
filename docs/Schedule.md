# Schedule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [readonly]
**identifier** | Option<**String**> | Unique schedule identifier | [readonly]
**uid** | **String** |  | [readonly]
**actor_name** | **String** | Dramatiq actor to call | [readonly]
**rel_obj_app_label** | **String** |  | [readonly]
**rel_obj_model** | **String** |  | [readonly]
**rel_obj_id** | Option<**String**> |  | [optional]
**crontab** | **String** | When to schedule tasks | 
**paused** | Option<**bool**> | Pause this schedule | [optional]
**next_run** | **String** |  | [readonly]
**description** | Option<**String**> |  | [readonly]
**last_task_status** | Option<[**models::LastTaskStatusEnum**](LastTaskStatusEnum.md)> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


