# \ReportsApi

All URIs are relative to */api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**reports_exports_destroy**](ReportsApi.md#reports_exports_destroy) | **DELETE** /reports/exports/{id}/ | 
[**reports_exports_list**](ReportsApi.md#reports_exports_list) | **GET** /reports/exports/ | 
[**reports_exports_retrieve**](ReportsApi.md#reports_exports_retrieve) | **GET** /reports/exports/{id}/ | 



## reports_exports_destroy

> reports_exports_destroy(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this Data Export. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reports_exports_list

> models::PaginatedDataExportList reports_exports_list(ordering, page, page_size, search)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedDataExportList**](PaginatedDataExportList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reports_exports_retrieve

> models::DataExport reports_exports_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this Data Export. | [required] |

### Return type

[**models::DataExport**](DataExport.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

