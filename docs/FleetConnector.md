# FleetConnector

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**connector_uuid** | Option<**uuid::Uuid**> |  | [optional]
**name** | **String** |  | 
**enabled** | Option<**bool**> |  | [optional]
**component** | **String** | Get object component so that we know how to edit the object | [readonly]
**verbose_name** | **String** | Return object's verbose_name | [readonly]
**verbose_name_plural** | **String** | Return object's plural verbose_name | [readonly]
**meta_model_name** | **String** | Return internal model name | [readonly]
**url** | **String** |  | 
**headers_mapping** | Option<**uuid::Uuid**> | Configure additional headers to be sent. Mapping should return a dictionary of key-value pairs | [optional]
**map_users** | Option<**bool**> |  | [optional]
**map_teams_access_group** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


