# AgentConnector

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**connector_uuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**name** | **String** |  | 
**enabled** | Option<**bool**> |  | [optional]
**component** | **String** | Get object component so that we know how to edit the object | [readonly]
**verbose_name** | **String** | Return object's verbose_name | [readonly]
**verbose_name_plural** | **String** | Return object's plural verbose_name | [readonly]
**meta_model_name** | **String** | Return internal model name | [readonly]
**snapshot_expiry** | Option<**String**> |  | [optional]
**auth_session_duration** | Option<**String**> |  | [optional]
**auth_terminate_session_on_expiry** | Option<**bool**> |  | [optional]
**refresh_interval** | Option<**String**> |  | [optional]
**authorization_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**nss_uid_offset** | Option<**u32**> |  | [optional]
**nss_gid_offset** | Option<**u32**> |  | [optional]
**challenge_key** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**challenge_idle_timeout** | Option<**String**> |  | [optional]
**challenge_trigger_check_in** | Option<**bool**> |  | [optional]
**jwt_federation_providers** | Option<**Vec<i32>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


