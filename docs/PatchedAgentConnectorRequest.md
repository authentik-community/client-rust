# PatchedAgentConnectorRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**connector_uuid** | Option<**uuid::Uuid**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**snapshot_expiry** | Option<**String**> |  | [optional]
**auth_session_duration** | Option<**String**> |  | [optional]
**auth_terminate_session_on_expiry** | Option<**bool**> |  | [optional]
**refresh_interval** | Option<**String**> |  | [optional]
**authorization_flow** | Option<**uuid::Uuid**> |  | [optional]
**nss_uid_offset** | Option<**u32**> |  | [optional]
**nss_gid_offset** | Option<**u32**> |  | [optional]
**challenge_key** | Option<**uuid::Uuid**> |  | [optional]
**challenge_idle_timeout** | Option<**String**> |  | [optional]
**challenge_trigger_check_in** | Option<**bool**> |  | [optional]
**jwt_federation_providers** | Option<**Vec<i32>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


