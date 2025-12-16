# PatchedTelegramSourceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Source's display Name. | [optional]
**slug** | Option<**String**> | Internal source name, used in URLs. | [optional]
**enabled** | Option<**bool**> |  | [optional]
**promoted** | Option<**bool**> | When enabled, this source will be displayed as a prominent button on the login page, instead of a small icon. | [optional]
**authentication_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow to use when authenticating existing users. | [optional]
**enrollment_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow to use when enrolling new users. | [optional]
**user_property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**group_property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**policy_engine_mode** | Option<[**models::PolicyEngineMode**](PolicyEngineMode.md)> |  | [optional]
**user_matching_mode** | Option<[**models::UserMatchingModeEnum**](UserMatchingModeEnum.md)> | How the source determines if an existing user should be authenticated or a new user enrolled. | [optional]
**user_path_template** | Option<**String**> |  | [optional]
**icon** | Option<**String**> |  | [optional]
**bot_username** | Option<**String**> | Telegram bot username | [optional]
**bot_token** | Option<**String**> | Telegram bot token | [optional]
**request_message_access** | Option<**bool**> | Request access to send messages from your bot. | [optional]
**pre_authentication_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow used before authentication. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


