# PatchedAuthenticatorSmsStageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**configure_flow** | Option<**uuid::Uuid**> | Flow used by an authenticated user to configure this Stage. If empty, user will not be able to configure this stage. | [optional]
**friendly_name** | Option<**String**> |  | [optional]
**provider** | Option<[**models::ProviderEnum**](ProviderEnum.md)> |  | [optional]
**from_number** | Option<**String**> |  | [optional]
**account_sid** | Option<**String**> |  | [optional]
**auth** | Option<**String**> |  | [optional]
**auth_password** | Option<**String**> |  | [optional]
**auth_type** | Option<[**models::AuthTypeEnum**](AuthTypeEnum.md)> |  | [optional]
**verify_only** | Option<**bool**> | When enabled, the Phone number is only used during enrollment to verify the users authenticity. Only a hash of the phone number is saved to ensure it is not reused in the future. | [optional]
**mapping** | Option<**uuid::Uuid**> | Optionally modify the payload being sent to custom providers. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


