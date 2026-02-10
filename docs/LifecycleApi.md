# \LifecycleApi

All URIs are relative to */api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**lifecycle_iterations_create**](LifecycleApi.md#lifecycle_iterations_create) | **POST** /lifecycle/iterations/ | 
[**lifecycle_iterations_latest_retrieve**](LifecycleApi.md#lifecycle_iterations_latest_retrieve) | **GET** /lifecycle/iterations/latest/{content_type}/{object_id}/ | 
[**lifecycle_iterations_list_open**](LifecycleApi.md#lifecycle_iterations_list_open) | **GET** /lifecycle/iterations/open/ | 
[**lifecycle_reviews_create**](LifecycleApi.md#lifecycle_reviews_create) | **POST** /lifecycle/reviews/ | 
[**lifecycle_rules_create**](LifecycleApi.md#lifecycle_rules_create) | **POST** /lifecycle/rules/ | 
[**lifecycle_rules_destroy**](LifecycleApi.md#lifecycle_rules_destroy) | **DELETE** /lifecycle/rules/{id}/ | 
[**lifecycle_rules_list**](LifecycleApi.md#lifecycle_rules_list) | **GET** /lifecycle/rules/ | 
[**lifecycle_rules_partial_update**](LifecycleApi.md#lifecycle_rules_partial_update) | **PATCH** /lifecycle/rules/{id}/ | 
[**lifecycle_rules_retrieve**](LifecycleApi.md#lifecycle_rules_retrieve) | **GET** /lifecycle/rules/{id}/ | 
[**lifecycle_rules_update**](LifecycleApi.md#lifecycle_rules_update) | **PUT** /lifecycle/rules/{id}/ | 



## lifecycle_iterations_create

> models::LifecycleIteration lifecycle_iterations_create(lifecycle_iteration_request)


Mixin to validate that a valid enterprise license exists before allowing to save the object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lifecycle_iteration_request** | [**LifecycleIterationRequest**](LifecycleIterationRequest.md) |  | [required] |

### Return type

[**models::LifecycleIteration**](LifecycleIteration.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lifecycle_iterations_latest_retrieve

> models::LifecycleIteration lifecycle_iterations_latest_retrieve(content_type, object_id)


Mixin to validate that a valid enterprise license exists before allowing to save the object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_type** | **String** |  | [required] |
**object_id** | **String** |  | [required] |

### Return type

[**models::LifecycleIteration**](LifecycleIteration.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lifecycle_iterations_list_open

> models::PaginatedLifecycleIterationList lifecycle_iterations_list_open(ordering, page, page_size, search, user_is_reviewer)


Mixin to validate that a valid enterprise license exists before allowing to save the object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**user_is_reviewer** | Option<**bool**> |  |  |

### Return type

[**models::PaginatedLifecycleIterationList**](PaginatedLifecycleIterationList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lifecycle_reviews_create

> models::Review lifecycle_reviews_create(review_request)


Mixin to validate that a valid enterprise license exists before allowing to save the object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**review_request** | [**ReviewRequest**](ReviewRequest.md) |  | [required] |

### Return type

[**models::Review**](Review.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lifecycle_rules_create

> models::LifecycleRule lifecycle_rules_create(lifecycle_rule_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lifecycle_rule_request** | [**LifecycleRuleRequest**](LifecycleRuleRequest.md) |  | [required] |

### Return type

[**models::LifecycleRule**](LifecycleRule.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lifecycle_rules_destroy

> lifecycle_rules_destroy(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this lifecycle rule. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lifecycle_rules_list

> models::PaginatedLifecycleRuleList lifecycle_rules_list(content_type__model, ordering, page, page_size, search)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_type__model** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedLifecycleRuleList**](PaginatedLifecycleRuleList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lifecycle_rules_partial_update

> models::LifecycleRule lifecycle_rules_partial_update(id, patched_lifecycle_rule_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this lifecycle rule. | [required] |
**patched_lifecycle_rule_request** | Option<[**PatchedLifecycleRuleRequest**](PatchedLifecycleRuleRequest.md)> |  |  |

### Return type

[**models::LifecycleRule**](LifecycleRule.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lifecycle_rules_retrieve

> models::LifecycleRule lifecycle_rules_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this lifecycle rule. | [required] |

### Return type

[**models::LifecycleRule**](LifecycleRule.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lifecycle_rules_update

> models::LifecycleRule lifecycle_rules_update(id, lifecycle_rule_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this lifecycle rule. | [required] |
**lifecycle_rule_request** | [**LifecycleRuleRequest**](LifecycleRuleRequest.md) |  | [required] |

### Return type

[**models::LifecycleRule**](LifecycleRule.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

