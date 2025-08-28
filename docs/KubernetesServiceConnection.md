# KubernetesServiceConnection

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
**kubeconfig** | Option<[**serde_json::Value**](.md)> | Paste your kubeconfig here. authentik will automatically use the currently selected context. | [optional]
**verify_ssl** | Option<**bool**> | Verify SSL Certificates of the Kubernetes API endpoint | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


