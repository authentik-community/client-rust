# \Oauth2Api

All URIs are relative to *http://localhost/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**oauth2_access_tokens_destroy**](Oauth2Api.md#oauth2_access_tokens_destroy) | **DELETE** /oauth2/access_tokens/{id}/ | 
[**oauth2_access_tokens_list**](Oauth2Api.md#oauth2_access_tokens_list) | **GET** /oauth2/access_tokens/ | 
[**oauth2_access_tokens_retrieve**](Oauth2Api.md#oauth2_access_tokens_retrieve) | **GET** /oauth2/access_tokens/{id}/ | 
[**oauth2_access_tokens_used_by_list**](Oauth2Api.md#oauth2_access_tokens_used_by_list) | **GET** /oauth2/access_tokens/{id}/used_by/ | 
[**oauth2_authorization_codes_destroy**](Oauth2Api.md#oauth2_authorization_codes_destroy) | **DELETE** /oauth2/authorization_codes/{id}/ | 
[**oauth2_authorization_codes_list**](Oauth2Api.md#oauth2_authorization_codes_list) | **GET** /oauth2/authorization_codes/ | 
[**oauth2_authorization_codes_retrieve**](Oauth2Api.md#oauth2_authorization_codes_retrieve) | **GET** /oauth2/authorization_codes/{id}/ | 
[**oauth2_authorization_codes_used_by_list**](Oauth2Api.md#oauth2_authorization_codes_used_by_list) | **GET** /oauth2/authorization_codes/{id}/used_by/ | 
[**oauth2_refresh_tokens_destroy**](Oauth2Api.md#oauth2_refresh_tokens_destroy) | **DELETE** /oauth2/refresh_tokens/{id}/ | 
[**oauth2_refresh_tokens_list**](Oauth2Api.md#oauth2_refresh_tokens_list) | **GET** /oauth2/refresh_tokens/ | 
[**oauth2_refresh_tokens_retrieve**](Oauth2Api.md#oauth2_refresh_tokens_retrieve) | **GET** /oauth2/refresh_tokens/{id}/ | 
[**oauth2_refresh_tokens_used_by_list**](Oauth2Api.md#oauth2_refresh_tokens_used_by_list) | **GET** /oauth2/refresh_tokens/{id}/used_by/ | 



## oauth2_access_tokens_destroy

> oauth2_access_tokens_destroy(id)


AccessToken Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this OAuth2 Access Token. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth2_access_tokens_list

> models::PaginatedTokenModelList oauth2_access_tokens_list(ordering, page, page_size, provider, search, user)


AccessToken Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**provider** | Option<**i32**> |  |  |
**search** | Option<**String**> | A search term. |  |
**user** | Option<**i32**> |  |  |

### Return type

[**models::PaginatedTokenModelList**](PaginatedTokenModelList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth2_access_tokens_retrieve

> models::TokenModel oauth2_access_tokens_retrieve(id)


AccessToken Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this OAuth2 Access Token. | [required] |

### Return type

[**models::TokenModel**](TokenModel.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth2_access_tokens_used_by_list

> Vec<models::UsedBy> oauth2_access_tokens_used_by_list(id)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this OAuth2 Access Token. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth2_authorization_codes_destroy

> oauth2_authorization_codes_destroy(id)


AuthorizationCode Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Authorization Code. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth2_authorization_codes_list

> models::PaginatedExpiringBaseGrantModelList oauth2_authorization_codes_list(ordering, page, page_size, provider, search, user)


AuthorizationCode Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**provider** | Option<**i32**> |  |  |
**search** | Option<**String**> | A search term. |  |
**user** | Option<**i32**> |  |  |

### Return type

[**models::PaginatedExpiringBaseGrantModelList**](PaginatedExpiringBaseGrantModelList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth2_authorization_codes_retrieve

> models::ExpiringBaseGrantModel oauth2_authorization_codes_retrieve(id)


AuthorizationCode Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Authorization Code. | [required] |

### Return type

[**models::ExpiringBaseGrantModel**](ExpiringBaseGrantModel.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth2_authorization_codes_used_by_list

> Vec<models::UsedBy> oauth2_authorization_codes_used_by_list(id)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Authorization Code. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth2_refresh_tokens_destroy

> oauth2_refresh_tokens_destroy(id)


RefreshToken Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this OAuth2 Refresh Token. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth2_refresh_tokens_list

> models::PaginatedTokenModelList oauth2_refresh_tokens_list(ordering, page, page_size, provider, search, user)


RefreshToken Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**provider** | Option<**i32**> |  |  |
**search** | Option<**String**> | A search term. |  |
**user** | Option<**i32**> |  |  |

### Return type

[**models::PaginatedTokenModelList**](PaginatedTokenModelList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth2_refresh_tokens_retrieve

> models::TokenModel oauth2_refresh_tokens_retrieve(id)


RefreshToken Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this OAuth2 Refresh Token. | [required] |

### Return type

[**models::TokenModel**](TokenModel.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth2_refresh_tokens_used_by_list

> Vec<models::UsedBy> oauth2_refresh_tokens_used_by_list(id)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this OAuth2 Refresh Token. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

