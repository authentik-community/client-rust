# AuthenticatorValidateStage

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
**not_configured_action** | Option<[**models::NotConfiguredActionEnum**](NotConfiguredActionEnum.md)> |  | [optional]
**device_classes** | Option<[**Vec<models::DeviceClassesEnum>**](DeviceClassesEnum.md)> | Device classes which can be used to authenticate | [optional]
**configuration_stages** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Stages used to configure Authenticator when user doesn't have any compatible devices. After this configuration Stage passes, the user is not prompted again. | [optional]
**last_auth_threshold** | Option<**String**> | If any of the user's device has been used within this threshold, this stage will be skipped | [optional]
**webauthn_user_verification** | Option<[**models::UserVerificationEnum**](UserVerificationEnum.md)> | Enforce user verification for WebAuthn devices. | [optional]
**webauthn_allowed_device_types** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**webauthn_allowed_device_types_obj** | [**Vec<models::WebAuthnDeviceType>**](WebAuthnDeviceType.md) |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


