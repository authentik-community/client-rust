# \TasksApi

All URIs are relative to */api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tasks_schedules_list**](TasksApi.md#tasks_schedules_list) | **GET** /tasks/schedules/ | 
[**tasks_schedules_partial_update**](TasksApi.md#tasks_schedules_partial_update) | **PATCH** /tasks/schedules/{id}/ | 
[**tasks_schedules_retrieve**](TasksApi.md#tasks_schedules_retrieve) | **GET** /tasks/schedules/{id}/ | 
[**tasks_schedules_send_create**](TasksApi.md#tasks_schedules_send_create) | **POST** /tasks/schedules/{id}/send/ | 
[**tasks_schedules_update**](TasksApi.md#tasks_schedules_update) | **PUT** /tasks/schedules/{id}/ | 
[**tasks_tasks_list**](TasksApi.md#tasks_tasks_list) | **GET** /tasks/tasks/ | 
[**tasks_tasks_retrieve**](TasksApi.md#tasks_tasks_retrieve) | **GET** /tasks/tasks/{message_id}/ | 
[**tasks_tasks_retry_create**](TasksApi.md#tasks_tasks_retry_create) | **POST** /tasks/tasks/{message_id}/retry/ | 
[**tasks_tasks_status_retrieve**](TasksApi.md#tasks_tasks_status_retrieve) | **GET** /tasks/tasks/status/ | 
[**tasks_workers_list**](TasksApi.md#tasks_workers_list) | **GET** /tasks/workers | 



## tasks_schedules_list

> models::PaginatedScheduleList tasks_schedules_list(actor_name, ordering, page, page_size, paused, rel_obj_content_type__app_label, rel_obj_content_type__model, rel_obj_id, rel_obj_id__isnull, search)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**paused** | Option<**bool**> |  |  |
**rel_obj_content_type__app_label** | Option<**String**> |  |  |
**rel_obj_content_type__model** | Option<**String**> |  |  |
**rel_obj_id** | Option<**String**> |  |  |
**rel_obj_id__isnull** | Option<**bool**> |  |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedScheduleList**](PaginatedScheduleList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tasks_schedules_partial_update

> models::Schedule tasks_schedules_partial_update(id, patched_schedule_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this Schedule. | [required] |
**patched_schedule_request** | Option<[**PatchedScheduleRequest**](PatchedScheduleRequest.md)> |  |  |

### Return type

[**models::Schedule**](Schedule.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tasks_schedules_retrieve

> models::Schedule tasks_schedules_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this Schedule. | [required] |

### Return type

[**models::Schedule**](Schedule.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tasks_schedules_send_create

> tasks_schedules_send_create(id)


Trigger this schedule now

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this Schedule. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tasks_schedules_update

> models::Schedule tasks_schedules_update(id, schedule_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this Schedule. | [required] |
**schedule_request** | [**ScheduleRequest**](ScheduleRequest.md) |  | [required] |

### Return type

[**models::Schedule**](Schedule.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tasks_tasks_list

> models::PaginatedTaskList tasks_tasks_list(actor_name, aggregated_status, ordering, page, page_size, queue_name, rel_obj_content_type__app_label, rel_obj_content_type__model, rel_obj_id, rel_obj_id__isnull, search, state)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_name** | Option<**String**> |  |  |
**aggregated_status** | Option<[**Vec<String>**](String.md)> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**queue_name** | Option<**String**> |  |  |
**rel_obj_content_type__app_label** | Option<**String**> |  |  |
**rel_obj_content_type__model** | Option<**String**> |  |  |
**rel_obj_id** | Option<**String**> |  |  |
**rel_obj_id__isnull** | Option<**bool**> |  |  |
**search** | Option<**String**> | A search term. |  |
**state** | Option<**String**> | Task status   |  |

### Return type

[**models::PaginatedTaskList**](PaginatedTaskList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tasks_tasks_retrieve

> models::Task tasks_tasks_retrieve(message_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **uuid::Uuid** | A UUID string identifying this Task. | [required] |

### Return type

[**models::Task**](Task.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tasks_tasks_retry_create

> tasks_tasks_retry_create(message_id)


Retry task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **uuid::Uuid** | A UUID string identifying this Task. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tasks_tasks_status_retrieve

> models::GlobalTaskStatus tasks_tasks_status_retrieve()


Global status summary for all tasks

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GlobalTaskStatus**](GlobalTaskStatus.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tasks_workers_list

> Vec<models::Worker> tasks_workers_list()


Get currently connected worker count.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Worker>**](Worker.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

