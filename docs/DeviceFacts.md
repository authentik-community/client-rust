# DeviceFacts

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**os** | Option<[**models::OperatingSystem**](OperatingSystem.md)> |  | [optional]
**disks** | Option<[**Vec<models::Disk>**](Disk.md)> |  | [optional]
**network** | Option<[**models::Network**](Network.md)> |  | [optional]
**hardware** | Option<[**models::Hardware**](Hardware.md)> |  | [optional]
**software** | Option<[**Vec<models::Software>**](Software.md)> |  | [optional]
**processes** | Option<[**Vec<models::Process>**](Process.md)> |  | [optional]
**users** | Option<[**Vec<models::DeviceUser>**](DeviceUser.md)> |  | [optional]
**groups** | Option<[**Vec<models::DeviceGroup>**](DeviceGroup.md)> |  | [optional]
**vendor** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


