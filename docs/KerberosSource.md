# KerberosSource

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
**realm** | **String** | Kerberos realm | 
**krb5_conf** | Option<**String**> | Custom krb5.conf to use. Uses the system one by default | [optional]
**sync_users** | Option<**bool**> | Sync users from Kerberos into authentik | [optional]
**sync_users_password** | Option<**bool**> | When a user changes their password, sync it back to Kerberos | [optional]
**sync_principal** | Option<**String**> | Principal to authenticate to kadmin for sync. | [optional]
**sync_ccache** | Option<**String**> | Credentials cache to authenticate to kadmin for sync. Must be in the form TYPE:residual | [optional]
**connectivity** | Option<**std::collections::HashMap<String, String>**> | Get cached source connectivity | [readonly]
**spnego_server_name** | Option<**String**> | Force the use of a specific server name for SPNEGO. Must be in the form HTTP@hostname | [optional]
**spnego_ccache** | Option<**String**> | Credential cache to use for SPNEGO in form type:residual | [optional]
**password_login_update_internal_password** | Option<**bool**> | If enabled, the authentik-stored password will be updated upon login with the Kerberos password backend | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


