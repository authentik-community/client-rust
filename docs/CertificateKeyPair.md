# CertificateKeyPair

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **uuid::Uuid** |  | [readonly]
**name** | **String** |  | 
**fingerprint_sha256** | Option<**String**> | SHA256 fingerprint of the certificate | [readonly]
**fingerprint_sha1** | Option<**String**> | SHA1 fingerprint of the certificate | [readonly]
**cert_expiry** | Option<**String**> | Certificate expiry date | [readonly]
**cert_subject** | Option<**String**> | Certificate subject as RFC4514 string | [readonly]
**private_key_available** | **bool** | Show if this keypair has a private key configured or not | [readonly]
**key_type** | Option<[**models::KeyTypeEnum**](KeyTypeEnum.md)> | Key algorithm type detected from the certificate's public key | [readonly]
**certificate_download_url** | **String** | Get URL to download certificate | [readonly]
**private_key_download_url** | **String** | Get URL to download private key | [readonly]
**managed** | Option<**String**> | Objects that are managed by authentik. These objects are created and updated automatically. This flag only indicates that an object can be overwritten by migrations. You can still modify the objects via the API, but expect changes to be overwritten in a later update. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


