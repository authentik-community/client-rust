# KubernetesServiceConnectionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**local** | Option<**bool**> | If enabled, use the local connection. Required Docker socket/Kubernetes Integration | [optional]
**kubeconfig** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Paste your kubeconfig here. authentik will automatically use the currently selected context. | [optional]
**verify_ssl** | Option<**bool**> | Verify SSL Certificates of the Kubernetes API endpoint | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


