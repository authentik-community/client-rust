# PatchedUserLoginStageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**session_duration** | Option<**String**> | Determines how long a session lasts. Default of 0 means that the sessions lasts until the browser is closed. (Format: hours=-1;minutes=-2;seconds=-3) | [optional]
**terminate_other_sessions** | Option<**bool**> | Terminate all other sessions of the user logging in. | [optional]
**remember_me_offset** | Option<**String**> | Offset the session will be extended by when the user picks the remember me option. Default of 0 means that the remember me option will not be shown. (Format: hours=-1;minutes=-2;seconds=-3) | [optional]
**network_binding** | Option<[**models::NetworkBindingEnum**](NetworkBindingEnum.md)> | Bind sessions created by this stage to the configured network | [optional]
**geoip_binding** | Option<[**models::GeoipBindingEnum**](GeoipBindingEnum.md)> | Bind sessions created by this stage to the configured GeoIP location | [optional]
**remember_device** | Option<**String**> | When set to a non-zero value, authentik will save a cookie with a longer expiry,to remember the device the user is logging in from. (Format: hours=-1;minutes=-2;seconds=-3) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


