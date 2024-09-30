# InvitationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**expires** | Option<**String**> |  | [optional]
**fixed_data** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**single_use** | Option<**bool**> | When enabled, the invitation will be deleted after usage. | [optional]
**flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | When set, only the configured flow can use this invitation. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


