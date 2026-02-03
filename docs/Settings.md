# Settings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**avatars** | Option<**String**> | Configure how authentik should show avatars for users. | [optional]
**default_user_change_name** | Option<**bool**> | Enable the ability for users to change their name. | [optional]
**default_user_change_email** | Option<**bool**> | Enable the ability for users to change their email address. | [optional]
**default_user_change_username** | Option<**bool**> | Enable the ability for users to change their username. | [optional]
**event_retention** | Option<**String**> | Events will be deleted after this duration.(Format: weeks=3;days=2;hours=3,seconds=2). | [optional]
**reputation_lower_limit** | Option<**i32**> | Reputation cannot decrease lower than this value. Zero or negative. | [optional]
**reputation_upper_limit** | Option<**u32**> | Reputation cannot increase higher than this value. Zero or positive. | [optional]
**footer_links** | Option<**serde_json::Value**> |  | [optional]
**gdpr_compliance** | Option<**bool**> | When enabled, all the events caused by a user will be deleted upon the user's deletion. | [optional]
**impersonation** | Option<**bool**> | Globally enable/disable impersonation. | [optional]
**impersonation_require_reason** | Option<**bool**> | Require administrators to provide a reason for impersonating a user. | [optional]
**default_token_duration** | Option<**String**> | Default token duration | [optional]
**default_token_length** | Option<**u32**> | Default token length | [optional]
**pagination_default_page_size** | Option<**u32**> | Default page size for API responses, if no size was requested. | [optional]
**pagination_max_page_size** | Option<**u32**> | Maximum page size | [optional]
**flags** | [**models::PatchedSettingsRequestFlags**](PatchedSettingsRequestFlags.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


