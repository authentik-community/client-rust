# PatchedUserWriteStageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**user_creation_mode** | Option<[**models::UserCreationModeEnum**](UserCreationModeEnum.md)> |  | [optional]
**create_users_as_inactive** | Option<**bool**> | When set, newly created users are inactive and cannot login. | [optional]
**create_users_group** | Option<**uuid::Uuid**> | Optionally add newly created users to this group. | [optional]
**user_type** | Option<[**models::UserTypeEnum**](UserTypeEnum.md)> |  | [optional]
**user_path_template** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


