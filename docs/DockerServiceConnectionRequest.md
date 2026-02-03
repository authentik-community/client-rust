# DockerServiceConnectionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**local** | Option<**bool**> | If enabled, use the local connection. Required Docker socket/Kubernetes Integration | [optional]
**url** | **String** | Can be in the format of 'unix://<path>' when connecting to a local docker daemon, or 'https://<hostname>:2376' when connecting to a remote system. | 
**tls_verification** | Option<**uuid::Uuid**> | CA which the endpoint's Certificate is verified against. Can be left empty for no validation. | [optional]
**tls_authentication** | Option<**uuid::Uuid**> | Certificate/Key used for authentication. Can be left empty for no authentication. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


