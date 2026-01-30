# CurrentBrand

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**matched_domain** | **String** |  | 
**branding_title** | **String** |  | 
**branding_logo** | **String** |  | 
**branding_logo_themed_urls** | Option<[**models::ThemedUrls**](ThemedUrls.md)> |  | [readonly]
**branding_favicon** | **String** |  | 
**branding_favicon_themed_urls** | Option<[**models::ThemedUrls**](ThemedUrls.md)> |  | [readonly]
**branding_custom_css** | **String** |  | 
**ui_footer_links** | [**Vec<models::FooterLink>**](FooterLink.md) |  | [readonly]
**ui_theme** | [**models::UiThemeEnum**](UiThemeEnum.md) |  | [readonly][default to Automatic]
**flow_authentication** | Option<**String**> |  | [optional]
**flow_invalidation** | Option<**String**> |  | [optional]
**flow_recovery** | Option<**String**> |  | [optional]
**flow_unenrollment** | Option<**String**> |  | [optional]
**flow_user_settings** | Option<**String**> |  | [optional]
**flow_device_code** | Option<**String**> |  | [optional]
**default_locale** | **String** |  | [readonly]
**flags** | [**models::CurrentBrandFlags**](CurrentBrand_flags.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


