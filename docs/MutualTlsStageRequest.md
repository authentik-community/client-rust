# MutualTlsStageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**mode** | [**models::StageModeEnum**](StageModeEnum.md) |  | 
**certificate_authorities** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Configure certificate authorities to validate the certificate against. This option has a higher priority than the `client_certificate` option on `Brand`. | [optional]
**cert_attribute** | [**models::CertAttributeEnum**](CertAttributeEnum.md) |  | 
**user_attribute** | [**models::UserAttributeEnum**](UserAttributeEnum.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


