# PatchedPlexSourceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Source's display Name. | [optional]
**slug** | Option<**String**> | Internal source name, used in URLs. | [optional]
**enabled** | Option<**bool**> |  | [optional]
**authentication_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow to use when authenticating existing users. | [optional]
**enrollment_flow** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Flow to use when enrolling new users. | [optional]
**user_property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**group_property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**policy_engine_mode** | Option<[**models::PolicyEngineMode**](PolicyEngineMode.md)> |  | [optional]
**user_matching_mode** | Option<[**models::UserMatchingModeEnum**](UserMatchingModeEnum.md)> | How the source determines if an existing user should be authenticated or a new user enrolled. | [optional]
**user_path_template** | Option<**String**> |  | [optional]
**group_matching_mode** | Option<[**models::GroupMatchingModeEnum**](GroupMatchingModeEnum.md)> | How the source determines if an existing group should be used or a new group created. | [optional]
**client_id** | Option<**String**> | Client identifier used to talk to Plex. | [optional]
**allowed_servers** | Option<**Vec<String>**> | Which servers a user has to be a member of to be granted access. Empty list allows every server. | [optional]
**allow_friends** | Option<**bool**> | Allow friends to authenticate, even if you don't share a server. | [optional]
**plex_token** | Option<**String**> | Plex token used to check friends | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


