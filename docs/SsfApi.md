# \SsfApi

All URIs are relative to */api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ssf_streams_list**](SsfApi.md#ssf_streams_list) | **GET** /ssf/streams/ | 
[**ssf_streams_retrieve**](SsfApi.md#ssf_streams_retrieve) | **GET** /ssf/streams/{uuid}/ | 



## ssf_streams_list

> models::PaginatedSsfStreamList ssf_streams_list(delivery_method, endpoint_url, ordering, page, page_size, provider, search)


SSFStream Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delivery_method** | Option<**String**> |  |  |
**endpoint_url** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**provider** | Option<**i32**> |  |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedSsfStreamList**](PaginatedSSFStreamList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ssf_streams_retrieve

> models::SsfStream ssf_streams_retrieve(uuid)


SSFStream Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this SSF Stream. | [required] |

### Return type

[**models::SsfStream**](SSFStream.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

