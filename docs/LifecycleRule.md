# LifecycleRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [readonly]
**name** | **String** |  | 
**content_type** | [**models::ContentTypeEnum**](ContentTypeEnum.md) |  | 
**object_id** | Option<**String**> |  | [optional]
**interval** | Option<**String**> |  | [optional]
**grace_period** | Option<**String**> |  | [optional]
**reviewer_groups** | Option<**Vec<uuid::Uuid>**> |  | [optional]
**reviewer_groups_obj** | [**Vec<models::ReviewerGroup>**](ReviewerGroup.md) |  | [readonly]
**min_reviewers** | Option<**u16**> |  | [optional]
**min_reviewers_is_per_group** | Option<**bool**> |  | [optional]
**reviewers** | **Vec<uuid::Uuid>** |  | 
**reviewers_obj** | [**Vec<models::ReviewerUser>**](ReviewerUser.md) |  | [readonly]
**notification_transports** | Option<**Vec<uuid::Uuid>**> | Select which transports should be used to notify the reviewers. If none are selected, the notification will only be shown in the authentik UI. | [optional]
**target_verbose** | **String** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


