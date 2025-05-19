# PatchedMutualTlsStageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**flow_set** | Option<[**Vec<models::FlowSetRequest>**](FlowSetRequest.md)> |  | [optional]
**mode** | Option<[**models::MutualTlsStageModeEnum**](MutualTLSStageModeEnum.md)> |  | [optional]
**certificate_authorities** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Configure certificate authorities to validate the certificate against. This option has a higher priority than the `client_certificate` option on `Brand`. | [optional]
**cert_attribute** | Option<[**models::CertAttributeEnum**](CertAttributeEnum.md)> |  | [optional]
**user_attribute** | Option<[**models::UserAttributeEnum**](UserAttributeEnum.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


