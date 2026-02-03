# UserWriteStage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **uuid::Uuid** |  | [readonly]
**name** | **String** |  | 
**component** | **String** | Get object type so that we know how to edit the object | [readonly]
**verbose_name** | **String** | Return object's verbose_name | [readonly]
**verbose_name_plural** | **String** | Return object's plural verbose_name | [readonly]
**meta_model_name** | **String** | Return internal model name | [readonly]
**flow_set** | [**Vec<models::FlowSet>**](FlowSet.md) |  | [readonly]
**user_creation_mode** | Option<[**models::UserCreationModeEnum**](UserCreationModeEnum.md)> |  | [optional]
**create_users_as_inactive** | Option<**bool**> | When set, newly created users are inactive and cannot login. | [optional]
**create_users_group** | Option<**uuid::Uuid**> | Optionally add newly created users to this group. | [optional]
**user_type** | Option<[**models::UserTypeEnum**](UserTypeEnum.md)> |  | [optional]
**user_path_template** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


