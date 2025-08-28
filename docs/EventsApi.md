# \EventsApi

All URIs are relative to *http://localhost/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**events_events_actions_list**](EventsApi.md#events_events_actions_list) | **GET** /events/events/actions/ | 
[**events_events_create**](EventsApi.md#events_events_create) | **POST** /events/events/ | 
[**events_events_destroy**](EventsApi.md#events_events_destroy) | **DELETE** /events/events/{event_uuid}/ | 
[**events_events_list**](EventsApi.md#events_events_list) | **GET** /events/events/ | 
[**events_events_partial_update**](EventsApi.md#events_events_partial_update) | **PATCH** /events/events/{event_uuid}/ | 
[**events_events_per_month_list**](EventsApi.md#events_events_per_month_list) | **GET** /events/events/per_month/ | 
[**events_events_retrieve**](EventsApi.md#events_events_retrieve) | **GET** /events/events/{event_uuid}/ | 
[**events_events_top_per_user_list**](EventsApi.md#events_events_top_per_user_list) | **GET** /events/events/top_per_user/ | 
[**events_events_update**](EventsApi.md#events_events_update) | **PUT** /events/events/{event_uuid}/ | 
[**events_events_volume_list**](EventsApi.md#events_events_volume_list) | **GET** /events/events/volume/ | 
[**events_notifications_destroy**](EventsApi.md#events_notifications_destroy) | **DELETE** /events/notifications/{uuid}/ | 
[**events_notifications_list**](EventsApi.md#events_notifications_list) | **GET** /events/notifications/ | 
[**events_notifications_mark_all_seen_create**](EventsApi.md#events_notifications_mark_all_seen_create) | **POST** /events/notifications/mark_all_seen/ | 
[**events_notifications_partial_update**](EventsApi.md#events_notifications_partial_update) | **PATCH** /events/notifications/{uuid}/ | 
[**events_notifications_retrieve**](EventsApi.md#events_notifications_retrieve) | **GET** /events/notifications/{uuid}/ | 
[**events_notifications_update**](EventsApi.md#events_notifications_update) | **PUT** /events/notifications/{uuid}/ | 
[**events_notifications_used_by_list**](EventsApi.md#events_notifications_used_by_list) | **GET** /events/notifications/{uuid}/used_by/ | 
[**events_rules_create**](EventsApi.md#events_rules_create) | **POST** /events/rules/ | 
[**events_rules_destroy**](EventsApi.md#events_rules_destroy) | **DELETE** /events/rules/{pbm_uuid}/ | 
[**events_rules_list**](EventsApi.md#events_rules_list) | **GET** /events/rules/ | 
[**events_rules_partial_update**](EventsApi.md#events_rules_partial_update) | **PATCH** /events/rules/{pbm_uuid}/ | 
[**events_rules_retrieve**](EventsApi.md#events_rules_retrieve) | **GET** /events/rules/{pbm_uuid}/ | 
[**events_rules_update**](EventsApi.md#events_rules_update) | **PUT** /events/rules/{pbm_uuid}/ | 
[**events_rules_used_by_list**](EventsApi.md#events_rules_used_by_list) | **GET** /events/rules/{pbm_uuid}/used_by/ | 
[**events_system_tasks_list**](EventsApi.md#events_system_tasks_list) | **GET** /events/system_tasks/ | 
[**events_system_tasks_retrieve**](EventsApi.md#events_system_tasks_retrieve) | **GET** /events/system_tasks/{uuid}/ | 
[**events_system_tasks_run_create**](EventsApi.md#events_system_tasks_run_create) | **POST** /events/system_tasks/{uuid}/run/ | 
[**events_transports_create**](EventsApi.md#events_transports_create) | **POST** /events/transports/ | 
[**events_transports_destroy**](EventsApi.md#events_transports_destroy) | **DELETE** /events/transports/{uuid}/ | 
[**events_transports_list**](EventsApi.md#events_transports_list) | **GET** /events/transports/ | 
[**events_transports_partial_update**](EventsApi.md#events_transports_partial_update) | **PATCH** /events/transports/{uuid}/ | 
[**events_transports_retrieve**](EventsApi.md#events_transports_retrieve) | **GET** /events/transports/{uuid}/ | 
[**events_transports_test_create**](EventsApi.md#events_transports_test_create) | **POST** /events/transports/{uuid}/test/ | 
[**events_transports_update**](EventsApi.md#events_transports_update) | **PUT** /events/transports/{uuid}/ | 
[**events_transports_used_by_list**](EventsApi.md#events_transports_used_by_list) | **GET** /events/transports/{uuid}/used_by/ | 



## events_events_actions_list

> Vec<models::TypeCreate> events_events_actions_list()


Get all actions

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


## events_events_create

> models::Event events_events_create(event_request)


Event Read-Only Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_request** | [**EventRequest**](EventRequest.md) |  | [required] |

### Return type

[**models::Event**](Event.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_events_destroy

> events_events_destroy(event_uuid)


Event Read-Only Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_uuid** | **uuid::Uuid** | A UUID string identifying this Event. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_events_list

> models::PaginatedEventList events_events_list(action, brand_name, client_ip, context_authorized_app, context_model_app, context_model_name, context_model_pk, ordering, page, page_size, search, username)


Event Read-Only Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action** | Option<**String**> |  |  |
**brand_name** | Option<**String**> | Brand name |  |
**client_ip** | Option<**String**> |  |  |
**context_authorized_app** | Option<**String**> | Context Authorized application |  |
**context_model_app** | Option<**String**> | Context Model App |  |
**context_model_name** | Option<**String**> | Context Model Name |  |
**context_model_pk** | Option<**String**> | Context Model Primary Key |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**username** | Option<**String**> | Username |  |

### Return type

[**models::PaginatedEventList**](PaginatedEventList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_events_partial_update

> models::Event events_events_partial_update(event_uuid, patched_event_request)


Event Read-Only Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_uuid** | **uuid::Uuid** | A UUID string identifying this Event. | [required] |
**patched_event_request** | Option<[**PatchedEventRequest**](PatchedEventRequest.md)> |  |  |

### Return type

[**models::Event**](Event.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_events_per_month_list

> Vec<models::Coordinate> events_events_per_month_list(action, query)


Get the count of events per month

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action** | Option<**String**> |  |  |
**query** | Option<**String**> |  |  |

### Return type

[**Vec<models::Coordinate>**](Coordinate.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_events_retrieve

> models::Event events_events_retrieve(event_uuid)


Event Read-Only Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_uuid** | **uuid::Uuid** | A UUID string identifying this Event. | [required] |

### Return type

[**models::Event**](Event.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_events_top_per_user_list

> Vec<models::EventTopPerUser> events_events_top_per_user_list(action, top_n)


Get the top_n events grouped by user count

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action** | Option<**String**> |  |  |
**top_n** | Option<**i32**> |  |  |

### Return type

[**Vec<models::EventTopPerUser>**](EventTopPerUser.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_events_update

> models::Event events_events_update(event_uuid, event_request)


Event Read-Only Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_uuid** | **uuid::Uuid** | A UUID string identifying this Event. | [required] |
**event_request** | [**EventRequest**](EventRequest.md) |  | [required] |

### Return type

[**models::Event**](Event.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_events_volume_list

> Vec<models::Coordinate> events_events_volume_list(action, brand_name, client_ip, context_authorized_app, context_model_app, context_model_name, context_model_pk, ordering, search, username)


Get event volume for specified filters and timeframe

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action** | Option<**String**> |  |  |
**brand_name** | Option<**String**> | Brand name |  |
**client_ip** | Option<**String**> |  |  |
**context_authorized_app** | Option<**String**> | Context Authorized application |  |
**context_model_app** | Option<**String**> | Context Model App |  |
**context_model_name** | Option<**String**> | Context Model Name |  |
**context_model_pk** | Option<**String**> | Context Model Primary Key |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**search** | Option<**String**> | A search term. |  |
**username** | Option<**String**> | Username |  |

### Return type

[**Vec<models::Coordinate>**](Coordinate.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_notifications_destroy

> events_notifications_destroy(uuid)


Notification Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Notification. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_notifications_list

> models::PaginatedNotificationList events_notifications_list(body, created, event, ordering, page, page_size, search, seen, severity, user)


Notification Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**String**> |  |  |
**created** | Option<**String**> |  |  |
**event** | Option<**uuid::Uuid**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**seen** | Option<**bool**> |  |  |
**severity** | Option<**String**> |  |  |
**user** | Option<**i32**> |  |  |

### Return type

[**models::PaginatedNotificationList**](PaginatedNotificationList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_notifications_mark_all_seen_create

> events_notifications_mark_all_seen_create()


Mark all the user's notifications as seen

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


## events_notifications_partial_update

> models::Notification events_notifications_partial_update(uuid, patched_notification_request)


Notification Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Notification. | [required] |
**patched_notification_request** | Option<[**PatchedNotificationRequest**](PatchedNotificationRequest.md)> |  |  |

### Return type

[**models::Notification**](Notification.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_notifications_retrieve

> models::Notification events_notifications_retrieve(uuid)


Notification Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Notification. | [required] |

### Return type

[**models::Notification**](Notification.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_notifications_update

> models::Notification events_notifications_update(uuid, notification_request)


Notification Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Notification. | [required] |
**notification_request** | Option<[**NotificationRequest**](NotificationRequest.md)> |  |  |

### Return type

[**models::Notification**](Notification.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_notifications_used_by_list

> Vec<models::UsedBy> events_notifications_used_by_list(uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Notification. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_rules_create

> models::NotificationRule events_rules_create(notification_rule_request)


NotificationRule Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_rule_request** | [**NotificationRuleRequest**](NotificationRuleRequest.md) |  | [required] |

### Return type

[**models::NotificationRule**](NotificationRule.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_rules_destroy

> events_rules_destroy(pbm_uuid)


NotificationRule Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pbm_uuid** | **uuid::Uuid** | A UUID string identifying this Notification Rule. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_rules_list

> models::PaginatedNotificationRuleList events_rules_list(group__name, name, ordering, page, page_size, search, severity)


NotificationRule Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group__name** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**severity** | Option<**String**> | Controls which severity level the created notifications will have.   |  |

### Return type

[**models::PaginatedNotificationRuleList**](PaginatedNotificationRuleList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_rules_partial_update

> models::NotificationRule events_rules_partial_update(pbm_uuid, patched_notification_rule_request)


NotificationRule Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pbm_uuid** | **uuid::Uuid** | A UUID string identifying this Notification Rule. | [required] |
**patched_notification_rule_request** | Option<[**PatchedNotificationRuleRequest**](PatchedNotificationRuleRequest.md)> |  |  |

### Return type

[**models::NotificationRule**](NotificationRule.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_rules_retrieve

> models::NotificationRule events_rules_retrieve(pbm_uuid)


NotificationRule Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pbm_uuid** | **uuid::Uuid** | A UUID string identifying this Notification Rule. | [required] |

### Return type

[**models::NotificationRule**](NotificationRule.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_rules_update

> models::NotificationRule events_rules_update(pbm_uuid, notification_rule_request)


NotificationRule Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pbm_uuid** | **uuid::Uuid** | A UUID string identifying this Notification Rule. | [required] |
**notification_rule_request** | [**NotificationRuleRequest**](NotificationRuleRequest.md) |  | [required] |

### Return type

[**models::NotificationRule**](NotificationRule.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_rules_used_by_list

> Vec<models::UsedBy> events_rules_used_by_list(pbm_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pbm_uuid** | **uuid::Uuid** | A UUID string identifying this Notification Rule. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_system_tasks_list

> models::PaginatedSystemTaskList events_system_tasks_list(name, ordering, page, page_size, search, status, uid)


Read-only view set that returns all background tasks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**status** | Option<**String**> |  |  |
**uid** | Option<**String**> |  |  |

### Return type

[**models::PaginatedSystemTaskList**](PaginatedSystemTaskList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_system_tasks_retrieve

> models::SystemTask events_system_tasks_retrieve(uuid)


Read-only view set that returns all background tasks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this System Task. | [required] |

### Return type

[**models::SystemTask**](SystemTask.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_system_tasks_run_create

> events_system_tasks_run_create(uuid)


Run task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this System Task. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_transports_create

> models::NotificationTransport events_transports_create(notification_transport_request)


NotificationTransport Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_transport_request** | [**NotificationTransportRequest**](NotificationTransportRequest.md) |  | [required] |

### Return type

[**models::NotificationTransport**](NotificationTransport.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_transports_destroy

> events_transports_destroy(uuid)


NotificationTransport Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Notification Transport. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_transports_list

> models::PaginatedNotificationTransportList events_transports_list(mode, name, ordering, page, page_size, search, send_once, webhook_url)


NotificationTransport Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mode** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**send_once** | Option<**bool**> |  |  |
**webhook_url** | Option<**String**> |  |  |

### Return type

[**models::PaginatedNotificationTransportList**](PaginatedNotificationTransportList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_transports_partial_update

> models::NotificationTransport events_transports_partial_update(uuid, patched_notification_transport_request)


NotificationTransport Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Notification Transport. | [required] |
**patched_notification_transport_request** | Option<[**PatchedNotificationTransportRequest**](PatchedNotificationTransportRequest.md)> |  |  |

### Return type

[**models::NotificationTransport**](NotificationTransport.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_transports_retrieve

> models::NotificationTransport events_transports_retrieve(uuid)


NotificationTransport Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Notification Transport. | [required] |

### Return type

[**models::NotificationTransport**](NotificationTransport.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_transports_test_create

> models::NotificationTransportTest events_transports_test_create(uuid)


Send example notification using selected transport. Requires Modify permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Notification Transport. | [required] |

### Return type

[**models::NotificationTransportTest**](NotificationTransportTest.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_transports_update

> models::NotificationTransport events_transports_update(uuid, notification_transport_request)


NotificationTransport Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Notification Transport. | [required] |
**notification_transport_request** | [**NotificationTransportRequest**](NotificationTransportRequest.md) |  | [required] |

### Return type

[**models::NotificationTransport**](NotificationTransport.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_transports_used_by_list

> Vec<models::UsedBy> events_transports_used_by_list(uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Notification Transport. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

