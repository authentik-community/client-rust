# BlueprintInstance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**name** | **String** |  | 
**path** | Option<**String**> |  | [optional][default to ]
**context** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**last_applied** | **String** |  | [readonly]
**last_applied_hash** | **String** |  | [readonly]
**status** | [**models::BlueprintInstanceStatusEnum**](BlueprintInstanceStatusEnum.md) |  | [readonly]
**enabled** | Option<**bool**> |  | [optional]
**managed_models** | **Vec<String>** |  | [readonly]
**metadata** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | [readonly]
**content** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


