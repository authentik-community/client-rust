# AuthenticatorDuoStage

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
**configure_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow used by an authenticated user to configure this Stage. If empty, user will not be able to configure this stage. | [optional]
**friendly_name** | Option<**String**> |  | [optional]
**client_id** | **String** |  | 
**api_hostname** | **String** |  | 
**admin_integration_key** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


