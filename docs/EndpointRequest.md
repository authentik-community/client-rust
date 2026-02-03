# EndpointRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**provider** | **i32** |  | 
**protocol** | [**models::ProtocolEnum**](ProtocolEnum.md) |  | 
**host** | **String** |  | 
**settings** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**property_mappings** | Option<**Vec<uuid::Uuid>**> |  | [optional]
**auth_mode** | [**models::EndpointAuthModeEnum**](EndpointAuthModeEnum.md) |  | 
**maximum_connections** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


