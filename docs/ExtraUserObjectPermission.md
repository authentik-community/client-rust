# ExtraUserObjectPermission

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**codename** | **String** |  | [readonly]
**model** | **String** |  | [readonly]
**app_label** | **String** |  | [readonly]
**object_pk** | **String** |  | 
**name** | **String** |  | [readonly]
**app_label_verbose** | **String** | Get app label from permission's model | [readonly]
**model_verbose** | **String** | Get model label from permission's model | [readonly]
**object_description** | Option<**String**> | Get model description from attached model. This operation takes at least one additional query, and the description is only shown if the user/role has the view_ permission on the object | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


