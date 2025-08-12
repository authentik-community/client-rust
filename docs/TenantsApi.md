# \TenantsApi

All URIs are relative to */api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tenants_domains_create**](TenantsApi.md#tenants_domains_create) | **POST** /tenants/domains/ | 
[**tenants_domains_destroy**](TenantsApi.md#tenants_domains_destroy) | **DELETE** /tenants/domains/{id}/ | 
[**tenants_domains_list**](TenantsApi.md#tenants_domains_list) | **GET** /tenants/domains/ | 
[**tenants_domains_partial_update**](TenantsApi.md#tenants_domains_partial_update) | **PATCH** /tenants/domains/{id}/ | 
[**tenants_domains_retrieve**](TenantsApi.md#tenants_domains_retrieve) | **GET** /tenants/domains/{id}/ | 
[**tenants_domains_update**](TenantsApi.md#tenants_domains_update) | **PUT** /tenants/domains/{id}/ | 
[**tenants_tenants_create**](TenantsApi.md#tenants_tenants_create) | **POST** /tenants/tenants/ | 
[**tenants_tenants_create_admin_group_create**](TenantsApi.md#tenants_tenants_create_admin_group_create) | **POST** /tenants/tenants/{tenant_uuid}/create_admin_group/ | 
[**tenants_tenants_create_recovery_key_create**](TenantsApi.md#tenants_tenants_create_recovery_key_create) | **POST** /tenants/tenants/{tenant_uuid}/create_recovery_key/ | 
[**tenants_tenants_destroy**](TenantsApi.md#tenants_tenants_destroy) | **DELETE** /tenants/tenants/{tenant_uuid}/ | 
[**tenants_tenants_list**](TenantsApi.md#tenants_tenants_list) | **GET** /tenants/tenants/ | 
[**tenants_tenants_partial_update**](TenantsApi.md#tenants_tenants_partial_update) | **PATCH** /tenants/tenants/{tenant_uuid}/ | 
[**tenants_tenants_retrieve**](TenantsApi.md#tenants_tenants_retrieve) | **GET** /tenants/tenants/{tenant_uuid}/ | 
[**tenants_tenants_update**](TenantsApi.md#tenants_tenants_update) | **PUT** /tenants/tenants/{tenant_uuid}/ | 



## tenants_domains_create

> models::Domain tenants_domains_create(domain_request)


Domain ViewSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_request** | [**DomainRequest**](DomainRequest.md) |  | [required] |

### Return type

[**models::Domain**](Domain.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenants_domains_destroy

> tenants_domains_destroy(id)


Domain ViewSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Domain. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenants_domains_list

> models::PaginatedDomainList tenants_domains_list(ordering, page, page_size, search)


Domain ViewSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedDomainList**](PaginatedDomainList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenants_domains_partial_update

> models::Domain tenants_domains_partial_update(id, patched_domain_request)


Domain ViewSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Domain. | [required] |
**patched_domain_request** | Option<[**PatchedDomainRequest**](PatchedDomainRequest.md)> |  |  |

### Return type

[**models::Domain**](Domain.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenants_domains_retrieve

> models::Domain tenants_domains_retrieve(id)


Domain ViewSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Domain. | [required] |

### Return type

[**models::Domain**](Domain.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenants_domains_update

> models::Domain tenants_domains_update(id, domain_request)


Domain ViewSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Domain. | [required] |
**domain_request** | [**DomainRequest**](DomainRequest.md) |  | [required] |

### Return type

[**models::Domain**](Domain.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenants_tenants_create

> models::Tenant tenants_tenants_create(tenant_request)


Tenant Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_request** | [**TenantRequest**](TenantRequest.md) |  | [required] |

### Return type

[**models::Tenant**](Tenant.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenants_tenants_create_admin_group_create

> tenants_tenants_create_admin_group_create(tenant_uuid, tenant_admin_group_request_request)


Create admin group and add user to it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_uuid** | **uuid::Uuid** | A UUID string identifying this Tenant. | [required] |
**tenant_admin_group_request_request** | [**TenantAdminGroupRequestRequest**](TenantAdminGroupRequestRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenants_tenants_create_recovery_key_create

> models::TenantRecoveryKeyResponse tenants_tenants_create_recovery_key_create(tenant_uuid, tenant_recovery_key_request_request)


Create recovery key for user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_uuid** | **uuid::Uuid** | A UUID string identifying this Tenant. | [required] |
**tenant_recovery_key_request_request** | [**TenantRecoveryKeyRequestRequest**](TenantRecoveryKeyRequestRequest.md) |  | [required] |

### Return type

[**models::TenantRecoveryKeyResponse**](TenantRecoveryKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenants_tenants_destroy

> tenants_tenants_destroy(tenant_uuid)


Tenant Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_uuid** | **uuid::Uuid** | A UUID string identifying this Tenant. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenants_tenants_list

> models::PaginatedTenantList tenants_tenants_list(ordering, page, page_size, search)


Tenant Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedTenantList**](PaginatedTenantList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenants_tenants_partial_update

> models::Tenant tenants_tenants_partial_update(tenant_uuid, patched_tenant_request)


Tenant Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_uuid** | **uuid::Uuid** | A UUID string identifying this Tenant. | [required] |
**patched_tenant_request** | Option<[**PatchedTenantRequest**](PatchedTenantRequest.md)> |  |  |

### Return type

[**models::Tenant**](Tenant.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenants_tenants_retrieve

> models::Tenant tenants_tenants_retrieve(tenant_uuid)


Tenant Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_uuid** | **uuid::Uuid** | A UUID string identifying this Tenant. | [required] |

### Return type

[**models::Tenant**](Tenant.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenants_tenants_update

> models::Tenant tenants_tenants_update(tenant_uuid, tenant_request)


Tenant Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_uuid** | **uuid::Uuid** | A UUID string identifying this Tenant. | [required] |
**tenant_request** | [**TenantRequest**](TenantRequest.md) |  | [required] |

### Return type

[**models::Tenant**](Tenant.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

