# NativeLogoutChallenge

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**flow_info** | Option<[**models::ContextualFlowInfo**](ContextualFlowInfo.md)> |  | [optional]
**component** | Option<**String**> |  | [optional][default to ak-provider-saml-native-logout]
**response_errors** | Option<[**std::collections::HashMap<String, Vec<models::ErrorDetail>>**](Vec.md)> |  | [optional]
**provider_name** | Option<**String**> |  | [optional]
**is_complete** | Option<**bool**> |  | [optional][default to false]
**post_url** | Option<**String**> |  | [optional]
**redirect_url** | Option<**String**> |  | [optional]
**saml_binding** | Option<[**models::SamlBindingsEnum**](SAMLBindingsEnum.md)> |  | [optional]
**saml_request** | Option<**String**> |  | [optional]
**saml_response** | Option<**String**> |  | [optional]
**saml_relay_state** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


