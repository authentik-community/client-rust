# PatchedNotificationRuleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**transports** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Select which transports should be used to notify the user. If none are selected, the notification will only be shown in the authentik UI. | [optional]
**severity** | Option<[**models::SeverityEnum**](SeverityEnum.md)> | Controls which severity level the created notifications will have. | [optional]
**group** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Define which group of users this notification should be sent and shown to. If left empty, Notification won't ben sent. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


