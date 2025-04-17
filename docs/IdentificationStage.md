# IdentificationStage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**name** | **String** |  | 
**component** | **String** | Get object type so that we know how to edit the object | [readonly]
**verbose_name** | **String** | Return object's verbose_name | [readonly]
**verbose_name_plural** | **String** | Return object's plural verbose_name | [readonly]
**meta_model_name** | **String** | Return internal model name | [readonly]
**flow_set** | Option<[**Vec<models::FlowSet>**](FlowSet.md)> |  | [optional]
**user_fields** | Option<[**Vec<models::UserFieldsEnum>**](UserFieldsEnum.md)> | Fields of the user object to match against. (Hold shift to select multiple options) | [optional]
**password_stage** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | When set, shows a password field, instead of showing the password field as separate step. | [optional]
**captcha_stage** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | When set, adds functionality exactly like a Captcha stage, but baked into the Identification stage. | [optional]
**case_insensitive_matching** | Option<**bool**> | When enabled, user fields are matched regardless of their casing. | [optional]
**show_matched_user** | Option<**bool**> | When a valid username/email has been entered, and this option is enabled, the user's username and avatar will be shown. Otherwise, the text that the user entered will be shown | [optional]
**enrollment_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Optional enrollment flow, which is linked at the bottom of the page. | [optional]
**recovery_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Optional recovery flow, which is linked at the bottom of the page. | [optional]
**passwordless_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Optional passwordless flow, which is linked at the bottom of the page. | [optional]
**sources** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Specify which sources should be shown. | [optional]
**show_source_labels** | Option<**bool**> |  | [optional]
**pretend_user_exists** | Option<**bool**> | When enabled, the stage will succeed and continue even when incorrect user info is entered. | [optional]
**enable_remember_me** | Option<**bool**> | Show the user the 'Remember me on this device' toggle, allowing repeat users to skip straight to entering their password. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


