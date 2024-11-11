# CaptchaStage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**name** | **String** |  | 
**component** | **String** | Get object type so that we know how to edit the object | [readonly]
**verbose_name** | **String** | Return object's verbose_name | [readonly]
**verbose_name_plural** | **String** | Return object's plural verbose_name | [readonly]
**meta_model_name** | **String** | Return internal model name | [readonly]
**flow_set** | Option<[**Vec<models::FlowSet>**](FlowSet.md)> |  | [optional]
**public_key** | **String** | Public key, acquired your captcha Provider. | 
**js_url** | Option<**String**> |  | [optional]
**api_url** | Option<**String**> |  | [optional]
**interactive** | Option<**bool**> |  | [optional]
**score_min_threshold** | Option<**f64**> |  | [optional]
**score_max_threshold** | Option<**f64**> |  | [optional]
**error_on_invalid_score** | Option<**bool**> | When enabled and the received captcha score is outside of the given threshold, the stage will show an error message. When not enabled, the flow will continue, but the data from the captcha will be available in the context for policy decisions | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


