# EndpointDevice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**device_uuid** | Option<**uuid::Uuid**> |  | [optional]
**pbm_uuid** | **uuid::Uuid** |  | [readonly]
**name** | **String** |  | 
**access_group** | Option<**uuid::Uuid**> |  | [optional]
**access_group_obj** | Option<[**models::DeviceAccessGroup**](DeviceAccessGroup.md)> |  | [optional]
**expiring** | Option<**bool**> |  | [optional]
**expires** | Option<**String**> |  | [optional]
**facts** | [**models::DeviceFactSnapshot**](DeviceFactSnapshot.md) |  | [readonly]
**attributes** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


