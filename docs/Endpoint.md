# Endpoint

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**name** | **String** |  | 
**provider** | **i32** |  | 
**provider_obj** | [**models::RacProvider**](RACProvider.md) |  | [readonly]
**protocol** | [**models::ProtocolEnum**](ProtocolEnum.md) |  | 
**host** | **String** |  | 
**settings** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**auth_mode** | [**models::AuthModeEnum**](AuthModeEnum.md) |  | 
**launch_url** | Option<**String**> | Build actual launch URL (the provider itself does not have one, just individual endpoints) | [readonly]
**maximum_connections** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


