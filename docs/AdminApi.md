# \AdminApi

All URIs are relative to *http://localhost/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_apps_list**](AdminApi.md#admin_apps_list) | **GET** /admin/apps/ | 
[**admin_metrics_retrieve**](AdminApi.md#admin_metrics_retrieve) | **GET** /admin/metrics/ | 
[**admin_models_list**](AdminApi.md#admin_models_list) | **GET** /admin/models/ | 
[**admin_settings_partial_update**](AdminApi.md#admin_settings_partial_update) | **PATCH** /admin/settings/ | 
[**admin_settings_retrieve**](AdminApi.md#admin_settings_retrieve) | **GET** /admin/settings/ | 
[**admin_settings_update**](AdminApi.md#admin_settings_update) | **PUT** /admin/settings/ | 
[**admin_system_create**](AdminApi.md#admin_system_create) | **POST** /admin/system/ | 
[**admin_system_retrieve**](AdminApi.md#admin_system_retrieve) | **GET** /admin/system/ | 
[**admin_version_history_list**](AdminApi.md#admin_version_history_list) | **GET** /admin/version/history/ | 
[**admin_version_history_retrieve**](AdminApi.md#admin_version_history_retrieve) | **GET** /admin/version/history/{id}/ | 
[**admin_version_retrieve**](AdminApi.md#admin_version_retrieve) | **GET** /admin/version/ | 
[**admin_workers_retrieve**](AdminApi.md#admin_workers_retrieve) | **GET** /admin/workers/ | 



## admin_apps_list

> Vec<models::App> admin_apps_list()


Read-only view list all installed apps

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::App>**](App.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_metrics_retrieve

> models::LoginMetrics admin_metrics_retrieve()


Login Metrics per 1h

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::LoginMetrics**](LoginMetrics.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_models_list

> Vec<models::App> admin_models_list()


Read-only view list all installed models

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::App>**](App.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_settings_partial_update

> models::Settings admin_settings_partial_update(patched_settings_request)


Settings view

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patched_settings_request** | Option<[**PatchedSettingsRequest**](PatchedSettingsRequest.md)> |  |  |

### Return type

[**models::Settings**](Settings.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_settings_retrieve

> models::Settings admin_settings_retrieve()


Settings view

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Settings**](Settings.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_settings_update

> models::Settings admin_settings_update(settings_request)


Settings view

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**settings_request** | Option<[**SettingsRequest**](SettingsRequest.md)> |  |  |

### Return type

[**models::Settings**](Settings.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_system_create

> models::SystemInfo admin_system_create()


Get system information.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SystemInfo**](SystemInfo.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_system_retrieve

> models::SystemInfo admin_system_retrieve()


Get system information.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SystemInfo**](SystemInfo.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_version_history_list

> Vec<models::VersionHistory> admin_version_history_list(build, ordering, search, version)


VersionHistory Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**search** | Option<**String**> | A search term. |  |
**version** | Option<**String**> |  |  |

### Return type

[**Vec<models::VersionHistory>**](VersionHistory.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_version_history_retrieve

> models::VersionHistory admin_version_history_retrieve(id)


VersionHistory Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Version history. | [required] |

### Return type

[**models::VersionHistory**](VersionHistory.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_version_retrieve

> models::Version admin_version_retrieve()


Get running and latest version.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Version**](Version.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_workers_retrieve

> models::Workers admin_workers_retrieve()


Get currently connected worker count.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Workers**](Workers.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

