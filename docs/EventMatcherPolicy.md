# EventMatcherPolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**name** | **String** |  | 
**execution_logging** | Option<**bool**> | When this option is enabled, all executions of this policy will be logged. By default, only execution errors are logged. | [optional]
**component** | **String** | Get object component so that we know how to edit the object | [readonly]
**verbose_name** | **String** | Return object's verbose_name | [readonly]
**verbose_name_plural** | **String** | Return object's plural verbose_name | [readonly]
**meta_model_name** | **String** | Return internal model name | [readonly]
**bound_to** | **i32** | Return objects policy is bound to | [readonly]
**action** | Option<[**models::EventActions**](EventActions.md)> | Match created events with this action type. When left empty, all action types will be matched. | [optional]
**client_ip** | Option<**String**> | Matches Event's Client IP (strict matching, for network matching use an Expression Policy) | [optional]
**app** | Option<[**models::AppEnum**](AppEnum.md)> | Match events created by selected application. When left empty, all applications are matched. | [optional]
**model** | Option<[**models::ModelEnum**](ModelEnum.md)> | Match events created by selected model. When left empty, all models are matched. When an app is selected, all the application's models are matched. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


