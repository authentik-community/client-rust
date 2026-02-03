# AuthenticatorValidationChallengeResponseRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**component** | Option<**String**> |  | [optional][default to ak-stage-authenticator-validate]
**selected_challenge** | Option<[**models::DeviceChallengeRequest**](DeviceChallengeRequest.md)> |  | [optional]
**selected_stage** | Option<**String**> |  | [optional]
**code** | Option<**String**> |  | [optional]
**webauthn** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**duo** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


