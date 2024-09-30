# SourceStageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**flow_set** | Option<[**Vec<models::FlowSetRequest>**](FlowSetRequest.md)> |  | [optional]
**source** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**resume_timeout** | Option<**String**> | Amount of time a user can take to return from the source to continue the flow (Format: hours=-1;minutes=-2;seconds=-3) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


