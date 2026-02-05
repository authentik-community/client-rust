# PatchedWsFederationProviderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**authentication_flow** | Option<**uuid::Uuid**> | Flow used for authentication when the associated application is accessed by an un-authenticated user. | [optional]
**authorization_flow** | Option<**uuid::Uuid**> | Flow used when authorizing this provider. | [optional]
**invalidation_flow** | Option<**uuid::Uuid**> | Flow used ending the session from a provider. | [optional]
**property_mappings** | Option<**Vec<uuid::Uuid>**> |  | [optional]
**reply_url** | Option<**String**> |  | [optional]
**wtrealm** | Option<**String**> |  | [optional]
**assertion_valid_not_before** | Option<**String**> | Assertion valid not before current time + this value (Format: hours=-1;minutes=-2;seconds=-3). | [optional]
**assertion_valid_not_on_or_after** | Option<**String**> | Assertion not valid on or after current time + this value (Format: hours=1;minutes=2;seconds=3). | [optional]
**session_valid_not_on_or_after** | Option<**String**> | Session not valid on or after current time + this value (Format: hours=1;minutes=2;seconds=3). | [optional]
**name_id_mapping** | Option<**uuid::Uuid**> | Configure how the NameID value will be created. When left empty, the NameIDPolicy of the incoming request will be considered | [optional]
**authn_context_class_ref_mapping** | Option<**uuid::Uuid**> | Configure how the AuthnContextClassRef value will be created. When left empty, the AuthnContextClassRef will be set based on which authentication methods the user used to authenticate. | [optional]
**digest_algorithm** | Option<[**models::DigestAlgorithmEnum**](DigestAlgorithmEnum.md)> |  | [optional]
**signature_algorithm** | Option<[**models::SignatureAlgorithmEnum**](SignatureAlgorithmEnum.md)> |  | [optional]
**signing_kp** | Option<**uuid::Uuid**> | Keypair used to sign outgoing Responses going to the Service Provider. | [optional]
**encryption_kp** | Option<**uuid::Uuid**> | When selected, incoming assertions are encrypted by the IdP using the public key of the encryption keypair. The assertion is decrypted by the SP using the the private key. | [optional]
**sign_assertion** | Option<**bool**> |  | [optional]
**sign_logout_request** | Option<**bool**> |  | [optional]
**default_name_id_policy** | Option<[**models::SamlNameIdPolicyEnum**](SAMLNameIDPolicyEnum.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


