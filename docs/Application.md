# Application

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**name** | **String** | Application's display Name. | 
**slug** | **String** | Internal application name, used in URLs. | 
**provider** | Option<**i32**> |  | [optional]
**provider_obj** | [**models::Provider**](Provider.md) |  | [readonly]
**backchannel_providers** | Option<**Vec<i32>**> |  | [optional]
**backchannel_providers_obj** | [**Vec<models::Provider>**](Provider.md) |  | [readonly]
**launch_url** | Option<**String**> | Allow formatting of launch URL | [readonly]
**open_in_new_tab** | Option<**bool**> | Open launch URL in a new browser tab or window. | [optional]
**meta_launch_url** | Option<**String**> |  | [optional]
**meta_icon** | Option<**String**> |  | [optional]
**meta_icon_url** | Option<**String**> | Get the URL to the App Icon image | [readonly]
**meta_icon_themed_urls** | Option<[**models::ThemedUrls**](ThemedUrls.md)> |  | [readonly]
**meta_description** | Option<**String**> |  | [optional]
**meta_publisher** | Option<**String**> |  | [optional]
**policy_engine_mode** | Option<[**models::PolicyEngineMode**](PolicyEngineMode.md)> |  | [optional]
**group** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


