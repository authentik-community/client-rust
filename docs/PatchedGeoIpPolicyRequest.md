# PatchedGeoIpPolicyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**execution_logging** | Option<**bool**> | When this option is enabled, all executions of this policy will be logged. By default, only execution errors are logged. | [optional]
**asns** | Option<**Vec<i32>**> |  | [optional]
**countries** | Option<[**Vec<models::CountryCodeEnum>**](CountryCodeEnum.md)> |  | [optional]
**check_history_distance** | Option<**bool**> |  | [optional]
**history_max_distance_km** | Option<**u64**> |  | [optional]
**distance_tolerance_km** | Option<**u32**> |  | [optional]
**history_login_count** | Option<**u32**> |  | [optional]
**check_impossible_travel** | Option<**bool**> |  | [optional]
**impossible_tolerance_km** | Option<**u32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


