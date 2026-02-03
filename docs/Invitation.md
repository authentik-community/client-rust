# Invitation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **uuid::Uuid** |  | [readonly]
**name** | **String** |  | 
**expires** | Option<**String**> |  | [optional]
**fixed_data** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**created_by** | [**models::PartialUser**](PartialUser.md) |  | [readonly]
**single_use** | Option<**bool**> | When enabled, the invitation will be deleted after usage. | [optional]
**flow** | Option<**uuid::Uuid**> | When set, only the configured flow can use this invitation. | [optional]
**flow_obj** | [**models::Flow**](Flow.md) |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


