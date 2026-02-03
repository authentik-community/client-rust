# DeviceFactsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**os** | Option<[**models::OperatingSystemRequest**](OperatingSystemRequest.md)> |  | [optional]
**disks** | Option<[**Vec<models::DiskRequest>**](DiskRequest.md)> |  | [optional]
**network** | Option<[**models::NetworkRequest**](NetworkRequest.md)> |  | [optional]
**hardware** | Option<[**models::HardwareRequest**](HardwareRequest.md)> |  | [optional]
**software** | Option<[**Vec<models::SoftwareRequest>**](SoftwareRequest.md)> |  | [optional]
**processes** | Option<[**Vec<models::ProcessRequest>**](ProcessRequest.md)> |  | [optional]
**users** | Option<[**Vec<models::DeviceUserRequest>**](DeviceUserRequest.md)> |  | [optional]
**groups** | Option<[**Vec<models::DeviceGroupRequest>**](DeviceGroupRequest.md)> |  | [optional]
**vendor** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


