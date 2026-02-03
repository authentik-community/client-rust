# TelegramSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **uuid::Uuid** |  | [readonly]
**name** | **String** | Source's display Name. | 
**slug** | **String** | Internal source name, used in URLs. | 
**enabled** | Option<**bool**> |  | [optional]
**promoted** | Option<**bool**> | When enabled, this source will be displayed as a prominent button on the login page, instead of a small icon. | [optional]
**authentication_flow** | Option<**uuid::Uuid**> | Flow to use when authenticating existing users. | [optional]
**enrollment_flow** | Option<**uuid::Uuid**> | Flow to use when enrolling new users. | [optional]
**user_property_mappings** | Option<**Vec<uuid::Uuid>**> |  | [optional]
**group_property_mappings** | Option<**Vec<uuid::Uuid>**> |  | [optional]
**component** | **String** | Get object component so that we know how to edit the object | [readonly]
**verbose_name** | **String** | Return object's verbose_name | [readonly]
**verbose_name_plural** | **String** | Return object's plural verbose_name | [readonly]
**meta_model_name** | **String** | Return internal model name | [readonly]
**policy_engine_mode** | Option<[**models::PolicyEngineMode**](PolicyEngineMode.md)> |  | [optional]
**user_matching_mode** | Option<[**models::UserMatchingModeEnum**](UserMatchingModeEnum.md)> | How the source determines if an existing user should be authenticated or a new user enrolled. | [optional]
**managed** | Option<**String**> | Objects that are managed by authentik. These objects are created and updated automatically. This flag only indicates that an object can be overwritten by migrations. You can still modify the objects via the API, but expect changes to be overwritten in a later update. | [readonly]
**user_path_template** | Option<**String**> |  | [optional]
**icon** | Option<**String**> |  | [optional]
**icon_url** | Option<**String**> |  | [readonly]
**icon_themed_urls** | Option<[**models::ThemedUrls**](ThemedUrls.md)> |  | [readonly]
**bot_username** | **String** | Telegram bot username | 
**request_message_access** | Option<**bool**> | Request access to send messages from your bot. | [optional]
**pre_authentication_flow** | **uuid::Uuid** | Flow used before authentication. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


