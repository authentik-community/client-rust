# TokenModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**provider** | [**models::OAuth2Provider**](OAuth2Provider.md) |  | 
**user** | [**models::User**](User.md) |  | 
**is_expired** | **bool** | Check if token is expired yet. | [readonly]
**expires** | Option<**String**> |  | [optional]
**scope** | **Vec<String>** |  | 
**id_token** | **String** | Get the token's id_token as JSON String | [readonly]
**revoked** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


