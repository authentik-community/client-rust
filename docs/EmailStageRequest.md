# EmailStageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**flow_set** | Option<[**Vec<models::FlowSetRequest>**](FlowSetRequest.md)> |  | [optional]
**use_global_settings** | Option<**bool**> | When enabled, global Email connection settings will be used and connection settings below will be ignored. | [optional]
**host** | Option<**String**> |  | [optional]
**port** | Option<**i32**> |  | [optional]
**username** | Option<**String**> |  | [optional]
**password** | Option<**String**> |  | [optional]
**use_tls** | Option<**bool**> |  | [optional]
**use_ssl** | Option<**bool**> |  | [optional]
**timeout** | Option<**i32**> |  | [optional]
**from_address** | Option<**String**> |  | [optional]
**token_expiry** | Option<**String**> | Time the token sent is valid (Format: hours=3,minutes=17,seconds=300). | [optional]
**subject** | Option<**String**> |  | [optional]
**template** | Option<**String**> |  | [optional]
**activate_user_on_success** | Option<**bool**> | Activate users upon completion of stage. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


