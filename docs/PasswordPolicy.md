# PasswordPolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **uuid::Uuid** |  | [readonly]
**name** | **String** |  | 
**execution_logging** | Option<**bool**> | When this option is enabled, all executions of this policy will be logged. By default, only execution errors are logged. | [optional]
**component** | **String** | Get object component so that we know how to edit the object | [readonly]
**verbose_name** | **String** | Return object's verbose_name | [readonly]
**verbose_name_plural** | **String** | Return object's plural verbose_name | [readonly]
**meta_model_name** | **String** | Return internal model name | [readonly]
**bound_to** | **i32** | Return objects policy is bound to | [readonly]
**password_field** | Option<**String**> | Field key to check, field keys defined in Prompt stages are available. | [optional]
**amount_digits** | Option<**u32**> |  | [optional]
**amount_uppercase** | Option<**u32**> |  | [optional]
**amount_lowercase** | Option<**u32**> |  | [optional]
**amount_symbols** | Option<**u32**> |  | [optional]
**length_min** | Option<**u32**> |  | [optional]
**symbol_charset** | Option<**String**> |  | [optional]
**error_message** | Option<**String**> |  | [optional]
**check_static_rules** | Option<**bool**> |  | [optional]
**check_have_i_been_pwned** | Option<**bool**> |  | [optional]
**check_zxcvbn** | Option<**bool**> |  | [optional]
**hibp_allowed_count** | Option<**u32**> | How many times the password hash is allowed to be on haveibeenpwned | [optional]
**zxcvbn_score_threshold** | Option<**u32**> | If the zxcvbn score is equal or less than this value, the policy will fail. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


