# OAuth2Provider

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**name** | **String** |  | 
**authentication_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow used for authentication when the associated application is accessed by an un-authenticated user. | [optional]
**authorization_flow** | [**uuid::Uuid**](uuid::Uuid.md) | Flow used when authorizing this provider. | 
**invalidation_flow** | [**uuid::Uuid**](uuid::Uuid.md) | Flow used ending the session from a provider. | 
**property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**component** | **String** | Get object component so that we know how to edit the object | [readonly]
**assigned_application_slug** | **String** | Internal application name, used in URLs. | [readonly]
**assigned_application_name** | **String** | Application's display Name. | [readonly]
**assigned_backchannel_application_slug** | **String** | Internal application name, used in URLs. | [readonly]
**assigned_backchannel_application_name** | **String** | Application's display Name. | [readonly]
**verbose_name** | **String** | Return object's verbose_name | [readonly]
**verbose_name_plural** | **String** | Return object's plural verbose_name | [readonly]
**meta_model_name** | **String** | Return internal model name | [readonly]
**client_type** | Option<[**models::ClientTypeEnum**](ClientTypeEnum.md)> | Confidential clients are capable of maintaining the confidentiality of their credentials. Public clients are incapable | [optional]
**client_id** | Option<**String**> |  | [optional]
**client_secret** | Option<**String**> |  | [optional]
**access_code_validity** | Option<**String**> | Access codes not valid on or after current time + this value (Format: hours=1;minutes=2;seconds=3). | [optional]
**access_token_validity** | Option<**String**> | Tokens not valid on or after current time + this value (Format: hours=1;minutes=2;seconds=3). | [optional]
**refresh_token_validity** | Option<**String**> | Tokens not valid on or after current time + this value (Format: hours=1;minutes=2;seconds=3). | [optional]
**refresh_token_threshold** | Option<**String**> | When refreshing a token, if the refresh token is valid for less than this duration, it will be renewed. When set to seconds=0, token will always be renewed. (Format: hours=1;minutes=2;seconds=3). | [optional]
**include_claims_in_id_token** | Option<**bool**> | Include User claims from scopes in the id_token, for applications that don't access the userinfo endpoint. | [optional]
**signing_key** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Key used to sign the tokens. | [optional]
**encryption_key** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Key used to encrypt the tokens. When set, tokens will be encrypted and returned as JWEs. | [optional]
**redirect_uris** | [**Vec<models::RedirectUri>**](RedirectURI.md) |  | 
**logout_uri** | Option<**String**> |  | [optional]
**logout_method** | Option<[**models::OAuth2ProviderLogoutMethodEnum**](OAuth2ProviderLogoutMethodEnum.md)> | Backchannel logs out with server to server calls. Frontchannel uses iframes in your browser | [optional]
**sub_mode** | Option<[**models::SubModeEnum**](SubModeEnum.md)> | Configure what data should be used as unique User Identifier. For most cases, the default should be fine. | [optional]
**issuer_mode** | Option<[**models::IssuerModeEnum**](IssuerModeEnum.md)> | Configure how the issuer field of the ID Token should be filled. | [optional]
**jwt_federation_sources** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**jwt_federation_providers** | Option<**Vec<i32>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


