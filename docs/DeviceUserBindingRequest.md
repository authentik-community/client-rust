# DeviceUserBindingRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**policy** | Option<**uuid::Uuid**> |  | [optional]
**group** | Option<**uuid::Uuid**> |  | [optional]
**user** | Option<**i32**> |  | [optional]
**target** | **uuid::Uuid** |  | 
**negate** | Option<**bool**> | Negates the outcome of the policy. Messages are unaffected. | [optional]
**enabled** | Option<**bool**> |  | [optional]
**order** | **i32** |  | 
**timeout** | Option<**u32**> | Timeout after which Policy execution is terminated. | [optional]
**failure_result** | Option<**bool**> | Result if the Policy execution fails. | [optional]
**is_primary** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


