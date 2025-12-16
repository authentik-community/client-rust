# AuthenticatorSmsStage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**name** | **String** |  | 
**component** | **String** | Get object type so that we know how to edit the object | [readonly]
**verbose_name** | **String** | Return object's verbose_name | [readonly]
**verbose_name_plural** | **String** | Return object's plural verbose_name | [readonly]
**meta_model_name** | **String** | Return internal model name | [readonly]
**flow_set** | [**Vec<models::FlowSet>**](FlowSet.md) |  | [readonly]
**configure_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow used by an authenticated user to configure this Stage. If empty, user will not be able to configure this stage. | [optional]
**friendly_name** | Option<**String**> |  | [optional]
**provider** | [**models::ProviderEnum**](ProviderEnum.md) |  | 
**from_number** | **String** |  | 
**account_sid** | **String** |  | 
**auth** | **String** |  | 
**auth_password** | Option<**String**> |  | [optional]
**auth_type** | Option<[**models::AuthTypeEnum**](AuthTypeEnum.md)> |  | [optional]
**verify_only** | Option<**bool**> | When enabled, the Phone number is only used during enrollment to verify the users authenticity. Only a hash of the phone number is saved to ensure it is not reused in the future. | [optional]
**mapping** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Optionally modify the payload being sent to custom providers. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


