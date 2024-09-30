# LdapProviderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**authentication_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow used for authentication when the associated application is accessed by an un-authenticated user. | [optional]
**authorization_flow** | [**uuid::Uuid**](uuid::Uuid.md) | Flow used when authorizing this provider. | 
**property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**base_dn** | Option<**String**> | DN under which objects are accessible. | [optional]
**certificate** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**tls_server_name** | Option<**String**> |  | [optional]
**uid_start_number** | Option<**i32**> | The start for uidNumbers, this number is added to the user.pk to make sure that the numbers aren't too low for POSIX users. Default is 2000 to ensure that we don't collide with local users uidNumber | [optional]
**gid_start_number** | Option<**i32**> | The start for gidNumbers, this number is added to a number generated from the group.pk to make sure that the numbers aren't too low for POSIX groups. Default is 4000 to ensure that we don't collide with local groups or users primary groups gidNumber | [optional]
**search_mode** | Option<[**models::LdapapiAccessMode**](LDAPAPIAccessMode.md)> |  | [optional]
**bind_mode** | Option<[**models::LdapapiAccessMode**](LDAPAPIAccessMode.md)> |  | [optional]
**mfa_support** | Option<**bool**> | When enabled, code-based multi-factor authentication can be used by appending a semicolon and the TOTP code to the password. This should only be enabled if all users that will bind to this provider have a TOTP device configured, as otherwise a password may incorrectly be rejected if it contains a semicolon. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


