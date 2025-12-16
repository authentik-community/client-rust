# AgentConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**device_id** | **String** |  | [readonly]
**refresh_interval** | **i32** |  | [readonly]
**authorization_flow** | Option<**String**> |  | [readonly]
**jwks_auth** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | [readonly]
**jwks_challenge** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [readonly]
**nss_uid_offset** | **i32** |  | 
**nss_gid_offset** | **i32** |  | 
**auth_terminate_session_on_expiry** | **bool** |  | 
**system_config** | [**models::Config**](Config.md) |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


