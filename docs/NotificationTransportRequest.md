# NotificationTransportRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**mode** | Option<[**models::NotificationTransportModeEnum**](NotificationTransportModeEnum.md)> |  | [optional]
**webhook_url** | Option<**String**> |  | [optional]
**webhook_mapping** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**send_once** | Option<**bool**> | Only send notification once, for example when sending a webhook into a chat channel. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


