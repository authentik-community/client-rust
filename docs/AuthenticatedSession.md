# AuthenticatedSession

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**current** | **bool** | Check if session is currently active session | [readonly]
**user_agent** | [**models::AuthenticatedSessionUserAgent**](AuthenticatedSession_user_agent.md) |  | 
**geo_ip** | Option<[**models::AuthenticatedSessionGeoIp**](AuthenticatedSession_geo_ip.md)> |  | 
**asn** | Option<[**models::AuthenticatedSessionAsn**](AuthenticatedSession_asn.md)> |  | 
**user** | **i32** |  | 
**last_ip** | **String** |  | 
**last_user_agent** | Option<**String**> |  | [optional]
**last_used** | **String** |  | [readonly]
**expires** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


