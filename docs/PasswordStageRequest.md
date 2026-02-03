# PasswordStageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**backends** | [**Vec<models::BackendsEnum>**](BackendsEnum.md) | Selection of backends to test the password against. | 
**configure_flow** | Option<**uuid::Uuid**> | Flow used by an authenticated user to configure this Stage. If empty, user will not be able to configure this stage. | [optional]
**failed_attempts_before_cancel** | Option<**i32**> | How many attempts a user has before the flow is canceled. To lock the user out, use a reputation policy and a user_write stage. | [optional]
**allow_show_password** | Option<**bool**> | When enabled, provides a 'show password' button with the password input field. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


