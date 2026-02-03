# AuthenticatedSession

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uuid** | Option<**uuid::Uuid**> |  | [optional]
**current** | **bool** | Check if session is currently active session | [readonly]
**user_agent** | [**models::AuthenticatedSessionUserAgent**](AuthenticatedSessionUserAgent.md) |  | 
**geo_ip** | Option<[**models::AuthenticatedSessionGeoIp**](AuthenticatedSessionGeoIp.md)> |  | 
**asn** | Option<[**models::AuthenticatedSessionAsn**](AuthenticatedSessionAsn.md)> |  | 
**user** | **i32** |  | 
**last_ip** | **String** |  | [readonly]
**last_user_agent** | **String** |  | [readonly]
**last_used** | **String** |  | [readonly]
**expires** | **String** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


