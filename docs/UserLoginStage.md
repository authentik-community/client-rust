# UserLoginStage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**name** | **String** |  | 
**component** | **String** | Get object type so that we know how to edit the object | [readonly]
**verbose_name** | **String** | Return object's verbose_name | [readonly]
**verbose_name_plural** | **String** | Return object's plural verbose_name | [readonly]
**meta_model_name** | **String** | Return internal model name | [readonly]
**flow_set** | Option<[**Vec<models::FlowSet>**](FlowSet.md)> |  | [optional]
**session_duration** | Option<**String**> | Determines how long a session lasts. Default of 0 means that the sessions lasts until the browser is closed. (Format: hours=-1;minutes=-2;seconds=-3) | [optional]
**terminate_other_sessions** | Option<**bool**> | Terminate all other sessions of the user logging in. | [optional]
**remember_me_offset** | Option<**String**> | Offset the session will be extended by when the user picks the remember me option. Default of 0 means that the remember me option will not be shown. (Format: hours=-1;minutes=-2;seconds=-3) | [optional]
**network_binding** | Option<[**models::NetworkBindingEnum**](NetworkBindingEnum.md)> | Bind sessions created by this stage to the configured network | [optional]
**geoip_binding** | Option<[**models::GeoipBindingEnum**](GeoipBindingEnum.md)> | Bind sessions created by this stage to the configured GeoIP location | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


