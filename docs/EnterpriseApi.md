# \EnterpriseApi

All URIs are relative to */api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**enterprise_license_create**](EnterpriseApi.md#enterprise_license_create) | **POST** /enterprise/license/ | 
[**enterprise_license_destroy**](EnterpriseApi.md#enterprise_license_destroy) | **DELETE** /enterprise/license/{license_uuid}/ | 
[**enterprise_license_forecast_retrieve**](EnterpriseApi.md#enterprise_license_forecast_retrieve) | **GET** /enterprise/license/forecast/ | 
[**enterprise_license_install_id_retrieve**](EnterpriseApi.md#enterprise_license_install_id_retrieve) | **GET** /enterprise/license/install_id/ | 
[**enterprise_license_list**](EnterpriseApi.md#enterprise_license_list) | **GET** /enterprise/license/ | 
[**enterprise_license_partial_update**](EnterpriseApi.md#enterprise_license_partial_update) | **PATCH** /enterprise/license/{license_uuid}/ | 
[**enterprise_license_retrieve**](EnterpriseApi.md#enterprise_license_retrieve) | **GET** /enterprise/license/{license_uuid}/ | 
[**enterprise_license_summary_retrieve**](EnterpriseApi.md#enterprise_license_summary_retrieve) | **GET** /enterprise/license/summary/ | 
[**enterprise_license_update**](EnterpriseApi.md#enterprise_license_update) | **PUT** /enterprise/license/{license_uuid}/ | 
[**enterprise_license_used_by_list**](EnterpriseApi.md#enterprise_license_used_by_list) | **GET** /enterprise/license/{license_uuid}/used_by/ | 



## enterprise_license_create

> models::License enterprise_license_create(license_request)


License Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_request** | [**LicenseRequest**](LicenseRequest.md) |  | [required] |

### Return type

[**models::License**](License.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_license_destroy

> enterprise_license_destroy(license_uuid)


License Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_uuid** | **uuid::Uuid** | A UUID string identifying this License. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_license_forecast_retrieve

> models::LicenseForecast enterprise_license_forecast_retrieve()


Forecast how many users will be required in a year

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::LicenseForecast**](LicenseForecast.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_license_install_id_retrieve

> models::InstallId enterprise_license_install_id_retrieve()


Get install_id

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::InstallId**](InstallID.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_license_list

> models::PaginatedLicenseList enterprise_license_list(name, ordering, page, page_size, search)


License Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedLicenseList**](PaginatedLicenseList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_license_partial_update

> models::License enterprise_license_partial_update(license_uuid, patched_license_request)


License Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_uuid** | **uuid::Uuid** | A UUID string identifying this License. | [required] |
**patched_license_request** | Option<[**PatchedLicenseRequest**](PatchedLicenseRequest.md)> |  |  |

### Return type

[**models::License**](License.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_license_retrieve

> models::License enterprise_license_retrieve(license_uuid)


License Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_uuid** | **uuid::Uuid** | A UUID string identifying this License. | [required] |

### Return type

[**models::License**](License.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_license_summary_retrieve

> models::LicenseSummary enterprise_license_summary_retrieve(cached)


Get the total license status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cached** | Option<**bool**> |  |  |[default to true]

### Return type

[**models::LicenseSummary**](LicenseSummary.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_license_update

> models::License enterprise_license_update(license_uuid, license_request)


License Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_uuid** | **uuid::Uuid** | A UUID string identifying this License. | [required] |
**license_request** | [**LicenseRequest**](LicenseRequest.md) |  | [required] |

### Return type

[**models::License**](License.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_license_used_by_list

> Vec<models::UsedBy> enterprise_license_used_by_list(license_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_uuid** | **uuid::Uuid** | A UUID string identifying this License. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

