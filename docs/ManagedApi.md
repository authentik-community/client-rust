# \ManagedApi

All URIs are relative to *http://localhost/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**managed_blueprints_apply_create**](ManagedApi.md#managed_blueprints_apply_create) | **POST** /managed/blueprints/{instance_uuid}/apply/ | 
[**managed_blueprints_available_list**](ManagedApi.md#managed_blueprints_available_list) | **GET** /managed/blueprints/available/ | 
[**managed_blueprints_create**](ManagedApi.md#managed_blueprints_create) | **POST** /managed/blueprints/ | 
[**managed_blueprints_destroy**](ManagedApi.md#managed_blueprints_destroy) | **DELETE** /managed/blueprints/{instance_uuid}/ | 
[**managed_blueprints_list**](ManagedApi.md#managed_blueprints_list) | **GET** /managed/blueprints/ | 
[**managed_blueprints_partial_update**](ManagedApi.md#managed_blueprints_partial_update) | **PATCH** /managed/blueprints/{instance_uuid}/ | 
[**managed_blueprints_retrieve**](ManagedApi.md#managed_blueprints_retrieve) | **GET** /managed/blueprints/{instance_uuid}/ | 
[**managed_blueprints_update**](ManagedApi.md#managed_blueprints_update) | **PUT** /managed/blueprints/{instance_uuid}/ | 
[**managed_blueprints_used_by_list**](ManagedApi.md#managed_blueprints_used_by_list) | **GET** /managed/blueprints/{instance_uuid}/used_by/ | 



## managed_blueprints_apply_create

> models::BlueprintInstance managed_blueprints_apply_create(instance_uuid)


Apply a blueprint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_uuid** | **uuid::Uuid** | A UUID string identifying this Blueprint Instance. | [required] |

### Return type

[**models::BlueprintInstance**](BlueprintInstance.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## managed_blueprints_available_list

> Vec<models::BlueprintFile> managed_blueprints_available_list()


Get blueprints

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::BlueprintFile>**](BlueprintFile.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## managed_blueprints_create

> models::BlueprintInstance managed_blueprints_create(blueprint_instance_request)


Blueprint instances

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blueprint_instance_request** | [**BlueprintInstanceRequest**](BlueprintInstanceRequest.md) |  | [required] |

### Return type

[**models::BlueprintInstance**](BlueprintInstance.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## managed_blueprints_destroy

> managed_blueprints_destroy(instance_uuid)


Blueprint instances

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_uuid** | **uuid::Uuid** | A UUID string identifying this Blueprint Instance. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## managed_blueprints_list

> models::PaginatedBlueprintInstanceList managed_blueprints_list(name, ordering, page, page_size, path, search)


Blueprint instances

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**path** | Option<**String**> |  |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedBlueprintInstanceList**](PaginatedBlueprintInstanceList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## managed_blueprints_partial_update

> models::BlueprintInstance managed_blueprints_partial_update(instance_uuid, patched_blueprint_instance_request)


Blueprint instances

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_uuid** | **uuid::Uuid** | A UUID string identifying this Blueprint Instance. | [required] |
**patched_blueprint_instance_request** | Option<[**PatchedBlueprintInstanceRequest**](PatchedBlueprintInstanceRequest.md)> |  |  |

### Return type

[**models::BlueprintInstance**](BlueprintInstance.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## managed_blueprints_retrieve

> models::BlueprintInstance managed_blueprints_retrieve(instance_uuid)


Blueprint instances

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_uuid** | **uuid::Uuid** | A UUID string identifying this Blueprint Instance. | [required] |

### Return type

[**models::BlueprintInstance**](BlueprintInstance.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## managed_blueprints_update

> models::BlueprintInstance managed_blueprints_update(instance_uuid, blueprint_instance_request)


Blueprint instances

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_uuid** | **uuid::Uuid** | A UUID string identifying this Blueprint Instance. | [required] |
**blueprint_instance_request** | [**BlueprintInstanceRequest**](BlueprintInstanceRequest.md) |  | [required] |

### Return type

[**models::BlueprintInstance**](BlueprintInstance.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## managed_blueprints_used_by_list

> Vec<models::UsedBy> managed_blueprints_used_by_list(instance_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_uuid** | **uuid::Uuid** | A UUID string identifying this Blueprint Instance. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

