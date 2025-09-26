# SamlSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**name** | **String** | Source's display Name. | 
**slug** | **String** | Internal source name, used in URLs. | 
**enabled** | Option<**bool**> |  | [optional]
**authentication_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow to use when authenticating existing users. | [optional]
**enrollment_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow to use when enrolling new users. | [optional]
**user_property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**group_property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**component** | **String** | Get object component so that we know how to edit the object | [readonly]
**verbose_name** | **String** | Return object's verbose_name | [readonly]
**verbose_name_plural** | **String** | Return object's plural verbose_name | [readonly]
**meta_model_name** | **String** | Return internal model name | [readonly]
**policy_engine_mode** | Option<[**models::PolicyEngineMode**](PolicyEngineMode.md)> |  | [optional]
**user_matching_mode** | Option<[**models::UserMatchingModeEnum**](UserMatchingModeEnum.md)> | How the source determines if an existing user should be authenticated or a new user enrolled. | [optional]
**managed** | Option<**String**> | Objects that are managed by authentik. These objects are created and updated automatically. This flag only indicates that an object can be overwritten by migrations. You can still modify the objects via the API, but expect changes to be overwritten in a later update. | [readonly]
**user_path_template** | Option<**String**> |  | [optional]
**icon** | **String** |  | [readonly]
**group_matching_mode** | Option<[**models::GroupMatchingModeEnum**](GroupMatchingModeEnum.md)> | How the source determines if an existing group should be used or a new group created. | [optional]
**pre_authentication_flow** | [**uuid::Uuid**](uuid::Uuid.md) | Flow used before authentication. | 
**issuer** | Option<**String**> | Also known as Entity ID. Defaults the Metadata URL. | [optional]
**sso_url** | **String** | URL that the initial Login request is sent to. | 
**slo_url** | Option<**String**> | Optional URL if your IDP supports Single-Logout. | [optional]
**allow_idp_initiated** | Option<**bool**> | Allows authentication flows initiated by the IdP. This can be a security risk, as no validation of the request ID is done. | [optional]
**name_id_policy** | Option<[**models::SamlNameIdPolicyEnum**](SAMLNameIDPolicyEnum.md)> | NameID Policy sent to the IdP. Can be unset, in which case no Policy is sent. | [optional]
**binding_type** | Option<[**models::BindingTypeEnum**](BindingTypeEnum.md)> |  | [optional]
**verification_kp** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | When selected, incoming assertion's Signatures will be validated against this certificate. To allow unsigned Requests, leave on default. | [optional]
**signing_kp** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Keypair used to sign outgoing Responses going to the Identity Provider. | [optional]
**digest_algorithm** | Option<[**models::DigestAlgorithmEnum**](DigestAlgorithmEnum.md)> |  | [optional]
**signature_algorithm** | Option<[**models::SignatureAlgorithmEnum**](SignatureAlgorithmEnum.md)> |  | [optional]
**temporary_user_delete_after** | Option<**String**> | Time offset when temporary users should be deleted. This only applies if your IDP uses the NameID Format 'transient', and the user doesn't log out manually. (Format: hours=1;minutes=2;seconds=3). | [optional]
**encryption_kp** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | When selected, incoming assertions are encrypted by the IdP using the public key of the encryption keypair. The assertion is decrypted by the SP using the the private key. | [optional]
**signed_assertion** | Option<**bool**> |  | [optional]
**signed_response** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


