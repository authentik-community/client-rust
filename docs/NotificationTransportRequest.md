# NotificationTransportRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**mode** | Option<[**models::NotificationTransportModeEnum**](NotificationTransportModeEnum.md)> |  | [optional]
**webhook_url** | Option<**String**> |  | [optional]
**webhook_mapping_body** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Customize the body of the request. Mapping should return data that is JSON-serializable. | [optional]
**webhook_mapping_headers** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Configure additional headers to be sent. Mapping should return a dictionary of key-value pairs | [optional]
**email_subject_prefix** | Option<**String**> |  | [optional]
**email_template** | Option<**String**> |  | [optional]
**send_once** | Option<**bool**> | Only send notification once, for example when sending a webhook into a chat channel. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


