# IdentificationStageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**user_fields** | Option<[**Vec<models::UserFieldsEnum>**](UserFieldsEnum.md)> | Fields of the user object to match against. (Hold shift to select multiple options) | [optional]
**password_stage** | Option<**uuid::Uuid**> | When set, shows a password field, instead of showing the password field as separate step. | [optional]
**captcha_stage** | Option<**uuid::Uuid**> | When set, adds functionality exactly like a Captcha stage, but baked into the Identification stage. | [optional]
**case_insensitive_matching** | Option<**bool**> | When enabled, user fields are matched regardless of their casing. | [optional]
**show_matched_user** | Option<**bool**> | When a valid username/email has been entered, and this option is enabled, the user's username and avatar will be shown. Otherwise, the text that the user entered will be shown | [optional]
**enrollment_flow** | Option<**uuid::Uuid**> | Optional enrollment flow, which is linked at the bottom of the page. | [optional]
**recovery_flow** | Option<**uuid::Uuid**> | Optional recovery flow, which is linked at the bottom of the page. | [optional]
**passwordless_flow** | Option<**uuid::Uuid**> | Optional passwordless flow, which is linked at the bottom of the page. | [optional]
**sources** | Option<**Vec<uuid::Uuid>**> | Specify which sources should be shown. | [optional]
**show_source_labels** | Option<**bool**> |  | [optional]
**pretend_user_exists** | Option<**bool**> | When enabled, the stage will succeed and continue even when incorrect user info is entered. | [optional]
**enable_remember_me** | Option<**bool**> | Show the user the 'Remember me on this device' toggle, allowing repeat users to skip straight to entering their password. | [optional]
**webauthn_stage** | Option<**uuid::Uuid**> | When set, and conditional WebAuthn is available, allow the user to use their passkey as a first factor. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


