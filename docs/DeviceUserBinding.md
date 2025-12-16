# DeviceUserBinding

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**policy** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**group** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**user** | Option<**i32**> |  | [optional]
**policy_obj** | [**models::Policy**](Policy.md) |  | [readonly]
**group_obj** | [**models::PartialGroup**](PartialGroup.md) |  | [readonly]
**user_obj** | [**models::PartialUser**](PartialUser.md) |  | [readonly]
**target** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**negate** | Option<**bool**> | Negates the outcome of the policy. Messages are unaffected. | [optional]
**enabled** | Option<**bool**> |  | [optional]
**order** | **i32** |  | 
**timeout** | Option<**u32**> | Timeout after which Policy execution is terminated. | [optional]
**failure_result** | Option<**bool**> | Result if the Policy execution fails. | [optional]
**is_primary** | Option<**bool**> |  | [optional]
**connector** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [readonly]
**connector_obj** | [**models::Connector**](Connector.md) |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


