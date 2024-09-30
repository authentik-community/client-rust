# PatchedAuthenticatorDuoStageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**flow_set** | Option<[**Vec<models::FlowSetRequest>**](FlowSetRequest.md)> |  | [optional]
**configure_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow used by an authenticated user to configure this Stage. If empty, user will not be able to configure this stage. | [optional]
**friendly_name** | Option<**String**> |  | [optional]
**client_id** | Option<**String**> |  | [optional]
**client_secret** | Option<**String**> |  | [optional]
**api_hostname** | Option<**String**> |  | [optional]
**admin_integration_key** | Option<**String**> |  | [optional]
**admin_secret_key** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


