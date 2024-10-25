# \StagesApi

All URIs are relative to *http://localhost/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**stages_all_destroy**](StagesApi.md#stages_all_destroy) | **DELETE** /stages/all/{stage_uuid}/ | 
[**stages_all_list**](StagesApi.md#stages_all_list) | **GET** /stages/all/ | 
[**stages_all_retrieve**](StagesApi.md#stages_all_retrieve) | **GET** /stages/all/{stage_uuid}/ | 
[**stages_all_types_list**](StagesApi.md#stages_all_types_list) | **GET** /stages/all/types/ | 
[**stages_all_used_by_list**](StagesApi.md#stages_all_used_by_list) | **GET** /stages/all/{stage_uuid}/used_by/ | 
[**stages_all_user_settings_list**](StagesApi.md#stages_all_user_settings_list) | **GET** /stages/all/user_settings/ | 
[**stages_authenticator_duo_create**](StagesApi.md#stages_authenticator_duo_create) | **POST** /stages/authenticator/duo/ | 
[**stages_authenticator_duo_destroy**](StagesApi.md#stages_authenticator_duo_destroy) | **DELETE** /stages/authenticator/duo/{stage_uuid}/ | 
[**stages_authenticator_duo_enrollment_status_create**](StagesApi.md#stages_authenticator_duo_enrollment_status_create) | **POST** /stages/authenticator/duo/{stage_uuid}/enrollment_status/ | 
[**stages_authenticator_duo_import_device_manual_create**](StagesApi.md#stages_authenticator_duo_import_device_manual_create) | **POST** /stages/authenticator/duo/{stage_uuid}/import_device_manual/ | 
[**stages_authenticator_duo_import_devices_automatic_create**](StagesApi.md#stages_authenticator_duo_import_devices_automatic_create) | **POST** /stages/authenticator/duo/{stage_uuid}/import_devices_automatic/ | 
[**stages_authenticator_duo_list**](StagesApi.md#stages_authenticator_duo_list) | **GET** /stages/authenticator/duo/ | 
[**stages_authenticator_duo_partial_update**](StagesApi.md#stages_authenticator_duo_partial_update) | **PATCH** /stages/authenticator/duo/{stage_uuid}/ | 
[**stages_authenticator_duo_retrieve**](StagesApi.md#stages_authenticator_duo_retrieve) | **GET** /stages/authenticator/duo/{stage_uuid}/ | 
[**stages_authenticator_duo_update**](StagesApi.md#stages_authenticator_duo_update) | **PUT** /stages/authenticator/duo/{stage_uuid}/ | 
[**stages_authenticator_duo_used_by_list**](StagesApi.md#stages_authenticator_duo_used_by_list) | **GET** /stages/authenticator/duo/{stage_uuid}/used_by/ | 
[**stages_authenticator_endpoint_gdtc_create**](StagesApi.md#stages_authenticator_endpoint_gdtc_create) | **POST** /stages/authenticator/endpoint_gdtc/ | 
[**stages_authenticator_endpoint_gdtc_destroy**](StagesApi.md#stages_authenticator_endpoint_gdtc_destroy) | **DELETE** /stages/authenticator/endpoint_gdtc/{stage_uuid}/ | 
[**stages_authenticator_endpoint_gdtc_list**](StagesApi.md#stages_authenticator_endpoint_gdtc_list) | **GET** /stages/authenticator/endpoint_gdtc/ | 
[**stages_authenticator_endpoint_gdtc_partial_update**](StagesApi.md#stages_authenticator_endpoint_gdtc_partial_update) | **PATCH** /stages/authenticator/endpoint_gdtc/{stage_uuid}/ | 
[**stages_authenticator_endpoint_gdtc_retrieve**](StagesApi.md#stages_authenticator_endpoint_gdtc_retrieve) | **GET** /stages/authenticator/endpoint_gdtc/{stage_uuid}/ | 
[**stages_authenticator_endpoint_gdtc_update**](StagesApi.md#stages_authenticator_endpoint_gdtc_update) | **PUT** /stages/authenticator/endpoint_gdtc/{stage_uuid}/ | 
[**stages_authenticator_endpoint_gdtc_used_by_list**](StagesApi.md#stages_authenticator_endpoint_gdtc_used_by_list) | **GET** /stages/authenticator/endpoint_gdtc/{stage_uuid}/used_by/ | 
[**stages_authenticator_sms_create**](StagesApi.md#stages_authenticator_sms_create) | **POST** /stages/authenticator/sms/ | 
[**stages_authenticator_sms_destroy**](StagesApi.md#stages_authenticator_sms_destroy) | **DELETE** /stages/authenticator/sms/{stage_uuid}/ | 
[**stages_authenticator_sms_list**](StagesApi.md#stages_authenticator_sms_list) | **GET** /stages/authenticator/sms/ | 
[**stages_authenticator_sms_partial_update**](StagesApi.md#stages_authenticator_sms_partial_update) | **PATCH** /stages/authenticator/sms/{stage_uuid}/ | 
[**stages_authenticator_sms_retrieve**](StagesApi.md#stages_authenticator_sms_retrieve) | **GET** /stages/authenticator/sms/{stage_uuid}/ | 
[**stages_authenticator_sms_update**](StagesApi.md#stages_authenticator_sms_update) | **PUT** /stages/authenticator/sms/{stage_uuid}/ | 
[**stages_authenticator_sms_used_by_list**](StagesApi.md#stages_authenticator_sms_used_by_list) | **GET** /stages/authenticator/sms/{stage_uuid}/used_by/ | 
[**stages_authenticator_static_create**](StagesApi.md#stages_authenticator_static_create) | **POST** /stages/authenticator/static/ | 
[**stages_authenticator_static_destroy**](StagesApi.md#stages_authenticator_static_destroy) | **DELETE** /stages/authenticator/static/{stage_uuid}/ | 
[**stages_authenticator_static_list**](StagesApi.md#stages_authenticator_static_list) | **GET** /stages/authenticator/static/ | 
[**stages_authenticator_static_partial_update**](StagesApi.md#stages_authenticator_static_partial_update) | **PATCH** /stages/authenticator/static/{stage_uuid}/ | 
[**stages_authenticator_static_retrieve**](StagesApi.md#stages_authenticator_static_retrieve) | **GET** /stages/authenticator/static/{stage_uuid}/ | 
[**stages_authenticator_static_update**](StagesApi.md#stages_authenticator_static_update) | **PUT** /stages/authenticator/static/{stage_uuid}/ | 
[**stages_authenticator_static_used_by_list**](StagesApi.md#stages_authenticator_static_used_by_list) | **GET** /stages/authenticator/static/{stage_uuid}/used_by/ | 
[**stages_authenticator_totp_create**](StagesApi.md#stages_authenticator_totp_create) | **POST** /stages/authenticator/totp/ | 
[**stages_authenticator_totp_destroy**](StagesApi.md#stages_authenticator_totp_destroy) | **DELETE** /stages/authenticator/totp/{stage_uuid}/ | 
[**stages_authenticator_totp_list**](StagesApi.md#stages_authenticator_totp_list) | **GET** /stages/authenticator/totp/ | 
[**stages_authenticator_totp_partial_update**](StagesApi.md#stages_authenticator_totp_partial_update) | **PATCH** /stages/authenticator/totp/{stage_uuid}/ | 
[**stages_authenticator_totp_retrieve**](StagesApi.md#stages_authenticator_totp_retrieve) | **GET** /stages/authenticator/totp/{stage_uuid}/ | 
[**stages_authenticator_totp_update**](StagesApi.md#stages_authenticator_totp_update) | **PUT** /stages/authenticator/totp/{stage_uuid}/ | 
[**stages_authenticator_totp_used_by_list**](StagesApi.md#stages_authenticator_totp_used_by_list) | **GET** /stages/authenticator/totp/{stage_uuid}/used_by/ | 
[**stages_authenticator_validate_create**](StagesApi.md#stages_authenticator_validate_create) | **POST** /stages/authenticator/validate/ | 
[**stages_authenticator_validate_destroy**](StagesApi.md#stages_authenticator_validate_destroy) | **DELETE** /stages/authenticator/validate/{stage_uuid}/ | 
[**stages_authenticator_validate_list**](StagesApi.md#stages_authenticator_validate_list) | **GET** /stages/authenticator/validate/ | 
[**stages_authenticator_validate_partial_update**](StagesApi.md#stages_authenticator_validate_partial_update) | **PATCH** /stages/authenticator/validate/{stage_uuid}/ | 
[**stages_authenticator_validate_retrieve**](StagesApi.md#stages_authenticator_validate_retrieve) | **GET** /stages/authenticator/validate/{stage_uuid}/ | 
[**stages_authenticator_validate_update**](StagesApi.md#stages_authenticator_validate_update) | **PUT** /stages/authenticator/validate/{stage_uuid}/ | 
[**stages_authenticator_validate_used_by_list**](StagesApi.md#stages_authenticator_validate_used_by_list) | **GET** /stages/authenticator/validate/{stage_uuid}/used_by/ | 
[**stages_authenticator_webauthn_create**](StagesApi.md#stages_authenticator_webauthn_create) | **POST** /stages/authenticator/webauthn/ | 
[**stages_authenticator_webauthn_destroy**](StagesApi.md#stages_authenticator_webauthn_destroy) | **DELETE** /stages/authenticator/webauthn/{stage_uuid}/ | 
[**stages_authenticator_webauthn_device_types_list**](StagesApi.md#stages_authenticator_webauthn_device_types_list) | **GET** /stages/authenticator/webauthn_device_types/ | 
[**stages_authenticator_webauthn_device_types_retrieve**](StagesApi.md#stages_authenticator_webauthn_device_types_retrieve) | **GET** /stages/authenticator/webauthn_device_types/{aaguid}/ | 
[**stages_authenticator_webauthn_list**](StagesApi.md#stages_authenticator_webauthn_list) | **GET** /stages/authenticator/webauthn/ | 
[**stages_authenticator_webauthn_partial_update**](StagesApi.md#stages_authenticator_webauthn_partial_update) | **PATCH** /stages/authenticator/webauthn/{stage_uuid}/ | 
[**stages_authenticator_webauthn_retrieve**](StagesApi.md#stages_authenticator_webauthn_retrieve) | **GET** /stages/authenticator/webauthn/{stage_uuid}/ | 
[**stages_authenticator_webauthn_update**](StagesApi.md#stages_authenticator_webauthn_update) | **PUT** /stages/authenticator/webauthn/{stage_uuid}/ | 
[**stages_authenticator_webauthn_used_by_list**](StagesApi.md#stages_authenticator_webauthn_used_by_list) | **GET** /stages/authenticator/webauthn/{stage_uuid}/used_by/ | 
[**stages_captcha_create**](StagesApi.md#stages_captcha_create) | **POST** /stages/captcha/ | 
[**stages_captcha_destroy**](StagesApi.md#stages_captcha_destroy) | **DELETE** /stages/captcha/{stage_uuid}/ | 
[**stages_captcha_list**](StagesApi.md#stages_captcha_list) | **GET** /stages/captcha/ | 
[**stages_captcha_partial_update**](StagesApi.md#stages_captcha_partial_update) | **PATCH** /stages/captcha/{stage_uuid}/ | 
[**stages_captcha_retrieve**](StagesApi.md#stages_captcha_retrieve) | **GET** /stages/captcha/{stage_uuid}/ | 
[**stages_captcha_update**](StagesApi.md#stages_captcha_update) | **PUT** /stages/captcha/{stage_uuid}/ | 
[**stages_captcha_used_by_list**](StagesApi.md#stages_captcha_used_by_list) | **GET** /stages/captcha/{stage_uuid}/used_by/ | 
[**stages_consent_create**](StagesApi.md#stages_consent_create) | **POST** /stages/consent/ | 
[**stages_consent_destroy**](StagesApi.md#stages_consent_destroy) | **DELETE** /stages/consent/{stage_uuid}/ | 
[**stages_consent_list**](StagesApi.md#stages_consent_list) | **GET** /stages/consent/ | 
[**stages_consent_partial_update**](StagesApi.md#stages_consent_partial_update) | **PATCH** /stages/consent/{stage_uuid}/ | 
[**stages_consent_retrieve**](StagesApi.md#stages_consent_retrieve) | **GET** /stages/consent/{stage_uuid}/ | 
[**stages_consent_update**](StagesApi.md#stages_consent_update) | **PUT** /stages/consent/{stage_uuid}/ | 
[**stages_consent_used_by_list**](StagesApi.md#stages_consent_used_by_list) | **GET** /stages/consent/{stage_uuid}/used_by/ | 
[**stages_deny_create**](StagesApi.md#stages_deny_create) | **POST** /stages/deny/ | 
[**stages_deny_destroy**](StagesApi.md#stages_deny_destroy) | **DELETE** /stages/deny/{stage_uuid}/ | 
[**stages_deny_list**](StagesApi.md#stages_deny_list) | **GET** /stages/deny/ | 
[**stages_deny_partial_update**](StagesApi.md#stages_deny_partial_update) | **PATCH** /stages/deny/{stage_uuid}/ | 
[**stages_deny_retrieve**](StagesApi.md#stages_deny_retrieve) | **GET** /stages/deny/{stage_uuid}/ | 
[**stages_deny_update**](StagesApi.md#stages_deny_update) | **PUT** /stages/deny/{stage_uuid}/ | 
[**stages_deny_used_by_list**](StagesApi.md#stages_deny_used_by_list) | **GET** /stages/deny/{stage_uuid}/used_by/ | 
[**stages_dummy_create**](StagesApi.md#stages_dummy_create) | **POST** /stages/dummy/ | 
[**stages_dummy_destroy**](StagesApi.md#stages_dummy_destroy) | **DELETE** /stages/dummy/{stage_uuid}/ | 
[**stages_dummy_list**](StagesApi.md#stages_dummy_list) | **GET** /stages/dummy/ | 
[**stages_dummy_partial_update**](StagesApi.md#stages_dummy_partial_update) | **PATCH** /stages/dummy/{stage_uuid}/ | 
[**stages_dummy_retrieve**](StagesApi.md#stages_dummy_retrieve) | **GET** /stages/dummy/{stage_uuid}/ | 
[**stages_dummy_update**](StagesApi.md#stages_dummy_update) | **PUT** /stages/dummy/{stage_uuid}/ | 
[**stages_dummy_used_by_list**](StagesApi.md#stages_dummy_used_by_list) | **GET** /stages/dummy/{stage_uuid}/used_by/ | 
[**stages_email_create**](StagesApi.md#stages_email_create) | **POST** /stages/email/ | 
[**stages_email_destroy**](StagesApi.md#stages_email_destroy) | **DELETE** /stages/email/{stage_uuid}/ | 
[**stages_email_list**](StagesApi.md#stages_email_list) | **GET** /stages/email/ | 
[**stages_email_partial_update**](StagesApi.md#stages_email_partial_update) | **PATCH** /stages/email/{stage_uuid}/ | 
[**stages_email_retrieve**](StagesApi.md#stages_email_retrieve) | **GET** /stages/email/{stage_uuid}/ | 
[**stages_email_templates_list**](StagesApi.md#stages_email_templates_list) | **GET** /stages/email/templates/ | 
[**stages_email_update**](StagesApi.md#stages_email_update) | **PUT** /stages/email/{stage_uuid}/ | 
[**stages_email_used_by_list**](StagesApi.md#stages_email_used_by_list) | **GET** /stages/email/{stage_uuid}/used_by/ | 
[**stages_identification_create**](StagesApi.md#stages_identification_create) | **POST** /stages/identification/ | 
[**stages_identification_destroy**](StagesApi.md#stages_identification_destroy) | **DELETE** /stages/identification/{stage_uuid}/ | 
[**stages_identification_list**](StagesApi.md#stages_identification_list) | **GET** /stages/identification/ | 
[**stages_identification_partial_update**](StagesApi.md#stages_identification_partial_update) | **PATCH** /stages/identification/{stage_uuid}/ | 
[**stages_identification_retrieve**](StagesApi.md#stages_identification_retrieve) | **GET** /stages/identification/{stage_uuid}/ | 
[**stages_identification_update**](StagesApi.md#stages_identification_update) | **PUT** /stages/identification/{stage_uuid}/ | 
[**stages_identification_used_by_list**](StagesApi.md#stages_identification_used_by_list) | **GET** /stages/identification/{stage_uuid}/used_by/ | 
[**stages_invitation_invitations_create**](StagesApi.md#stages_invitation_invitations_create) | **POST** /stages/invitation/invitations/ | 
[**stages_invitation_invitations_destroy**](StagesApi.md#stages_invitation_invitations_destroy) | **DELETE** /stages/invitation/invitations/{invite_uuid}/ | 
[**stages_invitation_invitations_list**](StagesApi.md#stages_invitation_invitations_list) | **GET** /stages/invitation/invitations/ | 
[**stages_invitation_invitations_partial_update**](StagesApi.md#stages_invitation_invitations_partial_update) | **PATCH** /stages/invitation/invitations/{invite_uuid}/ | 
[**stages_invitation_invitations_retrieve**](StagesApi.md#stages_invitation_invitations_retrieve) | **GET** /stages/invitation/invitations/{invite_uuid}/ | 
[**stages_invitation_invitations_update**](StagesApi.md#stages_invitation_invitations_update) | **PUT** /stages/invitation/invitations/{invite_uuid}/ | 
[**stages_invitation_invitations_used_by_list**](StagesApi.md#stages_invitation_invitations_used_by_list) | **GET** /stages/invitation/invitations/{invite_uuid}/used_by/ | 
[**stages_invitation_stages_create**](StagesApi.md#stages_invitation_stages_create) | **POST** /stages/invitation/stages/ | 
[**stages_invitation_stages_destroy**](StagesApi.md#stages_invitation_stages_destroy) | **DELETE** /stages/invitation/stages/{stage_uuid}/ | 
[**stages_invitation_stages_list**](StagesApi.md#stages_invitation_stages_list) | **GET** /stages/invitation/stages/ | 
[**stages_invitation_stages_partial_update**](StagesApi.md#stages_invitation_stages_partial_update) | **PATCH** /stages/invitation/stages/{stage_uuid}/ | 
[**stages_invitation_stages_retrieve**](StagesApi.md#stages_invitation_stages_retrieve) | **GET** /stages/invitation/stages/{stage_uuid}/ | 
[**stages_invitation_stages_update**](StagesApi.md#stages_invitation_stages_update) | **PUT** /stages/invitation/stages/{stage_uuid}/ | 
[**stages_invitation_stages_used_by_list**](StagesApi.md#stages_invitation_stages_used_by_list) | **GET** /stages/invitation/stages/{stage_uuid}/used_by/ | 
[**stages_password_create**](StagesApi.md#stages_password_create) | **POST** /stages/password/ | 
[**stages_password_destroy**](StagesApi.md#stages_password_destroy) | **DELETE** /stages/password/{stage_uuid}/ | 
[**stages_password_list**](StagesApi.md#stages_password_list) | **GET** /stages/password/ | 
[**stages_password_partial_update**](StagesApi.md#stages_password_partial_update) | **PATCH** /stages/password/{stage_uuid}/ | 
[**stages_password_retrieve**](StagesApi.md#stages_password_retrieve) | **GET** /stages/password/{stage_uuid}/ | 
[**stages_password_update**](StagesApi.md#stages_password_update) | **PUT** /stages/password/{stage_uuid}/ | 
[**stages_password_used_by_list**](StagesApi.md#stages_password_used_by_list) | **GET** /stages/password/{stage_uuid}/used_by/ | 
[**stages_prompt_prompts_create**](StagesApi.md#stages_prompt_prompts_create) | **POST** /stages/prompt/prompts/ | 
[**stages_prompt_prompts_destroy**](StagesApi.md#stages_prompt_prompts_destroy) | **DELETE** /stages/prompt/prompts/{prompt_uuid}/ | 
[**stages_prompt_prompts_list**](StagesApi.md#stages_prompt_prompts_list) | **GET** /stages/prompt/prompts/ | 
[**stages_prompt_prompts_partial_update**](StagesApi.md#stages_prompt_prompts_partial_update) | **PATCH** /stages/prompt/prompts/{prompt_uuid}/ | 
[**stages_prompt_prompts_preview_create**](StagesApi.md#stages_prompt_prompts_preview_create) | **POST** /stages/prompt/prompts/preview/ | 
[**stages_prompt_prompts_retrieve**](StagesApi.md#stages_prompt_prompts_retrieve) | **GET** /stages/prompt/prompts/{prompt_uuid}/ | 
[**stages_prompt_prompts_update**](StagesApi.md#stages_prompt_prompts_update) | **PUT** /stages/prompt/prompts/{prompt_uuid}/ | 
[**stages_prompt_prompts_used_by_list**](StagesApi.md#stages_prompt_prompts_used_by_list) | **GET** /stages/prompt/prompts/{prompt_uuid}/used_by/ | 
[**stages_prompt_stages_create**](StagesApi.md#stages_prompt_stages_create) | **POST** /stages/prompt/stages/ | 
[**stages_prompt_stages_destroy**](StagesApi.md#stages_prompt_stages_destroy) | **DELETE** /stages/prompt/stages/{stage_uuid}/ | 
[**stages_prompt_stages_list**](StagesApi.md#stages_prompt_stages_list) | **GET** /stages/prompt/stages/ | 
[**stages_prompt_stages_partial_update**](StagesApi.md#stages_prompt_stages_partial_update) | **PATCH** /stages/prompt/stages/{stage_uuid}/ | 
[**stages_prompt_stages_retrieve**](StagesApi.md#stages_prompt_stages_retrieve) | **GET** /stages/prompt/stages/{stage_uuid}/ | 
[**stages_prompt_stages_update**](StagesApi.md#stages_prompt_stages_update) | **PUT** /stages/prompt/stages/{stage_uuid}/ | 
[**stages_prompt_stages_used_by_list**](StagesApi.md#stages_prompt_stages_used_by_list) | **GET** /stages/prompt/stages/{stage_uuid}/used_by/ | 
[**stages_source_create**](StagesApi.md#stages_source_create) | **POST** /stages/source/ | 
[**stages_source_destroy**](StagesApi.md#stages_source_destroy) | **DELETE** /stages/source/{stage_uuid}/ | 
[**stages_source_list**](StagesApi.md#stages_source_list) | **GET** /stages/source/ | 
[**stages_source_partial_update**](StagesApi.md#stages_source_partial_update) | **PATCH** /stages/source/{stage_uuid}/ | 
[**stages_source_retrieve**](StagesApi.md#stages_source_retrieve) | **GET** /stages/source/{stage_uuid}/ | 
[**stages_source_update**](StagesApi.md#stages_source_update) | **PUT** /stages/source/{stage_uuid}/ | 
[**stages_source_used_by_list**](StagesApi.md#stages_source_used_by_list) | **GET** /stages/source/{stage_uuid}/used_by/ | 
[**stages_user_delete_create**](StagesApi.md#stages_user_delete_create) | **POST** /stages/user_delete/ | 
[**stages_user_delete_destroy**](StagesApi.md#stages_user_delete_destroy) | **DELETE** /stages/user_delete/{stage_uuid}/ | 
[**stages_user_delete_list**](StagesApi.md#stages_user_delete_list) | **GET** /stages/user_delete/ | 
[**stages_user_delete_partial_update**](StagesApi.md#stages_user_delete_partial_update) | **PATCH** /stages/user_delete/{stage_uuid}/ | 
[**stages_user_delete_retrieve**](StagesApi.md#stages_user_delete_retrieve) | **GET** /stages/user_delete/{stage_uuid}/ | 
[**stages_user_delete_update**](StagesApi.md#stages_user_delete_update) | **PUT** /stages/user_delete/{stage_uuid}/ | 
[**stages_user_delete_used_by_list**](StagesApi.md#stages_user_delete_used_by_list) | **GET** /stages/user_delete/{stage_uuid}/used_by/ | 
[**stages_user_login_create**](StagesApi.md#stages_user_login_create) | **POST** /stages/user_login/ | 
[**stages_user_login_destroy**](StagesApi.md#stages_user_login_destroy) | **DELETE** /stages/user_login/{stage_uuid}/ | 
[**stages_user_login_list**](StagesApi.md#stages_user_login_list) | **GET** /stages/user_login/ | 
[**stages_user_login_partial_update**](StagesApi.md#stages_user_login_partial_update) | **PATCH** /stages/user_login/{stage_uuid}/ | 
[**stages_user_login_retrieve**](StagesApi.md#stages_user_login_retrieve) | **GET** /stages/user_login/{stage_uuid}/ | 
[**stages_user_login_update**](StagesApi.md#stages_user_login_update) | **PUT** /stages/user_login/{stage_uuid}/ | 
[**stages_user_login_used_by_list**](StagesApi.md#stages_user_login_used_by_list) | **GET** /stages/user_login/{stage_uuid}/used_by/ | 
[**stages_user_logout_create**](StagesApi.md#stages_user_logout_create) | **POST** /stages/user_logout/ | 
[**stages_user_logout_destroy**](StagesApi.md#stages_user_logout_destroy) | **DELETE** /stages/user_logout/{stage_uuid}/ | 
[**stages_user_logout_list**](StagesApi.md#stages_user_logout_list) | **GET** /stages/user_logout/ | 
[**stages_user_logout_partial_update**](StagesApi.md#stages_user_logout_partial_update) | **PATCH** /stages/user_logout/{stage_uuid}/ | 
[**stages_user_logout_retrieve**](StagesApi.md#stages_user_logout_retrieve) | **GET** /stages/user_logout/{stage_uuid}/ | 
[**stages_user_logout_update**](StagesApi.md#stages_user_logout_update) | **PUT** /stages/user_logout/{stage_uuid}/ | 
[**stages_user_logout_used_by_list**](StagesApi.md#stages_user_logout_used_by_list) | **GET** /stages/user_logout/{stage_uuid}/used_by/ | 
[**stages_user_write_create**](StagesApi.md#stages_user_write_create) | **POST** /stages/user_write/ | 
[**stages_user_write_destroy**](StagesApi.md#stages_user_write_destroy) | **DELETE** /stages/user_write/{stage_uuid}/ | 
[**stages_user_write_list**](StagesApi.md#stages_user_write_list) | **GET** /stages/user_write/ | 
[**stages_user_write_partial_update**](StagesApi.md#stages_user_write_partial_update) | **PATCH** /stages/user_write/{stage_uuid}/ | 
[**stages_user_write_retrieve**](StagesApi.md#stages_user_write_retrieve) | **GET** /stages/user_write/{stage_uuid}/ | 
[**stages_user_write_update**](StagesApi.md#stages_user_write_update) | **PUT** /stages/user_write/{stage_uuid}/ | 
[**stages_user_write_used_by_list**](StagesApi.md#stages_user_write_used_by_list) | **GET** /stages/user_write/{stage_uuid}/used_by/ | 



## stages_all_destroy

> stages_all_destroy(stage_uuid)


Stage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this stage. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_all_list

> models::PaginatedStageList stages_all_list(name, ordering, page, page_size, search)


Stage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedStageList**](PaginatedStageList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_all_retrieve

> models::Stage stages_all_retrieve(stage_uuid)


Stage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this stage. | [required] |

### Return type

[**models::Stage**](Stage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_all_types_list

> Vec<models::TypeCreate> stages_all_types_list()


Get all creatable types

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::TypeCreate>**](TypeCreate.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_all_used_by_list

> Vec<models::UsedBy> stages_all_used_by_list(stage_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this stage. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_all_user_settings_list

> Vec<models::UserSetting> stages_all_user_settings_list()


Get all stages the user can configure

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::UserSetting>**](UserSetting.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_duo_create

> models::AuthenticatorDuoStage stages_authenticator_duo_create(authenticator_duo_stage_request)


AuthenticatorDuoStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authenticator_duo_stage_request** | [**AuthenticatorDuoStageRequest**](AuthenticatorDuoStageRequest.md) |  | [required] |

### Return type

[**models::AuthenticatorDuoStage**](AuthenticatorDuoStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_duo_destroy

> stages_authenticator_duo_destroy(stage_uuid)


AuthenticatorDuoStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Duo Authenticator Setup Stage. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_duo_enrollment_status_create

> models::DuoDeviceEnrollmentStatus stages_authenticator_duo_enrollment_status_create(stage_uuid)


Check enrollment status of user details in current session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Duo Authenticator Setup Stage. | [required] |

### Return type

[**models::DuoDeviceEnrollmentStatus**](DuoDeviceEnrollmentStatus.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_duo_import_device_manual_create

> stages_authenticator_duo_import_device_manual_create(stage_uuid, authenticator_duo_stage_manual_device_import_request)


Import duo devices into authentik

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Duo Authenticator Setup Stage. | [required] |
**authenticator_duo_stage_manual_device_import_request** | [**AuthenticatorDuoStageManualDeviceImportRequest**](AuthenticatorDuoStageManualDeviceImportRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_duo_import_devices_automatic_create

> models::AuthenticatorDuoStageDeviceImportResponse stages_authenticator_duo_import_devices_automatic_create(stage_uuid)


Import duo devices into authentik

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Duo Authenticator Setup Stage. | [required] |

### Return type

[**models::AuthenticatorDuoStageDeviceImportResponse**](AuthenticatorDuoStageDeviceImportResponse.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_duo_list

> models::PaginatedAuthenticatorDuoStageList stages_authenticator_duo_list(api_hostname, client_id, configure_flow, name, ordering, page, page_size, search)


AuthenticatorDuoStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_hostname** | Option<**String**> |  |  |
**client_id** | Option<**String**> |  |  |
**configure_flow** | Option<**uuid::Uuid**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedAuthenticatorDuoStageList**](PaginatedAuthenticatorDuoStageList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_duo_partial_update

> models::AuthenticatorDuoStage stages_authenticator_duo_partial_update(stage_uuid, patched_authenticator_duo_stage_request)


AuthenticatorDuoStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Duo Authenticator Setup Stage. | [required] |
**patched_authenticator_duo_stage_request** | Option<[**PatchedAuthenticatorDuoStageRequest**](PatchedAuthenticatorDuoStageRequest.md)> |  |  |

### Return type

[**models::AuthenticatorDuoStage**](AuthenticatorDuoStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_duo_retrieve

> models::AuthenticatorDuoStage stages_authenticator_duo_retrieve(stage_uuid)


AuthenticatorDuoStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Duo Authenticator Setup Stage. | [required] |

### Return type

[**models::AuthenticatorDuoStage**](AuthenticatorDuoStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_duo_update

> models::AuthenticatorDuoStage stages_authenticator_duo_update(stage_uuid, authenticator_duo_stage_request)


AuthenticatorDuoStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Duo Authenticator Setup Stage. | [required] |
**authenticator_duo_stage_request** | [**AuthenticatorDuoStageRequest**](AuthenticatorDuoStageRequest.md) |  | [required] |

### Return type

[**models::AuthenticatorDuoStage**](AuthenticatorDuoStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_duo_used_by_list

> Vec<models::UsedBy> stages_authenticator_duo_used_by_list(stage_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Duo Authenticator Setup Stage. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_endpoint_gdtc_create

> models::AuthenticatorEndpointGdtcStage stages_authenticator_endpoint_gdtc_create(authenticator_endpoint_gdtc_stage_request)


AuthenticatorEndpointGDTCStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authenticator_endpoint_gdtc_stage_request** | [**AuthenticatorEndpointGdtcStageRequest**](AuthenticatorEndpointGdtcStageRequest.md) |  | [required] |

### Return type

[**models::AuthenticatorEndpointGdtcStage**](AuthenticatorEndpointGDTCStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_endpoint_gdtc_destroy

> stages_authenticator_endpoint_gdtc_destroy(stage_uuid)


AuthenticatorEndpointGDTCStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Endpoint Authenticator Google Device Trust Connector Stage. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_endpoint_gdtc_list

> models::PaginatedAuthenticatorEndpointGdtcStageList stages_authenticator_endpoint_gdtc_list(configure_flow, name, ordering, page, page_size, search)


AuthenticatorEndpointGDTCStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configure_flow** | Option<**uuid::Uuid**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedAuthenticatorEndpointGdtcStageList**](PaginatedAuthenticatorEndpointGDTCStageList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_endpoint_gdtc_partial_update

> models::AuthenticatorEndpointGdtcStage stages_authenticator_endpoint_gdtc_partial_update(stage_uuid, patched_authenticator_endpoint_gdtc_stage_request)


AuthenticatorEndpointGDTCStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Endpoint Authenticator Google Device Trust Connector Stage. | [required] |
**patched_authenticator_endpoint_gdtc_stage_request** | Option<[**PatchedAuthenticatorEndpointGdtcStageRequest**](PatchedAuthenticatorEndpointGdtcStageRequest.md)> |  |  |

### Return type

[**models::AuthenticatorEndpointGdtcStage**](AuthenticatorEndpointGDTCStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_endpoint_gdtc_retrieve

> models::AuthenticatorEndpointGdtcStage stages_authenticator_endpoint_gdtc_retrieve(stage_uuid)


AuthenticatorEndpointGDTCStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Endpoint Authenticator Google Device Trust Connector Stage. | [required] |

### Return type

[**models::AuthenticatorEndpointGdtcStage**](AuthenticatorEndpointGDTCStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_endpoint_gdtc_update

> models::AuthenticatorEndpointGdtcStage stages_authenticator_endpoint_gdtc_update(stage_uuid, authenticator_endpoint_gdtc_stage_request)


AuthenticatorEndpointGDTCStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Endpoint Authenticator Google Device Trust Connector Stage. | [required] |
**authenticator_endpoint_gdtc_stage_request** | [**AuthenticatorEndpointGdtcStageRequest**](AuthenticatorEndpointGdtcStageRequest.md) |  | [required] |

### Return type

[**models::AuthenticatorEndpointGdtcStage**](AuthenticatorEndpointGDTCStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_endpoint_gdtc_used_by_list

> Vec<models::UsedBy> stages_authenticator_endpoint_gdtc_used_by_list(stage_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Endpoint Authenticator Google Device Trust Connector Stage. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_sms_create

> models::AuthenticatorSmsStage stages_authenticator_sms_create(authenticator_sms_stage_request)


AuthenticatorSMSStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authenticator_sms_stage_request** | [**AuthenticatorSmsStageRequest**](AuthenticatorSmsStageRequest.md) |  | [required] |

### Return type

[**models::AuthenticatorSmsStage**](AuthenticatorSMSStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_sms_destroy

> stages_authenticator_sms_destroy(stage_uuid)


AuthenticatorSMSStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this SMS Authenticator Setup Stage. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_sms_list

> models::PaginatedAuthenticatorSmsStageList stages_authenticator_sms_list(account_sid, auth, auth_password, auth_type, configure_flow, friendly_name, from_number, mapping, name, ordering, page, page_size, provider, search, stage_uuid, verify_only)


AuthenticatorSMSStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | Option<**String**> |  |  |
**auth** | Option<**String**> |  |  |
**auth_password** | Option<**String**> |  |  |
**auth_type** | Option<**String**> |  |  |
**configure_flow** | Option<**uuid::Uuid**> |  |  |
**friendly_name** | Option<**String**> |  |  |
**from_number** | Option<**String**> |  |  |
**mapping** | Option<**uuid::Uuid**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**provider** | Option<**String**> |  |  |
**search** | Option<**String**> | A search term. |  |
**stage_uuid** | Option<**uuid::Uuid**> |  |  |
**verify_only** | Option<**bool**> |  |  |

### Return type

[**models::PaginatedAuthenticatorSmsStageList**](PaginatedAuthenticatorSMSStageList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_sms_partial_update

> models::AuthenticatorSmsStage stages_authenticator_sms_partial_update(stage_uuid, patched_authenticator_sms_stage_request)


AuthenticatorSMSStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this SMS Authenticator Setup Stage. | [required] |
**patched_authenticator_sms_stage_request** | Option<[**PatchedAuthenticatorSmsStageRequest**](PatchedAuthenticatorSmsStageRequest.md)> |  |  |

### Return type

[**models::AuthenticatorSmsStage**](AuthenticatorSMSStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_sms_retrieve

> models::AuthenticatorSmsStage stages_authenticator_sms_retrieve(stage_uuid)


AuthenticatorSMSStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this SMS Authenticator Setup Stage. | [required] |

### Return type

[**models::AuthenticatorSmsStage**](AuthenticatorSMSStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_sms_update

> models::AuthenticatorSmsStage stages_authenticator_sms_update(stage_uuid, authenticator_sms_stage_request)


AuthenticatorSMSStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this SMS Authenticator Setup Stage. | [required] |
**authenticator_sms_stage_request** | [**AuthenticatorSmsStageRequest**](AuthenticatorSmsStageRequest.md) |  | [required] |

### Return type

[**models::AuthenticatorSmsStage**](AuthenticatorSMSStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_sms_used_by_list

> Vec<models::UsedBy> stages_authenticator_sms_used_by_list(stage_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this SMS Authenticator Setup Stage. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_static_create

> models::AuthenticatorStaticStage stages_authenticator_static_create(authenticator_static_stage_request)


AuthenticatorStaticStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authenticator_static_stage_request** | [**AuthenticatorStaticStageRequest**](AuthenticatorStaticStageRequest.md) |  | [required] |

### Return type

[**models::AuthenticatorStaticStage**](AuthenticatorStaticStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_static_destroy

> stages_authenticator_static_destroy(stage_uuid)


AuthenticatorStaticStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Static Authenticator Setup Stage. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_static_list

> models::PaginatedAuthenticatorStaticStageList stages_authenticator_static_list(configure_flow, friendly_name, name, ordering, page, page_size, search, stage_uuid, token_count, token_length)


AuthenticatorStaticStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configure_flow** | Option<**uuid::Uuid**> |  |  |
**friendly_name** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**stage_uuid** | Option<**uuid::Uuid**> |  |  |
**token_count** | Option<**i32**> |  |  |
**token_length** | Option<**i32**> |  |  |

### Return type

[**models::PaginatedAuthenticatorStaticStageList**](PaginatedAuthenticatorStaticStageList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_static_partial_update

> models::AuthenticatorStaticStage stages_authenticator_static_partial_update(stage_uuid, patched_authenticator_static_stage_request)


AuthenticatorStaticStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Static Authenticator Setup Stage. | [required] |
**patched_authenticator_static_stage_request** | Option<[**PatchedAuthenticatorStaticStageRequest**](PatchedAuthenticatorStaticStageRequest.md)> |  |  |

### Return type

[**models::AuthenticatorStaticStage**](AuthenticatorStaticStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_static_retrieve

> models::AuthenticatorStaticStage stages_authenticator_static_retrieve(stage_uuid)


AuthenticatorStaticStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Static Authenticator Setup Stage. | [required] |

### Return type

[**models::AuthenticatorStaticStage**](AuthenticatorStaticStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_static_update

> models::AuthenticatorStaticStage stages_authenticator_static_update(stage_uuid, authenticator_static_stage_request)


AuthenticatorStaticStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Static Authenticator Setup Stage. | [required] |
**authenticator_static_stage_request** | [**AuthenticatorStaticStageRequest**](AuthenticatorStaticStageRequest.md) |  | [required] |

### Return type

[**models::AuthenticatorStaticStage**](AuthenticatorStaticStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_static_used_by_list

> Vec<models::UsedBy> stages_authenticator_static_used_by_list(stage_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Static Authenticator Setup Stage. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_totp_create

> models::AuthenticatorTotpStage stages_authenticator_totp_create(authenticator_totp_stage_request)


AuthenticatorTOTPStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authenticator_totp_stage_request** | [**AuthenticatorTotpStageRequest**](AuthenticatorTotpStageRequest.md) |  | [required] |

### Return type

[**models::AuthenticatorTotpStage**](AuthenticatorTOTPStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_totp_destroy

> stages_authenticator_totp_destroy(stage_uuid)


AuthenticatorTOTPStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this TOTP Authenticator Setup Stage. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_totp_list

> models::PaginatedAuthenticatorTotpStageList stages_authenticator_totp_list(configure_flow, digits, friendly_name, name, ordering, page, page_size, search, stage_uuid)


AuthenticatorTOTPStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configure_flow** | Option<**uuid::Uuid**> |  |  |
**digits** | Option<**String**> |  |  |
**friendly_name** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**stage_uuid** | Option<**uuid::Uuid**> |  |  |

### Return type

[**models::PaginatedAuthenticatorTotpStageList**](PaginatedAuthenticatorTOTPStageList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_totp_partial_update

> models::AuthenticatorTotpStage stages_authenticator_totp_partial_update(stage_uuid, patched_authenticator_totp_stage_request)


AuthenticatorTOTPStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this TOTP Authenticator Setup Stage. | [required] |
**patched_authenticator_totp_stage_request** | Option<[**PatchedAuthenticatorTotpStageRequest**](PatchedAuthenticatorTotpStageRequest.md)> |  |  |

### Return type

[**models::AuthenticatorTotpStage**](AuthenticatorTOTPStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_totp_retrieve

> models::AuthenticatorTotpStage stages_authenticator_totp_retrieve(stage_uuid)


AuthenticatorTOTPStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this TOTP Authenticator Setup Stage. | [required] |

### Return type

[**models::AuthenticatorTotpStage**](AuthenticatorTOTPStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_totp_update

> models::AuthenticatorTotpStage stages_authenticator_totp_update(stage_uuid, authenticator_totp_stage_request)


AuthenticatorTOTPStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this TOTP Authenticator Setup Stage. | [required] |
**authenticator_totp_stage_request** | [**AuthenticatorTotpStageRequest**](AuthenticatorTotpStageRequest.md) |  | [required] |

### Return type

[**models::AuthenticatorTotpStage**](AuthenticatorTOTPStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_totp_used_by_list

> Vec<models::UsedBy> stages_authenticator_totp_used_by_list(stage_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this TOTP Authenticator Setup Stage. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_validate_create

> models::AuthenticatorValidateStage stages_authenticator_validate_create(authenticator_validate_stage_request)


AuthenticatorValidateStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authenticator_validate_stage_request** | [**AuthenticatorValidateStageRequest**](AuthenticatorValidateStageRequest.md) |  | [required] |

### Return type

[**models::AuthenticatorValidateStage**](AuthenticatorValidateStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_validate_destroy

> stages_authenticator_validate_destroy(stage_uuid)


AuthenticatorValidateStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Authenticator Validation Stage. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_validate_list

> models::PaginatedAuthenticatorValidateStageList stages_authenticator_validate_list(configuration_stages, name, not_configured_action, ordering, page, page_size, search)


AuthenticatorValidateStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configuration_stages** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  |  |
**name** | Option<**String**> |  |  |
**not_configured_action** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedAuthenticatorValidateStageList**](PaginatedAuthenticatorValidateStageList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_validate_partial_update

> models::AuthenticatorValidateStage stages_authenticator_validate_partial_update(stage_uuid, patched_authenticator_validate_stage_request)


AuthenticatorValidateStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Authenticator Validation Stage. | [required] |
**patched_authenticator_validate_stage_request** | Option<[**PatchedAuthenticatorValidateStageRequest**](PatchedAuthenticatorValidateStageRequest.md)> |  |  |

### Return type

[**models::AuthenticatorValidateStage**](AuthenticatorValidateStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_validate_retrieve

> models::AuthenticatorValidateStage stages_authenticator_validate_retrieve(stage_uuid)


AuthenticatorValidateStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Authenticator Validation Stage. | [required] |

### Return type

[**models::AuthenticatorValidateStage**](AuthenticatorValidateStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_validate_update

> models::AuthenticatorValidateStage stages_authenticator_validate_update(stage_uuid, authenticator_validate_stage_request)


AuthenticatorValidateStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Authenticator Validation Stage. | [required] |
**authenticator_validate_stage_request** | [**AuthenticatorValidateStageRequest**](AuthenticatorValidateStageRequest.md) |  | [required] |

### Return type

[**models::AuthenticatorValidateStage**](AuthenticatorValidateStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_validate_used_by_list

> Vec<models::UsedBy> stages_authenticator_validate_used_by_list(stage_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Authenticator Validation Stage. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_webauthn_create

> models::AuthenticatorWebAuthnStage stages_authenticator_webauthn_create(authenticator_web_authn_stage_request)


AuthenticatorWebAuthnStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authenticator_web_authn_stage_request** | [**AuthenticatorWebAuthnStageRequest**](AuthenticatorWebAuthnStageRequest.md) |  | [required] |

### Return type

[**models::AuthenticatorWebAuthnStage**](AuthenticatorWebAuthnStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_webauthn_destroy

> stages_authenticator_webauthn_destroy(stage_uuid)


AuthenticatorWebAuthnStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this WebAuthn Authenticator Setup Stage. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_webauthn_device_types_list

> models::PaginatedWebAuthnDeviceTypeList stages_authenticator_webauthn_device_types_list(aaguid, description, icon, ordering, page, page_size, search)


WebAuthnDeviceType Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**aaguid** | Option<**uuid::Uuid**> |  |  |
**description** | Option<**String**> |  |  |
**icon** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedWebAuthnDeviceTypeList**](PaginatedWebAuthnDeviceTypeList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_webauthn_device_types_retrieve

> models::WebAuthnDeviceType stages_authenticator_webauthn_device_types_retrieve(aaguid)


WebAuthnDeviceType Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**aaguid** | **uuid::Uuid** | A UUID string identifying this WebAuthn Device type. | [required] |

### Return type

[**models::WebAuthnDeviceType**](WebAuthnDeviceType.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_webauthn_list

> models::PaginatedAuthenticatorWebAuthnStageList stages_authenticator_webauthn_list(authenticator_attachment, configure_flow, device_type_restrictions, friendly_name, name, ordering, page, page_size, resident_key_requirement, search, stage_uuid, user_verification)


AuthenticatorWebAuthnStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authenticator_attachment** | Option<**String**> |  |  |
**configure_flow** | Option<**uuid::Uuid**> |  |  |
**device_type_restrictions** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  |  |
**friendly_name** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**resident_key_requirement** | Option<**String**> |  |  |
**search** | Option<**String**> | A search term. |  |
**stage_uuid** | Option<**uuid::Uuid**> |  |  |
**user_verification** | Option<**String**> |  |  |

### Return type

[**models::PaginatedAuthenticatorWebAuthnStageList**](PaginatedAuthenticatorWebAuthnStageList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_webauthn_partial_update

> models::AuthenticatorWebAuthnStage stages_authenticator_webauthn_partial_update(stage_uuid, patched_authenticator_web_authn_stage_request)


AuthenticatorWebAuthnStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this WebAuthn Authenticator Setup Stage. | [required] |
**patched_authenticator_web_authn_stage_request** | Option<[**PatchedAuthenticatorWebAuthnStageRequest**](PatchedAuthenticatorWebAuthnStageRequest.md)> |  |  |

### Return type

[**models::AuthenticatorWebAuthnStage**](AuthenticatorWebAuthnStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_webauthn_retrieve

> models::AuthenticatorWebAuthnStage stages_authenticator_webauthn_retrieve(stage_uuid)


AuthenticatorWebAuthnStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this WebAuthn Authenticator Setup Stage. | [required] |

### Return type

[**models::AuthenticatorWebAuthnStage**](AuthenticatorWebAuthnStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_webauthn_update

> models::AuthenticatorWebAuthnStage stages_authenticator_webauthn_update(stage_uuid, authenticator_web_authn_stage_request)


AuthenticatorWebAuthnStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this WebAuthn Authenticator Setup Stage. | [required] |
**authenticator_web_authn_stage_request** | [**AuthenticatorWebAuthnStageRequest**](AuthenticatorWebAuthnStageRequest.md) |  | [required] |

### Return type

[**models::AuthenticatorWebAuthnStage**](AuthenticatorWebAuthnStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_authenticator_webauthn_used_by_list

> Vec<models::UsedBy> stages_authenticator_webauthn_used_by_list(stage_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this WebAuthn Authenticator Setup Stage. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_captcha_create

> models::CaptchaStage stages_captcha_create(captcha_stage_request)


CaptchaStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**captcha_stage_request** | [**CaptchaStageRequest**](CaptchaStageRequest.md) |  | [required] |

### Return type

[**models::CaptchaStage**](CaptchaStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_captcha_destroy

> stages_captcha_destroy(stage_uuid)


CaptchaStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Captcha Stage. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_captcha_list

> models::PaginatedCaptchaStageList stages_captcha_list(name, ordering, page, page_size, public_key, search)


CaptchaStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**public_key** | Option<**String**> |  |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedCaptchaStageList**](PaginatedCaptchaStageList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_captcha_partial_update

> models::CaptchaStage stages_captcha_partial_update(stage_uuid, patched_captcha_stage_request)


CaptchaStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Captcha Stage. | [required] |
**patched_captcha_stage_request** | Option<[**PatchedCaptchaStageRequest**](PatchedCaptchaStageRequest.md)> |  |  |

### Return type

[**models::CaptchaStage**](CaptchaStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_captcha_retrieve

> models::CaptchaStage stages_captcha_retrieve(stage_uuid)


CaptchaStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Captcha Stage. | [required] |

### Return type

[**models::CaptchaStage**](CaptchaStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_captcha_update

> models::CaptchaStage stages_captcha_update(stage_uuid, captcha_stage_request)


CaptchaStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Captcha Stage. | [required] |
**captcha_stage_request** | [**CaptchaStageRequest**](CaptchaStageRequest.md) |  | [required] |

### Return type

[**models::CaptchaStage**](CaptchaStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_captcha_used_by_list

> Vec<models::UsedBy> stages_captcha_used_by_list(stage_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Captcha Stage. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_consent_create

> models::ConsentStage stages_consent_create(consent_stage_request)


ConsentStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_stage_request** | [**ConsentStageRequest**](ConsentStageRequest.md) |  | [required] |

### Return type

[**models::ConsentStage**](ConsentStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_consent_destroy

> stages_consent_destroy(stage_uuid)


ConsentStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Consent Stage. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_consent_list

> models::PaginatedConsentStageList stages_consent_list(consent_expire_in, mode, name, ordering, page, page_size, search, stage_uuid)


ConsentStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_expire_in** | Option<**String**> |  |  |
**mode** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**stage_uuid** | Option<**uuid::Uuid**> |  |  |

### Return type

[**models::PaginatedConsentStageList**](PaginatedConsentStageList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_consent_partial_update

> models::ConsentStage stages_consent_partial_update(stage_uuid, patched_consent_stage_request)


ConsentStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Consent Stage. | [required] |
**patched_consent_stage_request** | Option<[**PatchedConsentStageRequest**](PatchedConsentStageRequest.md)> |  |  |

### Return type

[**models::ConsentStage**](ConsentStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_consent_retrieve

> models::ConsentStage stages_consent_retrieve(stage_uuid)


ConsentStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Consent Stage. | [required] |

### Return type

[**models::ConsentStage**](ConsentStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_consent_update

> models::ConsentStage stages_consent_update(stage_uuid, consent_stage_request)


ConsentStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Consent Stage. | [required] |
**consent_stage_request** | [**ConsentStageRequest**](ConsentStageRequest.md) |  | [required] |

### Return type

[**models::ConsentStage**](ConsentStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_consent_used_by_list

> Vec<models::UsedBy> stages_consent_used_by_list(stage_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Consent Stage. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_deny_create

> models::DenyStage stages_deny_create(deny_stage_request)


DenyStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deny_stage_request** | [**DenyStageRequest**](DenyStageRequest.md) |  | [required] |

### Return type

[**models::DenyStage**](DenyStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_deny_destroy

> stages_deny_destroy(stage_uuid)


DenyStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Deny Stage. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_deny_list

> models::PaginatedDenyStageList stages_deny_list(deny_message, name, ordering, page, page_size, search, stage_uuid)


DenyStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deny_message** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**stage_uuid** | Option<**uuid::Uuid**> |  |  |

### Return type

[**models::PaginatedDenyStageList**](PaginatedDenyStageList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_deny_partial_update

> models::DenyStage stages_deny_partial_update(stage_uuid, patched_deny_stage_request)


DenyStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Deny Stage. | [required] |
**patched_deny_stage_request** | Option<[**PatchedDenyStageRequest**](PatchedDenyStageRequest.md)> |  |  |

### Return type

[**models::DenyStage**](DenyStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_deny_retrieve

> models::DenyStage stages_deny_retrieve(stage_uuid)


DenyStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Deny Stage. | [required] |

### Return type

[**models::DenyStage**](DenyStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_deny_update

> models::DenyStage stages_deny_update(stage_uuid, deny_stage_request)


DenyStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Deny Stage. | [required] |
**deny_stage_request** | [**DenyStageRequest**](DenyStageRequest.md) |  | [required] |

### Return type

[**models::DenyStage**](DenyStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_deny_used_by_list

> Vec<models::UsedBy> stages_deny_used_by_list(stage_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Deny Stage. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_dummy_create

> models::DummyStage stages_dummy_create(dummy_stage_request)


DummyStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dummy_stage_request** | [**DummyStageRequest**](DummyStageRequest.md) |  | [required] |

### Return type

[**models::DummyStage**](DummyStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_dummy_destroy

> stages_dummy_destroy(stage_uuid)


DummyStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Dummy Stage. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_dummy_list

> models::PaginatedDummyStageList stages_dummy_list(name, ordering, page, page_size, search, stage_uuid, throw_error)


DummyStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**stage_uuid** | Option<**uuid::Uuid**> |  |  |
**throw_error** | Option<**bool**> |  |  |

### Return type

[**models::PaginatedDummyStageList**](PaginatedDummyStageList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_dummy_partial_update

> models::DummyStage stages_dummy_partial_update(stage_uuid, patched_dummy_stage_request)


DummyStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Dummy Stage. | [required] |
**patched_dummy_stage_request** | Option<[**PatchedDummyStageRequest**](PatchedDummyStageRequest.md)> |  |  |

### Return type

[**models::DummyStage**](DummyStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_dummy_retrieve

> models::DummyStage stages_dummy_retrieve(stage_uuid)


DummyStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Dummy Stage. | [required] |

### Return type

[**models::DummyStage**](DummyStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_dummy_update

> models::DummyStage stages_dummy_update(stage_uuid, dummy_stage_request)


DummyStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Dummy Stage. | [required] |
**dummy_stage_request** | [**DummyStageRequest**](DummyStageRequest.md) |  | [required] |

### Return type

[**models::DummyStage**](DummyStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_dummy_used_by_list

> Vec<models::UsedBy> stages_dummy_used_by_list(stage_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Dummy Stage. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_email_create

> models::EmailStage stages_email_create(email_stage_request)


EmailStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_stage_request** | [**EmailStageRequest**](EmailStageRequest.md) |  | [required] |

### Return type

[**models::EmailStage**](EmailStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_email_destroy

> stages_email_destroy(stage_uuid)


EmailStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Email Stage. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_email_list

> models::PaginatedEmailStageList stages_email_list(activate_user_on_success, from_address, host, name, ordering, page, page_size, port, search, subject, template, timeout, token_expiry, use_global_settings, use_ssl, use_tls, username)


EmailStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**activate_user_on_success** | Option<**bool**> |  |  |
**from_address** | Option<**String**> |  |  |
**host** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**port** | Option<**i32**> |  |  |
**search** | Option<**String**> | A search term. |  |
**subject** | Option<**String**> |  |  |
**template** | Option<**String**> |  |  |
**timeout** | Option<**i32**> |  |  |
**token_expiry** | Option<**i32**> |  |  |
**use_global_settings** | Option<**bool**> |  |  |
**use_ssl** | Option<**bool**> |  |  |
**use_tls** | Option<**bool**> |  |  |
**username** | Option<**String**> |  |  |

### Return type

[**models::PaginatedEmailStageList**](PaginatedEmailStageList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_email_partial_update

> models::EmailStage stages_email_partial_update(stage_uuid, patched_email_stage_request)


EmailStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Email Stage. | [required] |
**patched_email_stage_request** | Option<[**PatchedEmailStageRequest**](PatchedEmailStageRequest.md)> |  |  |

### Return type

[**models::EmailStage**](EmailStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_email_retrieve

> models::EmailStage stages_email_retrieve(stage_uuid)


EmailStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Email Stage. | [required] |

### Return type

[**models::EmailStage**](EmailStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_email_templates_list

> Vec<models::TypeCreate> stages_email_templates_list()


Get all available templates, including custom templates

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::TypeCreate>**](TypeCreate.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_email_update

> models::EmailStage stages_email_update(stage_uuid, email_stage_request)


EmailStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Email Stage. | [required] |
**email_stage_request** | [**EmailStageRequest**](EmailStageRequest.md) |  | [required] |

### Return type

[**models::EmailStage**](EmailStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_email_used_by_list

> Vec<models::UsedBy> stages_email_used_by_list(stage_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Email Stage. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_identification_create

> models::IdentificationStage stages_identification_create(identification_stage_request)


IdentificationStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identification_stage_request** | [**IdentificationStageRequest**](IdentificationStageRequest.md) |  | [required] |

### Return type

[**models::IdentificationStage**](IdentificationStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_identification_destroy

> stages_identification_destroy(stage_uuid)


IdentificationStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Identification Stage. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_identification_list

> models::PaginatedIdentificationStageList stages_identification_list(captcha_stage, case_insensitive_matching, enrollment_flow, name, ordering, page, page_size, password_stage, passwordless_flow, recovery_flow, search, show_matched_user, show_source_labels)


IdentificationStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**captcha_stage** | Option<**uuid::Uuid**> |  |  |
**case_insensitive_matching** | Option<**bool**> |  |  |
**enrollment_flow** | Option<**uuid::Uuid**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**password_stage** | Option<**uuid::Uuid**> |  |  |
**passwordless_flow** | Option<**uuid::Uuid**> |  |  |
**recovery_flow** | Option<**uuid::Uuid**> |  |  |
**search** | Option<**String**> | A search term. |  |
**show_matched_user** | Option<**bool**> |  |  |
**show_source_labels** | Option<**bool**> |  |  |

### Return type

[**models::PaginatedIdentificationStageList**](PaginatedIdentificationStageList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_identification_partial_update

> models::IdentificationStage stages_identification_partial_update(stage_uuid, patched_identification_stage_request)


IdentificationStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Identification Stage. | [required] |
**patched_identification_stage_request** | Option<[**PatchedIdentificationStageRequest**](PatchedIdentificationStageRequest.md)> |  |  |

### Return type

[**models::IdentificationStage**](IdentificationStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_identification_retrieve

> models::IdentificationStage stages_identification_retrieve(stage_uuid)


IdentificationStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Identification Stage. | [required] |

### Return type

[**models::IdentificationStage**](IdentificationStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_identification_update

> models::IdentificationStage stages_identification_update(stage_uuid, identification_stage_request)


IdentificationStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Identification Stage. | [required] |
**identification_stage_request** | [**IdentificationStageRequest**](IdentificationStageRequest.md) |  | [required] |

### Return type

[**models::IdentificationStage**](IdentificationStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_identification_used_by_list

> Vec<models::UsedBy> stages_identification_used_by_list(stage_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Identification Stage. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_invitation_invitations_create

> models::Invitation stages_invitation_invitations_create(invitation_request)


Invitation Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invitation_request** | [**InvitationRequest**](InvitationRequest.md) |  | [required] |

### Return type

[**models::Invitation**](Invitation.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_invitation_invitations_destroy

> stages_invitation_invitations_destroy(invite_uuid)


Invitation Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invite_uuid** | **uuid::Uuid** | A UUID string identifying this Invitation. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_invitation_invitations_list

> models::PaginatedInvitationList stages_invitation_invitations_list(created_by__username, expires, flow__slug, name, ordering, page, page_size, search)


Invitation Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**created_by__username** | Option<**String**> |  |  |
**expires** | Option<**String**> |  |  |
**flow__slug** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedInvitationList**](PaginatedInvitationList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_invitation_invitations_partial_update

> models::Invitation stages_invitation_invitations_partial_update(invite_uuid, patched_invitation_request)


Invitation Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invite_uuid** | **uuid::Uuid** | A UUID string identifying this Invitation. | [required] |
**patched_invitation_request** | Option<[**PatchedInvitationRequest**](PatchedInvitationRequest.md)> |  |  |

### Return type

[**models::Invitation**](Invitation.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_invitation_invitations_retrieve

> models::Invitation stages_invitation_invitations_retrieve(invite_uuid)


Invitation Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invite_uuid** | **uuid::Uuid** | A UUID string identifying this Invitation. | [required] |

### Return type

[**models::Invitation**](Invitation.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_invitation_invitations_update

> models::Invitation stages_invitation_invitations_update(invite_uuid, invitation_request)


Invitation Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invite_uuid** | **uuid::Uuid** | A UUID string identifying this Invitation. | [required] |
**invitation_request** | [**InvitationRequest**](InvitationRequest.md) |  | [required] |

### Return type

[**models::Invitation**](Invitation.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_invitation_invitations_used_by_list

> Vec<models::UsedBy> stages_invitation_invitations_used_by_list(invite_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invite_uuid** | **uuid::Uuid** | A UUID string identifying this Invitation. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_invitation_stages_create

> models::InvitationStage stages_invitation_stages_create(invitation_stage_request)


InvitationStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invitation_stage_request** | [**InvitationStageRequest**](InvitationStageRequest.md) |  | [required] |

### Return type

[**models::InvitationStage**](InvitationStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_invitation_stages_destroy

> stages_invitation_stages_destroy(stage_uuid)


InvitationStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Invitation Stage. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_invitation_stages_list

> models::PaginatedInvitationStageList stages_invitation_stages_list(continue_flow_without_invitation, name, no_flows, ordering, page, page_size, search, stage_uuid)


InvitationStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**continue_flow_without_invitation** | Option<**bool**> |  |  |
**name** | Option<**String**> |  |  |
**no_flows** | Option<**bool**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**stage_uuid** | Option<**uuid::Uuid**> |  |  |

### Return type

[**models::PaginatedInvitationStageList**](PaginatedInvitationStageList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_invitation_stages_partial_update

> models::InvitationStage stages_invitation_stages_partial_update(stage_uuid, patched_invitation_stage_request)


InvitationStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Invitation Stage. | [required] |
**patched_invitation_stage_request** | Option<[**PatchedInvitationStageRequest**](PatchedInvitationStageRequest.md)> |  |  |

### Return type

[**models::InvitationStage**](InvitationStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_invitation_stages_retrieve

> models::InvitationStage stages_invitation_stages_retrieve(stage_uuid)


InvitationStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Invitation Stage. | [required] |

### Return type

[**models::InvitationStage**](InvitationStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_invitation_stages_update

> models::InvitationStage stages_invitation_stages_update(stage_uuid, invitation_stage_request)


InvitationStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Invitation Stage. | [required] |
**invitation_stage_request** | [**InvitationStageRequest**](InvitationStageRequest.md) |  | [required] |

### Return type

[**models::InvitationStage**](InvitationStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_invitation_stages_used_by_list

> Vec<models::UsedBy> stages_invitation_stages_used_by_list(stage_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Invitation Stage. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_password_create

> models::PasswordStage stages_password_create(password_stage_request)


PasswordStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**password_stage_request** | [**PasswordStageRequest**](PasswordStageRequest.md) |  | [required] |

### Return type

[**models::PasswordStage**](PasswordStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_password_destroy

> stages_password_destroy(stage_uuid)


PasswordStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Password Stage. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_password_list

> models::PaginatedPasswordStageList stages_password_list(allow_show_password, configure_flow, failed_attempts_before_cancel, name, ordering, page, page_size, search)


PasswordStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allow_show_password** | Option<**bool**> |  |  |
**configure_flow** | Option<**uuid::Uuid**> |  |  |
**failed_attempts_before_cancel** | Option<**i32**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedPasswordStageList**](PaginatedPasswordStageList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_password_partial_update

> models::PasswordStage stages_password_partial_update(stage_uuid, patched_password_stage_request)


PasswordStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Password Stage. | [required] |
**patched_password_stage_request** | Option<[**PatchedPasswordStageRequest**](PatchedPasswordStageRequest.md)> |  |  |

### Return type

[**models::PasswordStage**](PasswordStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_password_retrieve

> models::PasswordStage stages_password_retrieve(stage_uuid)


PasswordStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Password Stage. | [required] |

### Return type

[**models::PasswordStage**](PasswordStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_password_update

> models::PasswordStage stages_password_update(stage_uuid, password_stage_request)


PasswordStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Password Stage. | [required] |
**password_stage_request** | [**PasswordStageRequest**](PasswordStageRequest.md) |  | [required] |

### Return type

[**models::PasswordStage**](PasswordStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_password_used_by_list

> Vec<models::UsedBy> stages_password_used_by_list(stage_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Password Stage. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_prompt_prompts_create

> models::Prompt stages_prompt_prompts_create(prompt_request)


Prompt Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_request** | [**PromptRequest**](PromptRequest.md) |  | [required] |

### Return type

[**models::Prompt**](Prompt.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_prompt_prompts_destroy

> stages_prompt_prompts_destroy(prompt_uuid)


Prompt Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_uuid** | **uuid::Uuid** | A UUID string identifying this Prompt. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_prompt_prompts_list

> models::PaginatedPromptList stages_prompt_prompts_list(field_key, label, name, ordering, page, page_size, placeholder, search, r#type)


Prompt Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_key** | Option<**String**> |  |  |
**label** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**placeholder** | Option<**String**> |  |  |
**search** | Option<**String**> | A search term. |  |
**r#type** | Option<**String**> |  |  |

### Return type

[**models::PaginatedPromptList**](PaginatedPromptList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_prompt_prompts_partial_update

> models::Prompt stages_prompt_prompts_partial_update(prompt_uuid, patched_prompt_request)


Prompt Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_uuid** | **uuid::Uuid** | A UUID string identifying this Prompt. | [required] |
**patched_prompt_request** | Option<[**PatchedPromptRequest**](PatchedPromptRequest.md)> |  |  |

### Return type

[**models::Prompt**](Prompt.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_prompt_prompts_preview_create

> models::PromptChallenge stages_prompt_prompts_preview_create(prompt_request)


Preview a prompt as a challenge, just like a flow would receive

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_request** | [**PromptRequest**](PromptRequest.md) |  | [required] |

### Return type

[**models::PromptChallenge**](PromptChallenge.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_prompt_prompts_retrieve

> models::Prompt stages_prompt_prompts_retrieve(prompt_uuid)


Prompt Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_uuid** | **uuid::Uuid** | A UUID string identifying this Prompt. | [required] |

### Return type

[**models::Prompt**](Prompt.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_prompt_prompts_update

> models::Prompt stages_prompt_prompts_update(prompt_uuid, prompt_request)


Prompt Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_uuid** | **uuid::Uuid** | A UUID string identifying this Prompt. | [required] |
**prompt_request** | [**PromptRequest**](PromptRequest.md) |  | [required] |

### Return type

[**models::Prompt**](Prompt.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_prompt_prompts_used_by_list

> Vec<models::UsedBy> stages_prompt_prompts_used_by_list(prompt_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_uuid** | **uuid::Uuid** | A UUID string identifying this Prompt. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_prompt_stages_create

> models::PromptStage stages_prompt_stages_create(prompt_stage_request)


PromptStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_stage_request** | [**PromptStageRequest**](PromptStageRequest.md) |  | [required] |

### Return type

[**models::PromptStage**](PromptStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_prompt_stages_destroy

> stages_prompt_stages_destroy(stage_uuid)


PromptStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Prompt Stage. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_prompt_stages_list

> models::PaginatedPromptStageList stages_prompt_stages_list(fields, name, ordering, page, page_size, search, stage_uuid, validation_policies)


PromptStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**stage_uuid** | Option<**uuid::Uuid**> |  |  |
**validation_policies** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  |  |

### Return type

[**models::PaginatedPromptStageList**](PaginatedPromptStageList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_prompt_stages_partial_update

> models::PromptStage stages_prompt_stages_partial_update(stage_uuid, patched_prompt_stage_request)


PromptStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Prompt Stage. | [required] |
**patched_prompt_stage_request** | Option<[**PatchedPromptStageRequest**](PatchedPromptStageRequest.md)> |  |  |

### Return type

[**models::PromptStage**](PromptStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_prompt_stages_retrieve

> models::PromptStage stages_prompt_stages_retrieve(stage_uuid)


PromptStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Prompt Stage. | [required] |

### Return type

[**models::PromptStage**](PromptStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_prompt_stages_update

> models::PromptStage stages_prompt_stages_update(stage_uuid, prompt_stage_request)


PromptStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Prompt Stage. | [required] |
**prompt_stage_request** | [**PromptStageRequest**](PromptStageRequest.md) |  | [required] |

### Return type

[**models::PromptStage**](PromptStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_prompt_stages_used_by_list

> Vec<models::UsedBy> stages_prompt_stages_used_by_list(stage_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Prompt Stage. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_source_create

> models::SourceStage stages_source_create(source_stage_request)


SourceStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_stage_request** | [**SourceStageRequest**](SourceStageRequest.md) |  | [required] |

### Return type

[**models::SourceStage**](SourceStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_source_destroy

> stages_source_destroy(stage_uuid)


SourceStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Source Stage. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_source_list

> models::PaginatedSourceStageList stages_source_list(name, ordering, page, page_size, resume_timeout, search, source, stage_uuid)


SourceStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**resume_timeout** | Option<**String**> |  |  |
**search** | Option<**String**> | A search term. |  |
**source** | Option<**uuid::Uuid**> |  |  |
**stage_uuid** | Option<**uuid::Uuid**> |  |  |

### Return type

[**models::PaginatedSourceStageList**](PaginatedSourceStageList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_source_partial_update

> models::SourceStage stages_source_partial_update(stage_uuid, patched_source_stage_request)


SourceStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Source Stage. | [required] |
**patched_source_stage_request** | Option<[**PatchedSourceStageRequest**](PatchedSourceStageRequest.md)> |  |  |

### Return type

[**models::SourceStage**](SourceStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_source_retrieve

> models::SourceStage stages_source_retrieve(stage_uuid)


SourceStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Source Stage. | [required] |

### Return type

[**models::SourceStage**](SourceStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_source_update

> models::SourceStage stages_source_update(stage_uuid, source_stage_request)


SourceStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Source Stage. | [required] |
**source_stage_request** | [**SourceStageRequest**](SourceStageRequest.md) |  | [required] |

### Return type

[**models::SourceStage**](SourceStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_source_used_by_list

> Vec<models::UsedBy> stages_source_used_by_list(stage_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this Source Stage. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_delete_create

> models::UserDeleteStage stages_user_delete_create(user_delete_stage_request)


UserDeleteStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_delete_stage_request** | [**UserDeleteStageRequest**](UserDeleteStageRequest.md) |  | [required] |

### Return type

[**models::UserDeleteStage**](UserDeleteStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_delete_destroy

> stages_user_delete_destroy(stage_uuid)


UserDeleteStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this User Delete Stage. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_delete_list

> models::PaginatedUserDeleteStageList stages_user_delete_list(name, ordering, page, page_size, search, stage_uuid)


UserDeleteStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**stage_uuid** | Option<**uuid::Uuid**> |  |  |

### Return type

[**models::PaginatedUserDeleteStageList**](PaginatedUserDeleteStageList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_delete_partial_update

> models::UserDeleteStage stages_user_delete_partial_update(stage_uuid, patched_user_delete_stage_request)


UserDeleteStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this User Delete Stage. | [required] |
**patched_user_delete_stage_request** | Option<[**PatchedUserDeleteStageRequest**](PatchedUserDeleteStageRequest.md)> |  |  |

### Return type

[**models::UserDeleteStage**](UserDeleteStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_delete_retrieve

> models::UserDeleteStage stages_user_delete_retrieve(stage_uuid)


UserDeleteStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this User Delete Stage. | [required] |

### Return type

[**models::UserDeleteStage**](UserDeleteStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_delete_update

> models::UserDeleteStage stages_user_delete_update(stage_uuid, user_delete_stage_request)


UserDeleteStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this User Delete Stage. | [required] |
**user_delete_stage_request** | [**UserDeleteStageRequest**](UserDeleteStageRequest.md) |  | [required] |

### Return type

[**models::UserDeleteStage**](UserDeleteStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_delete_used_by_list

> Vec<models::UsedBy> stages_user_delete_used_by_list(stage_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this User Delete Stage. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_login_create

> models::UserLoginStage stages_user_login_create(user_login_stage_request)


UserLoginStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_login_stage_request** | [**UserLoginStageRequest**](UserLoginStageRequest.md) |  | [required] |

### Return type

[**models::UserLoginStage**](UserLoginStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_login_destroy

> stages_user_login_destroy(stage_uuid)


UserLoginStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this User Login Stage. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_login_list

> models::PaginatedUserLoginStageList stages_user_login_list(geoip_binding, name, network_binding, ordering, page, page_size, remember_me_offset, search, session_duration, stage_uuid, terminate_other_sessions)


UserLoginStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**geoip_binding** | Option<**String**> | Bind sessions created by this stage to the configured GeoIP location   |  |
**name** | Option<**String**> |  |  |
**network_binding** | Option<**String**> | Bind sessions created by this stage to the configured network   |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**remember_me_offset** | Option<**String**> |  |  |
**search** | Option<**String**> | A search term. |  |
**session_duration** | Option<**String**> |  |  |
**stage_uuid** | Option<**uuid::Uuid**> |  |  |
**terminate_other_sessions** | Option<**bool**> |  |  |

### Return type

[**models::PaginatedUserLoginStageList**](PaginatedUserLoginStageList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_login_partial_update

> models::UserLoginStage stages_user_login_partial_update(stage_uuid, patched_user_login_stage_request)


UserLoginStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this User Login Stage. | [required] |
**patched_user_login_stage_request** | Option<[**PatchedUserLoginStageRequest**](PatchedUserLoginStageRequest.md)> |  |  |

### Return type

[**models::UserLoginStage**](UserLoginStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_login_retrieve

> models::UserLoginStage stages_user_login_retrieve(stage_uuid)


UserLoginStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this User Login Stage. | [required] |

### Return type

[**models::UserLoginStage**](UserLoginStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_login_update

> models::UserLoginStage stages_user_login_update(stage_uuid, user_login_stage_request)


UserLoginStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this User Login Stage. | [required] |
**user_login_stage_request** | [**UserLoginStageRequest**](UserLoginStageRequest.md) |  | [required] |

### Return type

[**models::UserLoginStage**](UserLoginStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_login_used_by_list

> Vec<models::UsedBy> stages_user_login_used_by_list(stage_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this User Login Stage. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_logout_create

> models::UserLogoutStage stages_user_logout_create(user_logout_stage_request)


UserLogoutStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_logout_stage_request** | [**UserLogoutStageRequest**](UserLogoutStageRequest.md) |  | [required] |

### Return type

[**models::UserLogoutStage**](UserLogoutStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_logout_destroy

> stages_user_logout_destroy(stage_uuid)


UserLogoutStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this User Logout Stage. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_logout_list

> models::PaginatedUserLogoutStageList stages_user_logout_list(name, ordering, page, page_size, search, stage_uuid)


UserLogoutStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**stage_uuid** | Option<**uuid::Uuid**> |  |  |

### Return type

[**models::PaginatedUserLogoutStageList**](PaginatedUserLogoutStageList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_logout_partial_update

> models::UserLogoutStage stages_user_logout_partial_update(stage_uuid, patched_user_logout_stage_request)


UserLogoutStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this User Logout Stage. | [required] |
**patched_user_logout_stage_request** | Option<[**PatchedUserLogoutStageRequest**](PatchedUserLogoutStageRequest.md)> |  |  |

### Return type

[**models::UserLogoutStage**](UserLogoutStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_logout_retrieve

> models::UserLogoutStage stages_user_logout_retrieve(stage_uuid)


UserLogoutStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this User Logout Stage. | [required] |

### Return type

[**models::UserLogoutStage**](UserLogoutStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_logout_update

> models::UserLogoutStage stages_user_logout_update(stage_uuid, user_logout_stage_request)


UserLogoutStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this User Logout Stage. | [required] |
**user_logout_stage_request** | [**UserLogoutStageRequest**](UserLogoutStageRequest.md) |  | [required] |

### Return type

[**models::UserLogoutStage**](UserLogoutStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_logout_used_by_list

> Vec<models::UsedBy> stages_user_logout_used_by_list(stage_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this User Logout Stage. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_write_create

> models::UserWriteStage stages_user_write_create(user_write_stage_request)


UserWriteStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_write_stage_request** | [**UserWriteStageRequest**](UserWriteStageRequest.md) |  | [required] |

### Return type

[**models::UserWriteStage**](UserWriteStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_write_destroy

> stages_user_write_destroy(stage_uuid)


UserWriteStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this User Write Stage. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_write_list

> models::PaginatedUserWriteStageList stages_user_write_list(create_users_as_inactive, create_users_group, name, ordering, page, page_size, search, stage_uuid, user_creation_mode, user_path_template, user_type)


UserWriteStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_users_as_inactive** | Option<**bool**> |  |  |
**create_users_group** | Option<**uuid::Uuid**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**stage_uuid** | Option<**uuid::Uuid**> |  |  |
**user_creation_mode** | Option<**String**> |  |  |
**user_path_template** | Option<**String**> |  |  |
**user_type** | Option<**String**> |  |  |

### Return type

[**models::PaginatedUserWriteStageList**](PaginatedUserWriteStageList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_write_partial_update

> models::UserWriteStage stages_user_write_partial_update(stage_uuid, patched_user_write_stage_request)


UserWriteStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this User Write Stage. | [required] |
**patched_user_write_stage_request** | Option<[**PatchedUserWriteStageRequest**](PatchedUserWriteStageRequest.md)> |  |  |

### Return type

[**models::UserWriteStage**](UserWriteStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_write_retrieve

> models::UserWriteStage stages_user_write_retrieve(stage_uuid)


UserWriteStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this User Write Stage. | [required] |

### Return type

[**models::UserWriteStage**](UserWriteStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_write_update

> models::UserWriteStage stages_user_write_update(stage_uuid, user_write_stage_request)


UserWriteStage Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this User Write Stage. | [required] |
**user_write_stage_request** | [**UserWriteStageRequest**](UserWriteStageRequest.md) |  | [required] |

### Return type

[**models::UserWriteStage**](UserWriteStage.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stages_user_write_used_by_list

> Vec<models::UsedBy> stages_user_write_used_by_list(stage_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stage_uuid** | **uuid::Uuid** | A UUID string identifying this User Write Stage. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

