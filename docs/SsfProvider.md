# SsfProvider

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**name** | **String** |  | 
**component** | **String** | Get object component so that we know how to edit the object | [readonly]
**verbose_name** | **String** | Return object's verbose_name | [readonly]
**verbose_name_plural** | **String** | Return object's plural verbose_name | [readonly]
**meta_model_name** | **String** | Return internal model name | [readonly]
**signing_key** | **uuid::Uuid** | Key used to sign the SSF Events. | 
**token_obj** | [**models::Token**](Token.md) |  | [readonly]
**oidc_auth_providers** | Option<**Vec<i32>**> |  | [optional]
**ssf_url** | Option<**String**> |  | [readonly]
**event_retention** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


