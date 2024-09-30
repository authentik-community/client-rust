# EventMatcherPolicyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**execution_logging** | Option<**bool**> | When this option is enabled, all executions of this policy will be logged. By default, only execution errors are logged. | [optional]
**action** | Option<[**models::EventActions**](EventActions.md)> | Match created events with this action type. When left empty, all action types will be matched. | [optional]
**client_ip** | Option<**String**> | Matches Event's Client IP (strict matching, for network matching use an Expression Policy) | [optional]
**app** | Option<[**models::AppEnum**](AppEnum.md)> | Match events created by selected application. When left empty, all applications are matched. | [optional]
**model** | Option<[**models::ModelEnum**](ModelEnum.md)> | Match events created by selected model. When left empty, all models are matched. When an app is selected, all the application's models are matched. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


