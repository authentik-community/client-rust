# AuthenticatorValidationChallenge

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**flow_info** | Option<[**models::ContextualFlowInfo**](ContextualFlowInfo.md)> |  | [optional]
**component** | Option<**String**> |  | [optional][default to ak-stage-authenticator-validate]
**response_errors** | Option<[**std::collections::HashMap<String, Vec<models::ErrorDetail>>**](Vec.md)> |  | [optional]
**pending_user** | **String** |  | 
**pending_user_avatar** | **String** |  | 
**device_challenges** | [**Vec<models::DeviceChallenge>**](DeviceChallenge.md) |  | 
**configuration_stages** | [**Vec<models::SelectableStage>**](SelectableStage.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


