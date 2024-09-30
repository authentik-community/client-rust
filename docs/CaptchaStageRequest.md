# CaptchaStageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**flow_set** | Option<[**Vec<models::FlowSetRequest>**](FlowSetRequest.md)> |  | [optional]
**public_key** | **String** | Public key, acquired your captcha Provider. | 
**private_key** | **String** | Private key, acquired your captcha Provider. | 
**js_url** | Option<**String**> |  | [optional]
**api_url** | Option<**String**> |  | [optional]
**score_min_threshold** | Option<**f64**> |  | [optional]
**score_max_threshold** | Option<**f64**> |  | [optional]
**error_on_invalid_score** | Option<**bool**> | When enabled and the received captcha score is outside of the given threshold, the stage will show an error message. When not enabled, the flow will continue, but the data from the captcha will be available in the context for policy decisions | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


