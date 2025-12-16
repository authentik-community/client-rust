# MutualTlsStage

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
**mode** | [**models::StageModeEnum**](StageModeEnum.md) |  | 
**certificate_authorities** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Configure certificate authorities to validate the certificate against. This option has a higher priority than the `client_certificate` option on `Brand`. | [optional]
**cert_attribute** | [**models::CertAttributeEnum**](CertAttributeEnum.md) |  | 
**user_attribute** | [**models::UserAttributeEnum**](UserAttributeEnum.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


