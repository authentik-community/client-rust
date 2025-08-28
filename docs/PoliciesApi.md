# \PoliciesApi

All URIs are relative to *http://localhost/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**policies_all_cache_clear_create**](PoliciesApi.md#policies_all_cache_clear_create) | **POST** /policies/all/cache_clear/ | 
[**policies_all_cache_info_retrieve**](PoliciesApi.md#policies_all_cache_info_retrieve) | **GET** /policies/all/cache_info/ | 
[**policies_all_destroy**](PoliciesApi.md#policies_all_destroy) | **DELETE** /policies/all/{policy_uuid}/ | 
[**policies_all_list**](PoliciesApi.md#policies_all_list) | **GET** /policies/all/ | 
[**policies_all_retrieve**](PoliciesApi.md#policies_all_retrieve) | **GET** /policies/all/{policy_uuid}/ | 
[**policies_all_test_create**](PoliciesApi.md#policies_all_test_create) | **POST** /policies/all/{policy_uuid}/test/ | 
[**policies_all_types_list**](PoliciesApi.md#policies_all_types_list) | **GET** /policies/all/types/ | 
[**policies_all_used_by_list**](PoliciesApi.md#policies_all_used_by_list) | **GET** /policies/all/{policy_uuid}/used_by/ | 
[**policies_bindings_create**](PoliciesApi.md#policies_bindings_create) | **POST** /policies/bindings/ | 
[**policies_bindings_destroy**](PoliciesApi.md#policies_bindings_destroy) | **DELETE** /policies/bindings/{policy_binding_uuid}/ | 
[**policies_bindings_list**](PoliciesApi.md#policies_bindings_list) | **GET** /policies/bindings/ | 
[**policies_bindings_partial_update**](PoliciesApi.md#policies_bindings_partial_update) | **PATCH** /policies/bindings/{policy_binding_uuid}/ | 
[**policies_bindings_retrieve**](PoliciesApi.md#policies_bindings_retrieve) | **GET** /policies/bindings/{policy_binding_uuid}/ | 
[**policies_bindings_update**](PoliciesApi.md#policies_bindings_update) | **PUT** /policies/bindings/{policy_binding_uuid}/ | 
[**policies_bindings_used_by_list**](PoliciesApi.md#policies_bindings_used_by_list) | **GET** /policies/bindings/{policy_binding_uuid}/used_by/ | 
[**policies_dummy_create**](PoliciesApi.md#policies_dummy_create) | **POST** /policies/dummy/ | 
[**policies_dummy_destroy**](PoliciesApi.md#policies_dummy_destroy) | **DELETE** /policies/dummy/{policy_uuid}/ | 
[**policies_dummy_list**](PoliciesApi.md#policies_dummy_list) | **GET** /policies/dummy/ | 
[**policies_dummy_partial_update**](PoliciesApi.md#policies_dummy_partial_update) | **PATCH** /policies/dummy/{policy_uuid}/ | 
[**policies_dummy_retrieve**](PoliciesApi.md#policies_dummy_retrieve) | **GET** /policies/dummy/{policy_uuid}/ | 
[**policies_dummy_update**](PoliciesApi.md#policies_dummy_update) | **PUT** /policies/dummy/{policy_uuid}/ | 
[**policies_dummy_used_by_list**](PoliciesApi.md#policies_dummy_used_by_list) | **GET** /policies/dummy/{policy_uuid}/used_by/ | 
[**policies_event_matcher_create**](PoliciesApi.md#policies_event_matcher_create) | **POST** /policies/event_matcher/ | 
[**policies_event_matcher_destroy**](PoliciesApi.md#policies_event_matcher_destroy) | **DELETE** /policies/event_matcher/{policy_uuid}/ | 
[**policies_event_matcher_list**](PoliciesApi.md#policies_event_matcher_list) | **GET** /policies/event_matcher/ | 
[**policies_event_matcher_partial_update**](PoliciesApi.md#policies_event_matcher_partial_update) | **PATCH** /policies/event_matcher/{policy_uuid}/ | 
[**policies_event_matcher_retrieve**](PoliciesApi.md#policies_event_matcher_retrieve) | **GET** /policies/event_matcher/{policy_uuid}/ | 
[**policies_event_matcher_update**](PoliciesApi.md#policies_event_matcher_update) | **PUT** /policies/event_matcher/{policy_uuid}/ | 
[**policies_event_matcher_used_by_list**](PoliciesApi.md#policies_event_matcher_used_by_list) | **GET** /policies/event_matcher/{policy_uuid}/used_by/ | 
[**policies_expression_create**](PoliciesApi.md#policies_expression_create) | **POST** /policies/expression/ | 
[**policies_expression_destroy**](PoliciesApi.md#policies_expression_destroy) | **DELETE** /policies/expression/{policy_uuid}/ | 
[**policies_expression_list**](PoliciesApi.md#policies_expression_list) | **GET** /policies/expression/ | 
[**policies_expression_partial_update**](PoliciesApi.md#policies_expression_partial_update) | **PATCH** /policies/expression/{policy_uuid}/ | 
[**policies_expression_retrieve**](PoliciesApi.md#policies_expression_retrieve) | **GET** /policies/expression/{policy_uuid}/ | 
[**policies_expression_update**](PoliciesApi.md#policies_expression_update) | **PUT** /policies/expression/{policy_uuid}/ | 
[**policies_expression_used_by_list**](PoliciesApi.md#policies_expression_used_by_list) | **GET** /policies/expression/{policy_uuid}/used_by/ | 
[**policies_geoip_create**](PoliciesApi.md#policies_geoip_create) | **POST** /policies/geoip/ | 
[**policies_geoip_destroy**](PoliciesApi.md#policies_geoip_destroy) | **DELETE** /policies/geoip/{policy_uuid}/ | 
[**policies_geoip_iso3166_list**](PoliciesApi.md#policies_geoip_iso3166_list) | **GET** /policies/geoip_iso3166/ | 
[**policies_geoip_list**](PoliciesApi.md#policies_geoip_list) | **GET** /policies/geoip/ | 
[**policies_geoip_partial_update**](PoliciesApi.md#policies_geoip_partial_update) | **PATCH** /policies/geoip/{policy_uuid}/ | 
[**policies_geoip_retrieve**](PoliciesApi.md#policies_geoip_retrieve) | **GET** /policies/geoip/{policy_uuid}/ | 
[**policies_geoip_update**](PoliciesApi.md#policies_geoip_update) | **PUT** /policies/geoip/{policy_uuid}/ | 
[**policies_geoip_used_by_list**](PoliciesApi.md#policies_geoip_used_by_list) | **GET** /policies/geoip/{policy_uuid}/used_by/ | 
[**policies_password_create**](PoliciesApi.md#policies_password_create) | **POST** /policies/password/ | 
[**policies_password_destroy**](PoliciesApi.md#policies_password_destroy) | **DELETE** /policies/password/{policy_uuid}/ | 
[**policies_password_expiry_create**](PoliciesApi.md#policies_password_expiry_create) | **POST** /policies/password_expiry/ | 
[**policies_password_expiry_destroy**](PoliciesApi.md#policies_password_expiry_destroy) | **DELETE** /policies/password_expiry/{policy_uuid}/ | 
[**policies_password_expiry_list**](PoliciesApi.md#policies_password_expiry_list) | **GET** /policies/password_expiry/ | 
[**policies_password_expiry_partial_update**](PoliciesApi.md#policies_password_expiry_partial_update) | **PATCH** /policies/password_expiry/{policy_uuid}/ | 
[**policies_password_expiry_retrieve**](PoliciesApi.md#policies_password_expiry_retrieve) | **GET** /policies/password_expiry/{policy_uuid}/ | 
[**policies_password_expiry_update**](PoliciesApi.md#policies_password_expiry_update) | **PUT** /policies/password_expiry/{policy_uuid}/ | 
[**policies_password_expiry_used_by_list**](PoliciesApi.md#policies_password_expiry_used_by_list) | **GET** /policies/password_expiry/{policy_uuid}/used_by/ | 
[**policies_password_list**](PoliciesApi.md#policies_password_list) | **GET** /policies/password/ | 
[**policies_password_partial_update**](PoliciesApi.md#policies_password_partial_update) | **PATCH** /policies/password/{policy_uuid}/ | 
[**policies_password_retrieve**](PoliciesApi.md#policies_password_retrieve) | **GET** /policies/password/{policy_uuid}/ | 
[**policies_password_update**](PoliciesApi.md#policies_password_update) | **PUT** /policies/password/{policy_uuid}/ | 
[**policies_password_used_by_list**](PoliciesApi.md#policies_password_used_by_list) | **GET** /policies/password/{policy_uuid}/used_by/ | 
[**policies_reputation_create**](PoliciesApi.md#policies_reputation_create) | **POST** /policies/reputation/ | 
[**policies_reputation_destroy**](PoliciesApi.md#policies_reputation_destroy) | **DELETE** /policies/reputation/{policy_uuid}/ | 
[**policies_reputation_list**](PoliciesApi.md#policies_reputation_list) | **GET** /policies/reputation/ | 
[**policies_reputation_partial_update**](PoliciesApi.md#policies_reputation_partial_update) | **PATCH** /policies/reputation/{policy_uuid}/ | 
[**policies_reputation_retrieve**](PoliciesApi.md#policies_reputation_retrieve) | **GET** /policies/reputation/{policy_uuid}/ | 
[**policies_reputation_scores_destroy**](PoliciesApi.md#policies_reputation_scores_destroy) | **DELETE** /policies/reputation/scores/{reputation_uuid}/ | 
[**policies_reputation_scores_list**](PoliciesApi.md#policies_reputation_scores_list) | **GET** /policies/reputation/scores/ | 
[**policies_reputation_scores_retrieve**](PoliciesApi.md#policies_reputation_scores_retrieve) | **GET** /policies/reputation/scores/{reputation_uuid}/ | 
[**policies_reputation_scores_used_by_list**](PoliciesApi.md#policies_reputation_scores_used_by_list) | **GET** /policies/reputation/scores/{reputation_uuid}/used_by/ | 
[**policies_reputation_update**](PoliciesApi.md#policies_reputation_update) | **PUT** /policies/reputation/{policy_uuid}/ | 
[**policies_reputation_used_by_list**](PoliciesApi.md#policies_reputation_used_by_list) | **GET** /policies/reputation/{policy_uuid}/used_by/ | 
[**policies_unique_password_create**](PoliciesApi.md#policies_unique_password_create) | **POST** /policies/unique_password/ | 
[**policies_unique_password_destroy**](PoliciesApi.md#policies_unique_password_destroy) | **DELETE** /policies/unique_password/{policy_uuid}/ | 
[**policies_unique_password_list**](PoliciesApi.md#policies_unique_password_list) | **GET** /policies/unique_password/ | 
[**policies_unique_password_partial_update**](PoliciesApi.md#policies_unique_password_partial_update) | **PATCH** /policies/unique_password/{policy_uuid}/ | 
[**policies_unique_password_retrieve**](PoliciesApi.md#policies_unique_password_retrieve) | **GET** /policies/unique_password/{policy_uuid}/ | 
[**policies_unique_password_update**](PoliciesApi.md#policies_unique_password_update) | **PUT** /policies/unique_password/{policy_uuid}/ | 
[**policies_unique_password_used_by_list**](PoliciesApi.md#policies_unique_password_used_by_list) | **GET** /policies/unique_password/{policy_uuid}/used_by/ | 



## policies_all_cache_clear_create

> policies_all_cache_clear_create()


Clear policy cache

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_all_cache_info_retrieve

> models::Cache policies_all_cache_info_retrieve()


Info about cached policies

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Cache**](Cache.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_all_destroy

> policies_all_destroy(policy_uuid)


Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Policy. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_all_list

> models::PaginatedPolicyList policies_all_list(bindings__isnull, ordering, page, page_size, promptstage__isnull, search)


Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bindings__isnull** | Option<**bool**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**promptstage__isnull** | Option<**bool**> |  |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedPolicyList**](PaginatedPolicyList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_all_retrieve

> models::Policy policies_all_retrieve(policy_uuid)


Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Policy. | [required] |

### Return type

[**models::Policy**](Policy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_all_test_create

> models::PolicyTestResult policies_all_test_create(policy_uuid, policy_test_request)


Test policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Policy. | [required] |
**policy_test_request** | [**PolicyTestRequest**](PolicyTestRequest.md) |  | [required] |

### Return type

[**models::PolicyTestResult**](PolicyTestResult.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_all_types_list

> Vec<models::TypeCreate> policies_all_types_list()


Get all creatable types

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::TypeCreate>**](TypeCreate.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_all_used_by_list

> Vec<models::UsedBy> policies_all_used_by_list(policy_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Policy. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_bindings_create

> models::PolicyBinding policies_bindings_create(policy_binding_request)


PolicyBinding Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_binding_request** | [**PolicyBindingRequest**](PolicyBindingRequest.md) |  | [required] |

### Return type

[**models::PolicyBinding**](PolicyBinding.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_bindings_destroy

> policies_bindings_destroy(policy_binding_uuid)


PolicyBinding Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_binding_uuid** | **uuid::Uuid** | A UUID string identifying this Policy Binding. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_bindings_list

> models::PaginatedPolicyBindingList policies_bindings_list(enabled, order, ordering, page, page_size, policy, policy__isnull, search, target, target_in, timeout)


PolicyBinding Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enabled** | Option<**bool**> |  |  |
**order** | Option<**i32**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**policy** | Option<**uuid::Uuid**> |  |  |
**policy__isnull** | Option<**bool**> |  |  |
**search** | Option<**String**> | A search term. |  |
**target** | Option<**uuid::Uuid**> |  |  |
**target_in** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  |  |
**timeout** | Option<**i32**> |  |  |

### Return type

[**models::PaginatedPolicyBindingList**](PaginatedPolicyBindingList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_bindings_partial_update

> models::PolicyBinding policies_bindings_partial_update(policy_binding_uuid, patched_policy_binding_request)


PolicyBinding Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_binding_uuid** | **uuid::Uuid** | A UUID string identifying this Policy Binding. | [required] |
**patched_policy_binding_request** | Option<[**PatchedPolicyBindingRequest**](PatchedPolicyBindingRequest.md)> |  |  |

### Return type

[**models::PolicyBinding**](PolicyBinding.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_bindings_retrieve

> models::PolicyBinding policies_bindings_retrieve(policy_binding_uuid)


PolicyBinding Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_binding_uuid** | **uuid::Uuid** | A UUID string identifying this Policy Binding. | [required] |

### Return type

[**models::PolicyBinding**](PolicyBinding.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_bindings_update

> models::PolicyBinding policies_bindings_update(policy_binding_uuid, policy_binding_request)


PolicyBinding Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_binding_uuid** | **uuid::Uuid** | A UUID string identifying this Policy Binding. | [required] |
**policy_binding_request** | [**PolicyBindingRequest**](PolicyBindingRequest.md) |  | [required] |

### Return type

[**models::PolicyBinding**](PolicyBinding.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_bindings_used_by_list

> Vec<models::UsedBy> policies_bindings_used_by_list(policy_binding_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_binding_uuid** | **uuid::Uuid** | A UUID string identifying this Policy Binding. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_dummy_create

> models::DummyPolicy policies_dummy_create(dummy_policy_request)


Dummy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dummy_policy_request** | [**DummyPolicyRequest**](DummyPolicyRequest.md) |  | [required] |

### Return type

[**models::DummyPolicy**](DummyPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_dummy_destroy

> policies_dummy_destroy(policy_uuid)


Dummy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Dummy Policy. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_dummy_list

> models::PaginatedDummyPolicyList policies_dummy_list(created, execution_logging, last_updated, name, ordering, page, page_size, policy_uuid, result, search, wait_max, wait_min)


Dummy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**created** | Option<**String**> |  |  |
**execution_logging** | Option<**bool**> |  |  |
**last_updated** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**policy_uuid** | Option<**uuid::Uuid**> |  |  |
**result** | Option<**bool**> |  |  |
**search** | Option<**String**> | A search term. |  |
**wait_max** | Option<**i32**> |  |  |
**wait_min** | Option<**i32**> |  |  |

### Return type

[**models::PaginatedDummyPolicyList**](PaginatedDummyPolicyList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_dummy_partial_update

> models::DummyPolicy policies_dummy_partial_update(policy_uuid, patched_dummy_policy_request)


Dummy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Dummy Policy. | [required] |
**patched_dummy_policy_request** | Option<[**PatchedDummyPolicyRequest**](PatchedDummyPolicyRequest.md)> |  |  |

### Return type

[**models::DummyPolicy**](DummyPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_dummy_retrieve

> models::DummyPolicy policies_dummy_retrieve(policy_uuid)


Dummy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Dummy Policy. | [required] |

### Return type

[**models::DummyPolicy**](DummyPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_dummy_update

> models::DummyPolicy policies_dummy_update(policy_uuid, dummy_policy_request)


Dummy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Dummy Policy. | [required] |
**dummy_policy_request** | [**DummyPolicyRequest**](DummyPolicyRequest.md) |  | [required] |

### Return type

[**models::DummyPolicy**](DummyPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_dummy_used_by_list

> Vec<models::UsedBy> policies_dummy_used_by_list(policy_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Dummy Policy. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_event_matcher_create

> models::EventMatcherPolicy policies_event_matcher_create(event_matcher_policy_request)


Event Matcher Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_matcher_policy_request** | [**EventMatcherPolicyRequest**](EventMatcherPolicyRequest.md) |  | [required] |

### Return type

[**models::EventMatcherPolicy**](EventMatcherPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_event_matcher_destroy

> policies_event_matcher_destroy(policy_uuid)


Event Matcher Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Event Matcher Policy. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_event_matcher_list

> models::PaginatedEventMatcherPolicyList policies_event_matcher_list(action, app, client_ip, created, execution_logging, last_updated, model, name, ordering, page, page_size, policy_uuid, search)


Event Matcher Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action** | Option<**String**> | Match created events with this action type. When left empty, all action types will be matched.   |  |
**app** | Option<**String**> |  |  |
**client_ip** | Option<**String**> |  |  |
**created** | Option<**String**> |  |  |
**execution_logging** | Option<**bool**> |  |  |
**last_updated** | Option<**String**> |  |  |
**model** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**policy_uuid** | Option<**uuid::Uuid**> |  |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedEventMatcherPolicyList**](PaginatedEventMatcherPolicyList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_event_matcher_partial_update

> models::EventMatcherPolicy policies_event_matcher_partial_update(policy_uuid, patched_event_matcher_policy_request)


Event Matcher Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Event Matcher Policy. | [required] |
**patched_event_matcher_policy_request** | Option<[**PatchedEventMatcherPolicyRequest**](PatchedEventMatcherPolicyRequest.md)> |  |  |

### Return type

[**models::EventMatcherPolicy**](EventMatcherPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_event_matcher_retrieve

> models::EventMatcherPolicy policies_event_matcher_retrieve(policy_uuid)


Event Matcher Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Event Matcher Policy. | [required] |

### Return type

[**models::EventMatcherPolicy**](EventMatcherPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_event_matcher_update

> models::EventMatcherPolicy policies_event_matcher_update(policy_uuid, event_matcher_policy_request)


Event Matcher Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Event Matcher Policy. | [required] |
**event_matcher_policy_request** | [**EventMatcherPolicyRequest**](EventMatcherPolicyRequest.md) |  | [required] |

### Return type

[**models::EventMatcherPolicy**](EventMatcherPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_event_matcher_used_by_list

> Vec<models::UsedBy> policies_event_matcher_used_by_list(policy_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Event Matcher Policy. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_expression_create

> models::ExpressionPolicy policies_expression_create(expression_policy_request)


Source Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**expression_policy_request** | [**ExpressionPolicyRequest**](ExpressionPolicyRequest.md) |  | [required] |

### Return type

[**models::ExpressionPolicy**](ExpressionPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_expression_destroy

> policies_expression_destroy(policy_uuid)


Source Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Expression Policy. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_expression_list

> models::PaginatedExpressionPolicyList policies_expression_list(created, execution_logging, expression, last_updated, name, ordering, page, page_size, policy_uuid, search)


Source Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**created** | Option<**String**> |  |  |
**execution_logging** | Option<**bool**> |  |  |
**expression** | Option<**String**> |  |  |
**last_updated** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**policy_uuid** | Option<**uuid::Uuid**> |  |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedExpressionPolicyList**](PaginatedExpressionPolicyList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_expression_partial_update

> models::ExpressionPolicy policies_expression_partial_update(policy_uuid, patched_expression_policy_request)


Source Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Expression Policy. | [required] |
**patched_expression_policy_request** | Option<[**PatchedExpressionPolicyRequest**](PatchedExpressionPolicyRequest.md)> |  |  |

### Return type

[**models::ExpressionPolicy**](ExpressionPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_expression_retrieve

> models::ExpressionPolicy policies_expression_retrieve(policy_uuid)


Source Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Expression Policy. | [required] |

### Return type

[**models::ExpressionPolicy**](ExpressionPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_expression_update

> models::ExpressionPolicy policies_expression_update(policy_uuid, expression_policy_request)


Source Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Expression Policy. | [required] |
**expression_policy_request** | [**ExpressionPolicyRequest**](ExpressionPolicyRequest.md) |  | [required] |

### Return type

[**models::ExpressionPolicy**](ExpressionPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_expression_used_by_list

> Vec<models::UsedBy> policies_expression_used_by_list(policy_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Expression Policy. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_geoip_create

> models::GeoIpPolicy policies_geoip_create(geo_ip_policy_request)


GeoIP Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**geo_ip_policy_request** | [**GeoIpPolicyRequest**](GeoIpPolicyRequest.md) |  | [required] |

### Return type

[**models::GeoIpPolicy**](GeoIPPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_geoip_destroy

> policies_geoip_destroy(policy_uuid)


GeoIP Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this GeoIP Policy. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_geoip_iso3166_list

> Vec<models::DetailedCountry> policies_geoip_iso3166_list()


Get all countries in ISO-3166-1

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::DetailedCountry>**](DetailedCountry.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_geoip_list

> models::PaginatedGeoIpPolicyList policies_geoip_list(name, ordering, page, page_size, search)


GeoIP Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedGeoIpPolicyList**](PaginatedGeoIPPolicyList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_geoip_partial_update

> models::GeoIpPolicy policies_geoip_partial_update(policy_uuid, patched_geo_ip_policy_request)


GeoIP Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this GeoIP Policy. | [required] |
**patched_geo_ip_policy_request** | Option<[**PatchedGeoIpPolicyRequest**](PatchedGeoIpPolicyRequest.md)> |  |  |

### Return type

[**models::GeoIpPolicy**](GeoIPPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_geoip_retrieve

> models::GeoIpPolicy policies_geoip_retrieve(policy_uuid)


GeoIP Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this GeoIP Policy. | [required] |

### Return type

[**models::GeoIpPolicy**](GeoIPPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_geoip_update

> models::GeoIpPolicy policies_geoip_update(policy_uuid, geo_ip_policy_request)


GeoIP Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this GeoIP Policy. | [required] |
**geo_ip_policy_request** | [**GeoIpPolicyRequest**](GeoIpPolicyRequest.md) |  | [required] |

### Return type

[**models::GeoIpPolicy**](GeoIPPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_geoip_used_by_list

> Vec<models::UsedBy> policies_geoip_used_by_list(policy_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this GeoIP Policy. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_password_create

> models::PasswordPolicy policies_password_create(password_policy_request)


Password Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**password_policy_request** | [**PasswordPolicyRequest**](PasswordPolicyRequest.md) |  | [required] |

### Return type

[**models::PasswordPolicy**](PasswordPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_password_destroy

> policies_password_destroy(policy_uuid)


Password Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Password Policy. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_password_expiry_create

> models::PasswordExpiryPolicy policies_password_expiry_create(password_expiry_policy_request)


Password Expiry Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**password_expiry_policy_request** | [**PasswordExpiryPolicyRequest**](PasswordExpiryPolicyRequest.md) |  | [required] |

### Return type

[**models::PasswordExpiryPolicy**](PasswordExpiryPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_password_expiry_destroy

> policies_password_expiry_destroy(policy_uuid)


Password Expiry Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Password Expiry Policy. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_password_expiry_list

> models::PaginatedPasswordExpiryPolicyList policies_password_expiry_list(created, days, deny_only, execution_logging, last_updated, name, ordering, page, page_size, policy_uuid, search)


Password Expiry Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**created** | Option<**String**> |  |  |
**days** | Option<**i32**> |  |  |
**deny_only** | Option<**bool**> |  |  |
**execution_logging** | Option<**bool**> |  |  |
**last_updated** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**policy_uuid** | Option<**uuid::Uuid**> |  |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedPasswordExpiryPolicyList**](PaginatedPasswordExpiryPolicyList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_password_expiry_partial_update

> models::PasswordExpiryPolicy policies_password_expiry_partial_update(policy_uuid, patched_password_expiry_policy_request)


Password Expiry Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Password Expiry Policy. | [required] |
**patched_password_expiry_policy_request** | Option<[**PatchedPasswordExpiryPolicyRequest**](PatchedPasswordExpiryPolicyRequest.md)> |  |  |

### Return type

[**models::PasswordExpiryPolicy**](PasswordExpiryPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_password_expiry_retrieve

> models::PasswordExpiryPolicy policies_password_expiry_retrieve(policy_uuid)


Password Expiry Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Password Expiry Policy. | [required] |

### Return type

[**models::PasswordExpiryPolicy**](PasswordExpiryPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_password_expiry_update

> models::PasswordExpiryPolicy policies_password_expiry_update(policy_uuid, password_expiry_policy_request)


Password Expiry Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Password Expiry Policy. | [required] |
**password_expiry_policy_request** | [**PasswordExpiryPolicyRequest**](PasswordExpiryPolicyRequest.md) |  | [required] |

### Return type

[**models::PasswordExpiryPolicy**](PasswordExpiryPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_password_expiry_used_by_list

> Vec<models::UsedBy> policies_password_expiry_used_by_list(policy_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Password Expiry Policy. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_password_list

> models::PaginatedPasswordPolicyList policies_password_list(amount_digits, amount_lowercase, amount_symbols, amount_uppercase, check_have_i_been_pwned, check_static_rules, check_zxcvbn, created, error_message, execution_logging, hibp_allowed_count, last_updated, length_min, name, ordering, page, page_size, password_field, policy_uuid, search, symbol_charset, zxcvbn_score_threshold)


Password Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**amount_digits** | Option<**i32**> |  |  |
**amount_lowercase** | Option<**i32**> |  |  |
**amount_symbols** | Option<**i32**> |  |  |
**amount_uppercase** | Option<**i32**> |  |  |
**check_have_i_been_pwned** | Option<**bool**> |  |  |
**check_static_rules** | Option<**bool**> |  |  |
**check_zxcvbn** | Option<**bool**> |  |  |
**created** | Option<**String**> |  |  |
**error_message** | Option<**String**> |  |  |
**execution_logging** | Option<**bool**> |  |  |
**hibp_allowed_count** | Option<**i32**> |  |  |
**last_updated** | Option<**String**> |  |  |
**length_min** | Option<**i32**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**password_field** | Option<**String**> |  |  |
**policy_uuid** | Option<**uuid::Uuid**> |  |  |
**search** | Option<**String**> | A search term. |  |
**symbol_charset** | Option<**String**> |  |  |
**zxcvbn_score_threshold** | Option<**i32**> |  |  |

### Return type

[**models::PaginatedPasswordPolicyList**](PaginatedPasswordPolicyList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_password_partial_update

> models::PasswordPolicy policies_password_partial_update(policy_uuid, patched_password_policy_request)


Password Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Password Policy. | [required] |
**patched_password_policy_request** | Option<[**PatchedPasswordPolicyRequest**](PatchedPasswordPolicyRequest.md)> |  |  |

### Return type

[**models::PasswordPolicy**](PasswordPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_password_retrieve

> models::PasswordPolicy policies_password_retrieve(policy_uuid)


Password Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Password Policy. | [required] |

### Return type

[**models::PasswordPolicy**](PasswordPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_password_update

> models::PasswordPolicy policies_password_update(policy_uuid, password_policy_request)


Password Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Password Policy. | [required] |
**password_policy_request** | [**PasswordPolicyRequest**](PasswordPolicyRequest.md) |  | [required] |

### Return type

[**models::PasswordPolicy**](PasswordPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_password_used_by_list

> Vec<models::UsedBy> policies_password_used_by_list(policy_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Password Policy. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_reputation_create

> models::ReputationPolicy policies_reputation_create(reputation_policy_request)


Reputation Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reputation_policy_request** | [**ReputationPolicyRequest**](ReputationPolicyRequest.md) |  | [required] |

### Return type

[**models::ReputationPolicy**](ReputationPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_reputation_destroy

> policies_reputation_destroy(policy_uuid)


Reputation Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Reputation Policy. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_reputation_list

> models::PaginatedReputationPolicyList policies_reputation_list(check_ip, check_username, created, execution_logging, last_updated, name, ordering, page, page_size, policy_uuid, search, threshold)


Reputation Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_ip** | Option<**bool**> |  |  |
**check_username** | Option<**bool**> |  |  |
**created** | Option<**String**> |  |  |
**execution_logging** | Option<**bool**> |  |  |
**last_updated** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**policy_uuid** | Option<**uuid::Uuid**> |  |  |
**search** | Option<**String**> | A search term. |  |
**threshold** | Option<**i32**> |  |  |

### Return type

[**models::PaginatedReputationPolicyList**](PaginatedReputationPolicyList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_reputation_partial_update

> models::ReputationPolicy policies_reputation_partial_update(policy_uuid, patched_reputation_policy_request)


Reputation Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Reputation Policy. | [required] |
**patched_reputation_policy_request** | Option<[**PatchedReputationPolicyRequest**](PatchedReputationPolicyRequest.md)> |  |  |

### Return type

[**models::ReputationPolicy**](ReputationPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_reputation_retrieve

> models::ReputationPolicy policies_reputation_retrieve(policy_uuid)


Reputation Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Reputation Policy. | [required] |

### Return type

[**models::ReputationPolicy**](ReputationPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_reputation_scores_destroy

> policies_reputation_scores_destroy(reputation_uuid)


Reputation Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reputation_uuid** | **uuid::Uuid** | A UUID string identifying this Reputation Score. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_reputation_scores_list

> models::PaginatedReputationList policies_reputation_scores_list(identifier, identifier_in, ip, ordering, page, page_size, score, search)


Reputation Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | Option<**String**> |  |  |
**identifier_in** | Option<[**Vec<String>**](String.md)> | Multiple values may be separated by commas. |  |
**ip** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**score** | Option<**i32**> |  |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedReputationList**](PaginatedReputationList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_reputation_scores_retrieve

> models::Reputation policies_reputation_scores_retrieve(reputation_uuid)


Reputation Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reputation_uuid** | **uuid::Uuid** | A UUID string identifying this Reputation Score. | [required] |

### Return type

[**models::Reputation**](Reputation.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_reputation_scores_used_by_list

> Vec<models::UsedBy> policies_reputation_scores_used_by_list(reputation_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reputation_uuid** | **uuid::Uuid** | A UUID string identifying this Reputation Score. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_reputation_update

> models::ReputationPolicy policies_reputation_update(policy_uuid, reputation_policy_request)


Reputation Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Reputation Policy. | [required] |
**reputation_policy_request** | [**ReputationPolicyRequest**](ReputationPolicyRequest.md) |  | [required] |

### Return type

[**models::ReputationPolicy**](ReputationPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_reputation_used_by_list

> Vec<models::UsedBy> policies_reputation_used_by_list(policy_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Reputation Policy. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_unique_password_create

> models::UniquePasswordPolicy policies_unique_password_create(unique_password_policy_request)


Password Uniqueness Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unique_password_policy_request** | [**UniquePasswordPolicyRequest**](UniquePasswordPolicyRequest.md) |  | [required] |

### Return type

[**models::UniquePasswordPolicy**](UniquePasswordPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_unique_password_destroy

> policies_unique_password_destroy(policy_uuid)


Password Uniqueness Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Password Uniqueness Policy. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_unique_password_list

> models::PaginatedUniquePasswordPolicyList policies_unique_password_list(created, execution_logging, last_updated, name, num_historical_passwords, ordering, page, page_size, password_field, policy_uuid, search)


Password Uniqueness Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**created** | Option<**String**> |  |  |
**execution_logging** | Option<**bool**> |  |  |
**last_updated** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**num_historical_passwords** | Option<**i32**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**password_field** | Option<**String**> |  |  |
**policy_uuid** | Option<**uuid::Uuid**> |  |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedUniquePasswordPolicyList**](PaginatedUniquePasswordPolicyList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_unique_password_partial_update

> models::UniquePasswordPolicy policies_unique_password_partial_update(policy_uuid, patched_unique_password_policy_request)


Password Uniqueness Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Password Uniqueness Policy. | [required] |
**patched_unique_password_policy_request** | Option<[**PatchedUniquePasswordPolicyRequest**](PatchedUniquePasswordPolicyRequest.md)> |  |  |

### Return type

[**models::UniquePasswordPolicy**](UniquePasswordPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_unique_password_retrieve

> models::UniquePasswordPolicy policies_unique_password_retrieve(policy_uuid)


Password Uniqueness Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Password Uniqueness Policy. | [required] |

### Return type

[**models::UniquePasswordPolicy**](UniquePasswordPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_unique_password_update

> models::UniquePasswordPolicy policies_unique_password_update(policy_uuid, unique_password_policy_request)


Password Uniqueness Policy Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Password Uniqueness Policy. | [required] |
**unique_password_policy_request** | [**UniquePasswordPolicyRequest**](UniquePasswordPolicyRequest.md) |  | [required] |

### Return type

[**models::UniquePasswordPolicy**](UniquePasswordPolicy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_unique_password_used_by_list

> Vec<models::UsedBy> policies_unique_password_used_by_list(policy_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | A UUID string identifying this Password Uniqueness Policy. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

