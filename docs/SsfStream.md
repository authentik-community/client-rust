# SsfStream

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**provider** | **i32** |  | 
**provider_obj** | [**models::SsfProvider**](SSFProvider.md) |  | [readonly]
**delivery_method** | [**models::DeliveryMethodEnum**](DeliveryMethodEnum.md) |  | 
**endpoint_url** | Option<**String**> |  | [optional]
**events_requested** | Option<[**Vec<models::EventsRequestedEnum>**](EventsRequestedEnum.md)> |  | [optional]
**format** | **String** |  | 
**aud** | Option<**Vec<String>**> |  | [optional]
**iss** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


