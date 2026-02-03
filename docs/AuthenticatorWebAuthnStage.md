# AuthenticatorWebAuthnStage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **uuid::Uuid** |  | [readonly]
**name** | **String** |  | 
**component** | **String** | Get object type so that we know how to edit the object | [readonly]
**verbose_name** | **String** | Return object's verbose_name | [readonly]
**verbose_name_plural** | **String** | Return object's plural verbose_name | [readonly]
**meta_model_name** | **String** | Return internal model name | [readonly]
**flow_set** | [**Vec<models::FlowSet>**](FlowSet.md) |  | [readonly]
**configure_flow** | Option<**uuid::Uuid**> | Flow used by an authenticated user to configure this Stage. If empty, user will not be able to configure this stage. | [optional]
**friendly_name** | Option<**String**> |  | [optional]
**user_verification** | Option<[**models::UserVerificationEnum**](UserVerificationEnum.md)> |  | [optional]
**authenticator_attachment** | Option<[**models::AuthenticatorAttachmentEnum**](AuthenticatorAttachmentEnum.md)> |  | [optional]
**resident_key_requirement** | Option<[**models::ResidentKeyRequirementEnum**](ResidentKeyRequirementEnum.md)> |  | [optional]
**device_type_restrictions** | Option<**Vec<uuid::Uuid>**> |  | [optional]
**device_type_restrictions_obj** | [**Vec<models::WebAuthnDeviceType>**](WebAuthnDeviceType.md) |  | [readonly]
**max_attempts** | Option<**u32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


