# LdapSourceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Source's display Name. | 
**slug** | **String** | Internal source name, used in URLs. | 
**enabled** | Option<**bool**> |  | [optional]
**authentication_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow to use when authenticating existing users. | [optional]
**enrollment_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow to use when enrolling new users. | [optional]
**user_property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**group_property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**policy_engine_mode** | Option<[**models::PolicyEngineMode**](PolicyEngineMode.md)> |  | [optional]
**user_matching_mode** | Option<[**models::UserMatchingModeEnum**](UserMatchingModeEnum.md)> | How the source determines if an existing user should be authenticated or a new user enrolled. | [optional]
**user_path_template** | Option<**String**> |  | [optional]
**server_uri** | **String** |  | 
**peer_certificate** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Optionally verify the LDAP Server's Certificate against the CA Chain in this keypair. | [optional]
**client_certificate** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Client certificate to authenticate against the LDAP Server's Certificate. | [optional]
**bind_cn** | Option<**String**> |  | [optional]
**bind_password** | Option<**String**> |  | [optional]
**start_tls** | Option<**bool**> |  | [optional]
**sni** | Option<**bool**> |  | [optional]
**base_dn** | **String** |  | 
**additional_user_dn** | Option<**String**> | Prepended to Base DN for User-queries. | [optional]
**additional_group_dn** | Option<**String**> | Prepended to Base DN for Group-queries. | [optional]
**user_object_filter** | Option<**String**> | Consider Objects matching this filter to be Users. | [optional]
**group_object_filter** | Option<**String**> | Consider Objects matching this filter to be Groups. | [optional]
**group_membership_field** | Option<**String**> | Field which contains members of a group. | [optional]
**object_uniqueness_field** | Option<**String**> | Field which contains a unique Identifier. | [optional]
**password_login_update_internal_password** | Option<**bool**> | Update internal authentik password when login succeeds with LDAP | [optional]
**sync_users** | Option<**bool**> |  | [optional]
**sync_users_password** | Option<**bool**> | When a user changes their password, sync it back to LDAP. This can only be enabled on a single LDAP source. | [optional]
**sync_groups** | Option<**bool**> |  | [optional]
**sync_parent_group** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


