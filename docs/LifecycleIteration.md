# LifecycleIteration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [readonly]
**content_type** | [**models::ContentTypeEnum**](ContentTypeEnum.md) |  | 
**object_id** | **String** |  | [readonly]
**object_verbose** | **String** |  | [readonly]
**object_admin_url** | **String** |  | [readonly]
**state** | [**models::LifecycleIterationStateEnum**](LifecycleIterationStateEnum.md) |  | [readonly]
**opened_on** | [**String**](String.md) |  | [readonly]
**grace_period_end** | [**String**](String.md) |  | [readonly]
**next_review_date** | [**String**](String.md) |  | [readonly]
**reviews** | [**Vec<models::Review>**](Review.md) |  | [readonly]
**user_can_review** | **bool** |  | [readonly]
**reviewer_groups** | [**Vec<models::ReviewerGroup>**](ReviewerGroup.md) |  | [readonly]
**min_reviewers** | **i32** |  | [readonly]
**reviewers** | [**Vec<models::ReviewerUser>**](ReviewerUser.md) |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


