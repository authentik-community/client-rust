# FlowRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**slug** | **String** | Visible in the URL. | 
**title** | **String** | Shown as the Title in Flow pages. | 
**designation** | [**models::FlowDesignationEnum**](FlowDesignationEnum.md) | Decides what this Flow is used for. For example, the Authentication flow is redirect to when an un-authenticated user visits authentik. | 
**background** | Option<**String**> | Background shown during execution | [optional]
**policy_engine_mode** | Option<[**models::PolicyEngineMode**](PolicyEngineMode.md)> |  | [optional]
**compatibility_mode** | Option<**bool**> | Enable compatibility mode, increases compatibility with password managers on mobile devices. | [optional]
**layout** | Option<[**models::FlowLayoutEnum**](FlowLayoutEnum.md)> |  | [optional]
**denied_action** | Option<[**models::DeniedActionEnum**](DeniedActionEnum.md)> | Configure what should happen when a flow denies access to a user. | [optional]
**authentication** | Option<[**models::AuthenticationEnum**](AuthenticationEnum.md)> | Required level of authentication and authorization to access a flow. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


