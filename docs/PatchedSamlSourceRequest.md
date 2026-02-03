# PatchedSamlSourceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Source's display Name. | [optional]
**slug** | Option<**String**> | Internal source name, used in URLs. | [optional]
**enabled** | Option<**bool**> |  | [optional]
**promoted** | Option<**bool**> | When enabled, this source will be displayed as a prominent button on the login page, instead of a small icon. | [optional]
**authentication_flow** | Option<**uuid::Uuid**> | Flow to use when authenticating existing users. | [optional]
**enrollment_flow** | Option<**uuid::Uuid**> | Flow to use when enrolling new users. | [optional]
**user_property_mappings** | Option<**Vec<uuid::Uuid>**> |  | [optional]
**group_property_mappings** | Option<**Vec<uuid::Uuid>**> |  | [optional]
**policy_engine_mode** | Option<[**models::PolicyEngineMode**](PolicyEngineMode.md)> |  | [optional]
**user_matching_mode** | Option<[**models::UserMatchingModeEnum**](UserMatchingModeEnum.md)> | How the source determines if an existing user should be authenticated or a new user enrolled. | [optional]
**user_path_template** | Option<**String**> |  | [optional]
**icon** | Option<**String**> |  | [optional]
**group_matching_mode** | Option<[**models::GroupMatchingModeEnum**](GroupMatchingModeEnum.md)> | How the source determines if an existing group should be used or a new group created. | [optional]
**pre_authentication_flow** | Option<**uuid::Uuid**> | Flow used before authentication. | [optional]
**issuer** | Option<**String**> | Also known as Entity ID. Defaults the Metadata URL. | [optional]
**sso_url** | Option<**String**> | URL that the initial Login request is sent to. | [optional]
**slo_url** | Option<**String**> | Optional URL if your IDP supports Single-Logout. | [optional]
**allow_idp_initiated** | Option<**bool**> | Allows authentication flows initiated by the IdP. This can be a security risk, as no validation of the request ID is done. | [optional]
**name_id_policy** | Option<[**models::SamlNameIdPolicyEnum**](SAMLNameIDPolicyEnum.md)> | NameID Policy sent to the IdP. Can be unset, in which case no Policy is sent. | [optional]
**binding_type** | Option<[**models::BindingTypeEnum**](BindingTypeEnum.md)> |  | [optional]
**verification_kp** | Option<**uuid::Uuid**> | When selected, incoming assertion's Signatures will be validated against this certificate. To allow unsigned Requests, leave on default. | [optional]
**signing_kp** | Option<**uuid::Uuid**> | Keypair used to sign outgoing Responses going to the Identity Provider. | [optional]
**digest_algorithm** | Option<[**models::DigestAlgorithmEnum**](DigestAlgorithmEnum.md)> |  | [optional]
**signature_algorithm** | Option<[**models::SignatureAlgorithmEnum**](SignatureAlgorithmEnum.md)> |  | [optional]
**temporary_user_delete_after** | Option<**String**> | Time offset when temporary users should be deleted. This only applies if your IDP uses the NameID Format 'transient', and the user doesn't log out manually. (Format: hours=1;minutes=2;seconds=3). | [optional]
**encryption_kp** | Option<**uuid::Uuid**> | When selected, incoming assertions are encrypted by the IdP using the public key of the encryption keypair. The assertion is decrypted by the SP using the the private key. | [optional]
**signed_assertion** | Option<**bool**> |  | [optional]
**signed_response** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


