# GeoIpPolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**name** | **String** |  | 
**execution_logging** | Option<**bool**> | When this option is enabled, all executions of this policy will be logged. By default, only execution errors are logged. | [optional]
**component** | **String** | Get object component so that we know how to edit the object | [readonly]
**verbose_name** | **String** | Return object's verbose_name | [readonly]
**verbose_name_plural** | **String** | Return object's plural verbose_name | [readonly]
**meta_model_name** | **String** | Return internal model name | [readonly]
**bound_to** | **i32** | Return objects policy is bound to | [readonly]
**asns** | Option<**Vec<i32>**> |  | [optional]
**countries** | [**Vec<models::CountryCodeEnum>**](CountryCodeEnum.md) |  | 
**countries_obj** | [**Vec<models::DetailedCountryField>**](DetailedCountryField.md) |  | [readonly]
**check_history_distance** | Option<**bool**> |  | [optional]
**history_max_distance_km** | Option<**u64**> |  | [optional]
**distance_tolerance_km** | Option<**u32**> |  | [optional]
**history_login_count** | Option<**u32**> |  | [optional]
**check_impossible_travel** | Option<**bool**> |  | [optional]
**impossible_tolerance_km** | Option<**u32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


