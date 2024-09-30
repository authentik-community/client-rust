# DockerServiceConnection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**name** | **String** |  | 
**local** | Option<**bool**> | If enabled, use the local connection. Required Docker socket/Kubernetes Integration | [optional]
**component** | **String** |  | [readonly]
**verbose_name** | **String** | Return object's verbose_name | [readonly]
**verbose_name_plural** | **String** | Return object's plural verbose_name | [readonly]
**meta_model_name** | **String** | Return internal model name | [readonly]
**url** | **String** | Can be in the format of 'unix://<path>' when connecting to a local docker daemon, or 'https://<hostname>:2376' when connecting to a remote system. | 
**tls_verification** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | CA which the endpoint's Certificate is verified against. Can be left empty for no validation. | [optional]
**tls_authentication** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Certificate/Key used for authentication. Can be left empty for no authentication. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


