# NotificationRuleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**transports** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Select which transports should be used to notify the user. If none are selected, the notification will only be shown in the authentik UI. | [optional]
**severity** | Option<[**models::SeverityEnum**](SeverityEnum.md)> | Controls which severity level the created notifications will have. | [optional]
**destination_group** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Define which group of users this notification should be sent and shown to. If left empty, Notification won't ben sent. | [optional]
**destination_event_user** | Option<**bool**> | When enabled, notification will be sent to user the user that triggered the event.When destination_group is configured, notification is sent to both. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


