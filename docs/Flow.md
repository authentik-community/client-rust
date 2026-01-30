# Flow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**policybindingmodel_ptr_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**name** | **String** |  | 
**slug** | **String** | Visible in the URL. | 
**title** | **String** | Shown as the Title in Flow pages. | 
**designation** | [**models::FlowDesignationEnum**](FlowDesignationEnum.md) | Decides what this Flow is used for. For example, the Authentication flow is redirect to when an un-authenticated user visits authentik. | 
**background** | Option<**String**> | Background shown during execution | [optional]
**background_url** | **String** | Get the URL to the background image | [readonly]
**background_themed_urls** | Option<[**models::ThemedUrls**](ThemedUrls.md)> |  | [readonly]
**stages** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) |  | [readonly]
**policies** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) |  | [readonly]
**cache_count** | **i32** | Get count of cached flows | [readonly]
**policy_engine_mode** | Option<[**models::PolicyEngineMode**](PolicyEngineMode.md)> |  | [optional]
**compatibility_mode** | Option<**bool**> | Enable compatibility mode, increases compatibility with password managers on mobile devices. | [optional]
**export_url** | **String** | Get export URL for flow | [readonly]
**layout** | Option<[**models::FlowLayoutEnum**](FlowLayoutEnum.md)> |  | [optional]
**denied_action** | Option<[**models::DeniedActionEnum**](DeniedActionEnum.md)> | Configure what should happen when a flow denies access to a user. | [optional]
**authentication** | Option<[**models::AuthenticationEnum**](AuthenticationEnum.md)> | Required level of authentication and authorization to access a flow. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


