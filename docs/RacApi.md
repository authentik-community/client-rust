# \RacApi

All URIs are relative to */api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rac_connection_tokens_destroy**](RacApi.md#rac_connection_tokens_destroy) | **DELETE** /rac/connection_tokens/{connection_token_uuid}/ | 
[**rac_connection_tokens_list**](RacApi.md#rac_connection_tokens_list) | **GET** /rac/connection_tokens/ | 
[**rac_connection_tokens_partial_update**](RacApi.md#rac_connection_tokens_partial_update) | **PATCH** /rac/connection_tokens/{connection_token_uuid}/ | 
[**rac_connection_tokens_retrieve**](RacApi.md#rac_connection_tokens_retrieve) | **GET** /rac/connection_tokens/{connection_token_uuid}/ | 
[**rac_connection_tokens_update**](RacApi.md#rac_connection_tokens_update) | **PUT** /rac/connection_tokens/{connection_token_uuid}/ | 
[**rac_connection_tokens_used_by_list**](RacApi.md#rac_connection_tokens_used_by_list) | **GET** /rac/connection_tokens/{connection_token_uuid}/used_by/ | 
[**rac_endpoints_create**](RacApi.md#rac_endpoints_create) | **POST** /rac/endpoints/ | 
[**rac_endpoints_destroy**](RacApi.md#rac_endpoints_destroy) | **DELETE** /rac/endpoints/{pbm_uuid}/ | 
[**rac_endpoints_list**](RacApi.md#rac_endpoints_list) | **GET** /rac/endpoints/ | 
[**rac_endpoints_partial_update**](RacApi.md#rac_endpoints_partial_update) | **PATCH** /rac/endpoints/{pbm_uuid}/ | 
[**rac_endpoints_retrieve**](RacApi.md#rac_endpoints_retrieve) | **GET** /rac/endpoints/{pbm_uuid}/ | 
[**rac_endpoints_update**](RacApi.md#rac_endpoints_update) | **PUT** /rac/endpoints/{pbm_uuid}/ | 
[**rac_endpoints_used_by_list**](RacApi.md#rac_endpoints_used_by_list) | **GET** /rac/endpoints/{pbm_uuid}/used_by/ | 



## rac_connection_tokens_destroy

> rac_connection_tokens_destroy(connection_token_uuid)


ConnectionToken Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_token_uuid** | **uuid::Uuid** | A UUID string identifying this RAC Connection token. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rac_connection_tokens_list

> models::PaginatedConnectionTokenList rac_connection_tokens_list(endpoint, ordering, page, page_size, provider, search, session__user)


ConnectionToken Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint** | Option<**uuid::Uuid**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**provider** | Option<**i32**> |  |  |
**search** | Option<**String**> | A search term. |  |
**session__user** | Option<**i32**> |  |  |

### Return type

[**models::PaginatedConnectionTokenList**](PaginatedConnectionTokenList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rac_connection_tokens_partial_update

> models::ConnectionToken rac_connection_tokens_partial_update(connection_token_uuid, patched_connection_token_request)


ConnectionToken Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_token_uuid** | **uuid::Uuid** | A UUID string identifying this RAC Connection token. | [required] |
**patched_connection_token_request** | Option<[**PatchedConnectionTokenRequest**](PatchedConnectionTokenRequest.md)> |  |  |

### Return type

[**models::ConnectionToken**](ConnectionToken.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rac_connection_tokens_retrieve

> models::ConnectionToken rac_connection_tokens_retrieve(connection_token_uuid)


ConnectionToken Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_token_uuid** | **uuid::Uuid** | A UUID string identifying this RAC Connection token. | [required] |

### Return type

[**models::ConnectionToken**](ConnectionToken.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rac_connection_tokens_update

> models::ConnectionToken rac_connection_tokens_update(connection_token_uuid, connection_token_request)


ConnectionToken Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_token_uuid** | **uuid::Uuid** | A UUID string identifying this RAC Connection token. | [required] |
**connection_token_request** | [**ConnectionTokenRequest**](ConnectionTokenRequest.md) |  | [required] |

### Return type

[**models::ConnectionToken**](ConnectionToken.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rac_connection_tokens_used_by_list

> Vec<models::UsedBy> rac_connection_tokens_used_by_list(connection_token_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_token_uuid** | **uuid::Uuid** | A UUID string identifying this RAC Connection token. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rac_endpoints_create

> models::Endpoint rac_endpoints_create(endpoint_request)


Endpoint Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_request** | [**EndpointRequest**](EndpointRequest.md) |  | [required] |

### Return type

[**models::Endpoint**](Endpoint.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rac_endpoints_destroy

> rac_endpoints_destroy(pbm_uuid)


Endpoint Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pbm_uuid** | **uuid::Uuid** | A UUID string identifying this RAC Endpoint. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rac_endpoints_list

> models::PaginatedEndpointList rac_endpoints_list(name, ordering, page, page_size, provider, search, superuser_full_list)


List accessible endpoints

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**provider** | Option<**i32**> |  |  |
**search** | Option<**String**> |  |  |
**superuser_full_list** | Option<**bool**> |  |  |

### Return type

[**models::PaginatedEndpointList**](PaginatedEndpointList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rac_endpoints_partial_update

> models::Endpoint rac_endpoints_partial_update(pbm_uuid, patched_endpoint_request)


Endpoint Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pbm_uuid** | **uuid::Uuid** | A UUID string identifying this RAC Endpoint. | [required] |
**patched_endpoint_request** | Option<[**PatchedEndpointRequest**](PatchedEndpointRequest.md)> |  |  |

### Return type

[**models::Endpoint**](Endpoint.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rac_endpoints_retrieve

> models::Endpoint rac_endpoints_retrieve(pbm_uuid)


Endpoint Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pbm_uuid** | **uuid::Uuid** | A UUID string identifying this RAC Endpoint. | [required] |

### Return type

[**models::Endpoint**](Endpoint.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rac_endpoints_update

> models::Endpoint rac_endpoints_update(pbm_uuid, endpoint_request)


Endpoint Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pbm_uuid** | **uuid::Uuid** | A UUID string identifying this RAC Endpoint. | [required] |
**endpoint_request** | [**EndpointRequest**](EndpointRequest.md) |  | [required] |

### Return type

[**models::Endpoint**](Endpoint.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rac_endpoints_used_by_list

> Vec<models::UsedBy> rac_endpoints_used_by_list(pbm_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pbm_uuid** | **uuid::Uuid** | A UUID string identifying this RAC Endpoint. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

