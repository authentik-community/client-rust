# AuthenticatorTotpStageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**configure_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow used by an authenticated user to configure this Stage. If empty, user will not be able to configure this stage. | [optional]
**friendly_name** | Option<**String**> |  | [optional]
**digits** | [**models::DigitsEnum**](DigitsEnum.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


