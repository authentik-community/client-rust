# RadiusProviderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**authentication_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow used for authentication when the associated application is accessed by an un-authenticated user. | [optional]
**authorization_flow** | [**uuid::Uuid**](uuid::Uuid.md) | Flow used when authorizing this provider. | 
**invalidation_flow** | [**uuid::Uuid**](uuid::Uuid.md) | Flow used ending the session from a provider. | 
**property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**client_networks** | Option<**String**> | List of CIDRs (comma-separated) that clients can connect from. A more specific CIDR will match before a looser one. Clients connecting from a non-specified CIDR will be dropped. | [optional]
**shared_secret** | Option<**String**> | Shared secret between clients and server to hash packets. | [optional]
**mfa_support** | Option<**bool**> | When enabled, code-based multi-factor authentication can be used by appending a semicolon and the TOTP code to the password. This should only be enabled if all users that will bind to this provider have a TOTP device configured, as otherwise a password may incorrectly be rejected if it contains a semicolon. | [optional]
**certificate** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


