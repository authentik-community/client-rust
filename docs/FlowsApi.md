# \FlowsApi

All URIs are relative to */api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**flows_bindings_create**](FlowsApi.md#flows_bindings_create) | **POST** /flows/bindings/ | 
[**flows_bindings_destroy**](FlowsApi.md#flows_bindings_destroy) | **DELETE** /flows/bindings/{fsb_uuid}/ | 
[**flows_bindings_list**](FlowsApi.md#flows_bindings_list) | **GET** /flows/bindings/ | 
[**flows_bindings_partial_update**](FlowsApi.md#flows_bindings_partial_update) | **PATCH** /flows/bindings/{fsb_uuid}/ | 
[**flows_bindings_retrieve**](FlowsApi.md#flows_bindings_retrieve) | **GET** /flows/bindings/{fsb_uuid}/ | 
[**flows_bindings_update**](FlowsApi.md#flows_bindings_update) | **PUT** /flows/bindings/{fsb_uuid}/ | 
[**flows_bindings_used_by_list**](FlowsApi.md#flows_bindings_used_by_list) | **GET** /flows/bindings/{fsb_uuid}/used_by/ | 
[**flows_executor_get**](FlowsApi.md#flows_executor_get) | **GET** /flows/executor/{flow_slug}/ | 
[**flows_executor_solve**](FlowsApi.md#flows_executor_solve) | **POST** /flows/executor/{flow_slug}/ | 
[**flows_inspector_get**](FlowsApi.md#flows_inspector_get) | **GET** /flows/inspector/{flow_slug}/ | 
[**flows_instances_cache_clear_create**](FlowsApi.md#flows_instances_cache_clear_create) | **POST** /flows/instances/cache_clear/ | 
[**flows_instances_cache_info_retrieve**](FlowsApi.md#flows_instances_cache_info_retrieve) | **GET** /flows/instances/cache_info/ | 
[**flows_instances_create**](FlowsApi.md#flows_instances_create) | **POST** /flows/instances/ | 
[**flows_instances_destroy**](FlowsApi.md#flows_instances_destroy) | **DELETE** /flows/instances/{slug}/ | 
[**flows_instances_diagram_retrieve**](FlowsApi.md#flows_instances_diagram_retrieve) | **GET** /flows/instances/{slug}/diagram/ | 
[**flows_instances_execute_retrieve**](FlowsApi.md#flows_instances_execute_retrieve) | **GET** /flows/instances/{slug}/execute/ | 
[**flows_instances_export_retrieve**](FlowsApi.md#flows_instances_export_retrieve) | **GET** /flows/instances/{slug}/export/ | 
[**flows_instances_import_create**](FlowsApi.md#flows_instances_import_create) | **POST** /flows/instances/import/ | 
[**flows_instances_list**](FlowsApi.md#flows_instances_list) | **GET** /flows/instances/ | 
[**flows_instances_partial_update**](FlowsApi.md#flows_instances_partial_update) | **PATCH** /flows/instances/{slug}/ | 
[**flows_instances_retrieve**](FlowsApi.md#flows_instances_retrieve) | **GET** /flows/instances/{slug}/ | 
[**flows_instances_set_background_create**](FlowsApi.md#flows_instances_set_background_create) | **POST** /flows/instances/{slug}/set_background/ | 
[**flows_instances_set_background_url_create**](FlowsApi.md#flows_instances_set_background_url_create) | **POST** /flows/instances/{slug}/set_background_url/ | 
[**flows_instances_update**](FlowsApi.md#flows_instances_update) | **PUT** /flows/instances/{slug}/ | 
[**flows_instances_used_by_list**](FlowsApi.md#flows_instances_used_by_list) | **GET** /flows/instances/{slug}/used_by/ | 



## flows_bindings_create

> models::FlowStageBinding flows_bindings_create(flow_stage_binding_request)


FlowStageBinding Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_stage_binding_request** | [**FlowStageBindingRequest**](FlowStageBindingRequest.md) |  | [required] |

### Return type

[**models::FlowStageBinding**](FlowStageBinding.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flows_bindings_destroy

> flows_bindings_destroy(fsb_uuid)


FlowStageBinding Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fsb_uuid** | **uuid::Uuid** | A UUID string identifying this Flow Stage Binding. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flows_bindings_list

> models::PaginatedFlowStageBindingList flows_bindings_list(evaluate_on_plan, fsb_uuid, invalid_response_action, order, ordering, page, page_size, pbm_uuid, policies, policy_engine_mode, re_evaluate_policies, search, stage, target)


FlowStageBinding Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**evaluate_on_plan** | Option<**bool**> |  |  |
**fsb_uuid** | Option<**uuid::Uuid**> |  |  |
**invalid_response_action** | Option<**String**> | Configure how the flow executor should handle an invalid response to a challenge. RETRY returns the error message and a similar challenge to the executor. RESTART restarts the flow from the beginning, and RESTART_WITH_CONTEXT restarts the flow while keeping the current context.   |  |
**order** | Option<**i32**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**pbm_uuid** | Option<**uuid::Uuid**> |  |  |
**policies** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  |  |
**policy_engine_mode** | Option<**String**> |  |  |
**re_evaluate_policies** | Option<**bool**> |  |  |
**search** | Option<**String**> | A search term. |  |
**stage** | Option<**uuid::Uuid**> |  |  |
**target** | Option<**uuid::Uuid**> |  |  |

### Return type

[**models::PaginatedFlowStageBindingList**](PaginatedFlowStageBindingList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flows_bindings_partial_update

> models::FlowStageBinding flows_bindings_partial_update(fsb_uuid, patched_flow_stage_binding_request)


FlowStageBinding Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fsb_uuid** | **uuid::Uuid** | A UUID string identifying this Flow Stage Binding. | [required] |
**patched_flow_stage_binding_request** | Option<[**PatchedFlowStageBindingRequest**](PatchedFlowStageBindingRequest.md)> |  |  |

### Return type

[**models::FlowStageBinding**](FlowStageBinding.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flows_bindings_retrieve

> models::FlowStageBinding flows_bindings_retrieve(fsb_uuid)


FlowStageBinding Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fsb_uuid** | **uuid::Uuid** | A UUID string identifying this Flow Stage Binding. | [required] |

### Return type

[**models::FlowStageBinding**](FlowStageBinding.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flows_bindings_update

> models::FlowStageBinding flows_bindings_update(fsb_uuid, flow_stage_binding_request)


FlowStageBinding Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fsb_uuid** | **uuid::Uuid** | A UUID string identifying this Flow Stage Binding. | [required] |
**flow_stage_binding_request** | [**FlowStageBindingRequest**](FlowStageBindingRequest.md) |  | [required] |

### Return type

[**models::FlowStageBinding**](FlowStageBinding.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flows_bindings_used_by_list

> Vec<models::UsedBy> flows_bindings_used_by_list(fsb_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fsb_uuid** | **uuid::Uuid** | A UUID string identifying this Flow Stage Binding. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flows_executor_get

> models::ChallengeTypes flows_executor_get(flow_slug, query)


Get the next pending challenge from the currently active flow.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_slug** | **String** |  | [required] |
**query** | **String** | Querystring as received | [required] |

### Return type

[**models::ChallengeTypes**](ChallengeTypes.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flows_executor_solve

> models::ChallengeTypes flows_executor_solve(flow_slug, query, flow_challenge_response_request)


Solve the previously retrieved challenge and advanced to the next stage.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_slug** | **String** |  | [required] |
**query** | **String** | Querystring as received | [required] |
**flow_challenge_response_request** | Option<[**FlowChallengeResponseRequest**](FlowChallengeResponseRequest.md)> |  |  |

### Return type

[**models::ChallengeTypes**](ChallengeTypes.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flows_inspector_get

> models::FlowInspection flows_inspector_get(flow_slug)


Get current flow state and record it

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_slug** | **String** |  | [required] |

### Return type

[**models::FlowInspection**](FlowInspection.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flows_instances_cache_clear_create

> flows_instances_cache_clear_create()


Clear flow cache

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


## flows_instances_cache_info_retrieve

> models::Cache flows_instances_cache_info_retrieve()


Info about cached flows

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


## flows_instances_create

> models::Flow flows_instances_create(flow_request)


Flow Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_request** | [**FlowRequest**](FlowRequest.md) |  | [required] |

### Return type

[**models::Flow**](Flow.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flows_instances_destroy

> flows_instances_destroy(slug)


Flow Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flows_instances_diagram_retrieve

> models::FlowDiagram flows_instances_diagram_retrieve(slug)


Return diagram for flow with slug `slug`, in the format used by flowchart.js

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

[**models::FlowDiagram**](FlowDiagram.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flows_instances_execute_retrieve

> models::Link flows_instances_execute_retrieve(slug)


Execute flow for current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

[**models::Link**](Link.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flows_instances_export_retrieve

> std::path::PathBuf flows_instances_export_retrieve(slug)


Export flow to .yaml file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flows_instances_import_create

> models::FlowImportResult flows_instances_import_create(file, clear)


Import flow from .yaml file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | Option<**std::path::PathBuf**> |  |  |
**clear** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::FlowImportResult**](FlowImportResult.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flows_instances_list

> models::PaginatedFlowList flows_instances_list(denied_action, designation, flow_uuid, name, ordering, page, page_size, search, slug)


Flow Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**denied_action** | Option<**String**> | Configure what should happen when a flow denies access to a user.   |  |
**designation** | Option<**String**> | Decides what this Flow is used for. For example, the Authentication flow is redirect to when an un-authenticated user visits authentik.   |  |
**flow_uuid** | Option<**uuid::Uuid**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**slug** | Option<**String**> |  |  |

### Return type

[**models::PaginatedFlowList**](PaginatedFlowList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flows_instances_partial_update

> models::Flow flows_instances_partial_update(slug, patched_flow_request)


Flow Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**patched_flow_request** | Option<[**PatchedFlowRequest**](PatchedFlowRequest.md)> |  |  |

### Return type

[**models::Flow**](Flow.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flows_instances_retrieve

> models::Flow flows_instances_retrieve(slug)


Flow Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

[**models::Flow**](Flow.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flows_instances_set_background_create

> flows_instances_set_background_create(slug, file, clear)


Set Flow background

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**file** | Option<**std::path::PathBuf**> |  |  |
**clear** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flows_instances_set_background_url_create

> flows_instances_set_background_url_create(slug, file_path_request)


Set Flow background (as URL)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**file_path_request** | [**FilePathRequest**](FilePathRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flows_instances_update

> models::Flow flows_instances_update(slug, flow_request)


Flow Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**flow_request** | [**FlowRequest**](FlowRequest.md) |  | [required] |

### Return type

[**models::Flow**](Flow.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flows_instances_used_by_list

> Vec<models::UsedBy> flows_instances_used_by_list(slug)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

