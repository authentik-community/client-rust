# IdentificationChallenge

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**flow_info** | Option<[**models::ContextualFlowInfo**](ContextualFlowInfo.md)> |  | [optional]
**component** | Option<**String**> |  | [optional][default to ak-stage-identification]
**response_errors** | Option<[**std::collections::HashMap<String, Vec<models::ErrorDetail>>**](Vec.md)> |  | [optional]
**user_fields** | Option<**Vec<String>**> |  | 
**password_fields** | **bool** |  | 
**allow_show_password** | Option<**bool**> |  | [optional][default to false]
**application_pre** | Option<**String**> |  | [optional]
**flow_designation** | [**models::FlowDesignationEnum**](FlowDesignationEnum.md) |  | 
**captcha_stage** | Option<[**models::CaptchaChallenge**](CaptchaChallenge.md)> |  | [optional]
**enroll_url** | Option<**String**> |  | [optional]
**recovery_url** | Option<**String**> |  | [optional]
**passwordless_url** | Option<**String**> |  | [optional]
**primary_action** | **String** |  | 
**sources** | Option<[**Vec<models::LoginSource>**](LoginSource.md)> |  | [optional]
**show_source_labels** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


