# Prompt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**name** | **String** |  | 
**field_key** | **String** | Name of the form field, also used to store the value | 
**label** | **String** |  | 
**r#type** | [**models::PromptTypeEnum**](PromptTypeEnum.md) |  | 
**required** | Option<**bool**> |  | [optional]
**placeholder** | Option<**String**> | Optionally provide a short hint that describes the expected input value. When creating a fixed choice field, enable interpreting as expression and return a list to return multiple choices. | [optional]
**initial_value** | Option<**String**> | Optionally pre-fill the input with an initial value. When creating a fixed choice field, enable interpreting as expression and return a list to return multiple default choices. | [optional]
**order** | Option<**i32**> |  | [optional]
**prompt_stages_obj** | [**Vec<models::PromptStage>**](PromptStage.md) |  | [readonly]
**sub_text** | Option<**String**> |  | [optional]
**placeholder_expression** | Option<**bool**> |  | [optional]
**initial_value_expression** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


