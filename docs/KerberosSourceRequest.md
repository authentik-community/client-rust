# KerberosSourceRequest

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
**group_matching_mode** | Option<[**models::GroupMatchingModeEnum**](GroupMatchingModeEnum.md)> | How the source determines if an existing group should be used or a new group created. | [optional]
**realm** | **String** | Kerberos realm | 
**krb5_conf** | Option<**String**> | Custom krb5.conf to use. Uses the system one by default | [optional]
**kadmin_type** | Option<[**models::KadminTypeEnum**](KadminTypeEnum.md)> | KAdmin server type | [optional]
**sync_users** | Option<**bool**> | Sync users from Kerberos into authentik | [optional]
**sync_users_password** | Option<**bool**> | When a user changes their password, sync it back to Kerberos | [optional]
**sync_principal** | Option<**String**> | Principal to authenticate to kadmin for sync. | [optional]
**sync_password** | Option<**String**> | Password to authenticate to kadmin for sync | [optional]
**sync_keytab** | Option<**String**> | Keytab to authenticate to kadmin for sync. Must be base64-encoded or in the form TYPE:residual | [optional]
**sync_ccache** | Option<**String**> | Credentials cache to authenticate to kadmin for sync. Must be in the form TYPE:residual | [optional]
**spnego_server_name** | Option<**String**> | Force the use of a specific server name for SPNEGO. Must be in the form HTTP@hostname | [optional]
**spnego_keytab** | Option<**String**> | SPNEGO keytab base64-encoded or path to keytab in the form FILE:path | [optional]
**spnego_ccache** | Option<**String**> | Credential cache to use for SPNEGO in form type:residual | [optional]
**password_login_update_internal_password** | Option<**bool**> | If enabled, the authentik-stored password will be updated upon login with the Kerberos password backend | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


