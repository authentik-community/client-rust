# FlowStageBindingRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**target** | **uuid::Uuid** |  | 
**stage** | **uuid::Uuid** |  | 
**evaluate_on_plan** | Option<**bool**> | Evaluate policies during the Flow planning process. | [optional]
**re_evaluate_policies** | Option<**bool**> | Evaluate policies when the Stage is presented to the user. | [optional]
**order** | **i32** |  | 
**policy_engine_mode** | Option<[**models::PolicyEngineMode**](PolicyEngineMode.md)> |  | [optional]
**invalid_response_action** | Option<[**models::InvalidResponseActionEnum**](InvalidResponseActionEnum.md)> | Configure how the flow executor should handle an invalid response to a challenge. RETRY returns the error message and a similar challenge to the executor. RESTART restarts the flow from the beginning, and RESTART_WITH_CONTEXT restarts the flow while keeping the current context. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


