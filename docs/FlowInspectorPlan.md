# FlowInspectorPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current_stage** | [**models::FlowStageBinding**](FlowStageBinding.md) |  | [readonly]
**next_planned_stage** | [**models::FlowStageBinding**](FlowStageBinding.md) |  | [readonly]
**plan_context** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | Get the plan's context, sanitized | [readonly]
**session_id** | **String** | Get a unique session ID | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


