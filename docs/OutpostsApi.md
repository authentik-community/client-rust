# \OutpostsApi

All URIs are relative to */api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**outposts_instances_create**](OutpostsApi.md#outposts_instances_create) | **POST** /outposts/instances/ | 
[**outposts_instances_default_settings_retrieve**](OutpostsApi.md#outposts_instances_default_settings_retrieve) | **GET** /outposts/instances/default_settings/ | 
[**outposts_instances_destroy**](OutpostsApi.md#outposts_instances_destroy) | **DELETE** /outposts/instances/{uuid}/ | 
[**outposts_instances_health_list**](OutpostsApi.md#outposts_instances_health_list) | **GET** /outposts/instances/{uuid}/health/ | 
[**outposts_instances_list**](OutpostsApi.md#outposts_instances_list) | **GET** /outposts/instances/ | 
[**outposts_instances_partial_update**](OutpostsApi.md#outposts_instances_partial_update) | **PATCH** /outposts/instances/{uuid}/ | 
[**outposts_instances_retrieve**](OutpostsApi.md#outposts_instances_retrieve) | **GET** /outposts/instances/{uuid}/ | 
[**outposts_instances_update**](OutpostsApi.md#outposts_instances_update) | **PUT** /outposts/instances/{uuid}/ | 
[**outposts_instances_used_by_list**](OutpostsApi.md#outposts_instances_used_by_list) | **GET** /outposts/instances/{uuid}/used_by/ | 
[**outposts_ldap_access_check**](OutpostsApi.md#outposts_ldap_access_check) | **GET** /outposts/ldap/{id}/check_access/ | 
[**outposts_ldap_list**](OutpostsApi.md#outposts_ldap_list) | **GET** /outposts/ldap/ | 
[**outposts_proxy_list**](OutpostsApi.md#outposts_proxy_list) | **GET** /outposts/proxy/ | 
[**outposts_radius_access_check**](OutpostsApi.md#outposts_radius_access_check) | **GET** /outposts/radius/{id}/check_access/ | 
[**outposts_radius_list**](OutpostsApi.md#outposts_radius_list) | **GET** /outposts/radius/ | 
[**outposts_service_connections_all_destroy**](OutpostsApi.md#outposts_service_connections_all_destroy) | **DELETE** /outposts/service_connections/all/{uuid}/ | 
[**outposts_service_connections_all_list**](OutpostsApi.md#outposts_service_connections_all_list) | **GET** /outposts/service_connections/all/ | 
[**outposts_service_connections_all_retrieve**](OutpostsApi.md#outposts_service_connections_all_retrieve) | **GET** /outposts/service_connections/all/{uuid}/ | 
[**outposts_service_connections_all_state_retrieve**](OutpostsApi.md#outposts_service_connections_all_state_retrieve) | **GET** /outposts/service_connections/all/{uuid}/state/ | 
[**outposts_service_connections_all_types_list**](OutpostsApi.md#outposts_service_connections_all_types_list) | **GET** /outposts/service_connections/all/types/ | 
[**outposts_service_connections_all_used_by_list**](OutpostsApi.md#outposts_service_connections_all_used_by_list) | **GET** /outposts/service_connections/all/{uuid}/used_by/ | 
[**outposts_service_connections_docker_create**](OutpostsApi.md#outposts_service_connections_docker_create) | **POST** /outposts/service_connections/docker/ | 
[**outposts_service_connections_docker_destroy**](OutpostsApi.md#outposts_service_connections_docker_destroy) | **DELETE** /outposts/service_connections/docker/{uuid}/ | 
[**outposts_service_connections_docker_list**](OutpostsApi.md#outposts_service_connections_docker_list) | **GET** /outposts/service_connections/docker/ | 
[**outposts_service_connections_docker_partial_update**](OutpostsApi.md#outposts_service_connections_docker_partial_update) | **PATCH** /outposts/service_connections/docker/{uuid}/ | 
[**outposts_service_connections_docker_retrieve**](OutpostsApi.md#outposts_service_connections_docker_retrieve) | **GET** /outposts/service_connections/docker/{uuid}/ | 
[**outposts_service_connections_docker_update**](OutpostsApi.md#outposts_service_connections_docker_update) | **PUT** /outposts/service_connections/docker/{uuid}/ | 
[**outposts_service_connections_docker_used_by_list**](OutpostsApi.md#outposts_service_connections_docker_used_by_list) | **GET** /outposts/service_connections/docker/{uuid}/used_by/ | 
[**outposts_service_connections_kubernetes_create**](OutpostsApi.md#outposts_service_connections_kubernetes_create) | **POST** /outposts/service_connections/kubernetes/ | 
[**outposts_service_connections_kubernetes_destroy**](OutpostsApi.md#outposts_service_connections_kubernetes_destroy) | **DELETE** /outposts/service_connections/kubernetes/{uuid}/ | 
[**outposts_service_connections_kubernetes_list**](OutpostsApi.md#outposts_service_connections_kubernetes_list) | **GET** /outposts/service_connections/kubernetes/ | 
[**outposts_service_connections_kubernetes_partial_update**](OutpostsApi.md#outposts_service_connections_kubernetes_partial_update) | **PATCH** /outposts/service_connections/kubernetes/{uuid}/ | 
[**outposts_service_connections_kubernetes_retrieve**](OutpostsApi.md#outposts_service_connections_kubernetes_retrieve) | **GET** /outposts/service_connections/kubernetes/{uuid}/ | 
[**outposts_service_connections_kubernetes_update**](OutpostsApi.md#outposts_service_connections_kubernetes_update) | **PUT** /outposts/service_connections/kubernetes/{uuid}/ | 
[**outposts_service_connections_kubernetes_used_by_list**](OutpostsApi.md#outposts_service_connections_kubernetes_used_by_list) | **GET** /outposts/service_connections/kubernetes/{uuid}/used_by/ | 



## outposts_instances_create

> models::Outpost outposts_instances_create(outpost_request)


Outpost Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**outpost_request** | [**OutpostRequest**](OutpostRequest.md) |  | [required] |

### Return type

[**models::Outpost**](Outpost.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_instances_default_settings_retrieve

> models::OutpostDefaultConfig outposts_instances_default_settings_retrieve()


Global default outpost config

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::OutpostDefaultConfig**](OutpostDefaultConfig.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_instances_destroy

> outposts_instances_destroy(uuid)


Outpost Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Outpost. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_instances_health_list

> Vec<models::OutpostHealth> outposts_instances_health_list(uuid, managed__icontains, managed__iexact, name__icontains, name__iexact, ordering, providers__isnull, providers_by_pk, search, service_connection__name__icontains, service_connection__name__iexact)


Get outposts current health

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Outpost. | [required] |
**managed__icontains** | Option<**String**> |  |  |
**managed__iexact** | Option<**String**> |  |  |
**name__icontains** | Option<**String**> |  |  |
**name__iexact** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**providers__isnull** | Option<**bool**> |  |  |
**providers_by_pk** | Option<[**Vec<i32>**](I32.md)> |  |  |
**search** | Option<**String**> | A search term. |  |
**service_connection__name__icontains** | Option<**String**> |  |  |
**service_connection__name__iexact** | Option<**String**> |  |  |

### Return type

[**Vec<models::OutpostHealth>**](OutpostHealth.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_instances_list

> models::PaginatedOutpostList outposts_instances_list(managed__icontains, managed__iexact, name__icontains, name__iexact, ordering, page, page_size, providers__isnull, providers_by_pk, search, service_connection__name__icontains, service_connection__name__iexact)


Outpost Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**managed__icontains** | Option<**String**> |  |  |
**managed__iexact** | Option<**String**> |  |  |
**name__icontains** | Option<**String**> |  |  |
**name__iexact** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**providers__isnull** | Option<**bool**> |  |  |
**providers_by_pk** | Option<[**Vec<i32>**](I32.md)> |  |  |
**search** | Option<**String**> | A search term. |  |
**service_connection__name__icontains** | Option<**String**> |  |  |
**service_connection__name__iexact** | Option<**String**> |  |  |

### Return type

[**models::PaginatedOutpostList**](PaginatedOutpostList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_instances_partial_update

> models::Outpost outposts_instances_partial_update(uuid, patched_outpost_request)


Outpost Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Outpost. | [required] |
**patched_outpost_request** | Option<[**PatchedOutpostRequest**](PatchedOutpostRequest.md)> |  |  |

### Return type

[**models::Outpost**](Outpost.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_instances_retrieve

> models::Outpost outposts_instances_retrieve(uuid)


Outpost Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Outpost. | [required] |

### Return type

[**models::Outpost**](Outpost.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_instances_update

> models::Outpost outposts_instances_update(uuid, outpost_request)


Outpost Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Outpost. | [required] |
**outpost_request** | [**OutpostRequest**](OutpostRequest.md) |  | [required] |

### Return type

[**models::Outpost**](Outpost.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_instances_used_by_list

> Vec<models::UsedBy> outposts_instances_used_by_list(uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Outpost. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_ldap_access_check

> models::LdapCheckAccess outposts_ldap_access_check(id, app_slug)


Check access to a single application by slug

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this LDAP Provider. | [required] |
**app_slug** | Option<**String**> |  |  |

### Return type

[**models::LdapCheckAccess**](LDAPCheckAccess.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_ldap_list

> models::PaginatedLdapOutpostConfigList outposts_ldap_list(name, ordering, page, page_size, search)


LDAPProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedLdapOutpostConfigList**](PaginatedLDAPOutpostConfigList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_proxy_list

> models::PaginatedProxyOutpostConfigList outposts_proxy_list(name, ordering, page, page_size, search)


ProxyProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedProxyOutpostConfigList**](PaginatedProxyOutpostConfigList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_radius_access_check

> models::RadiusCheckAccess outposts_radius_access_check(id, app_slug)


Check access to a single application by slug

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Radius Provider. | [required] |
**app_slug** | Option<**String**> |  |  |

### Return type

[**models::RadiusCheckAccess**](RadiusCheckAccess.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_radius_list

> models::PaginatedRadiusOutpostConfigList outposts_radius_list(name, ordering, page, page_size, search)


RadiusProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedRadiusOutpostConfigList**](PaginatedRadiusOutpostConfigList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_service_connections_all_destroy

> outposts_service_connections_all_destroy(uuid)


ServiceConnection Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Outpost Service-Connection. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_service_connections_all_list

> models::PaginatedServiceConnectionList outposts_service_connections_all_list(name, ordering, page, page_size, search)


ServiceConnection Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedServiceConnectionList**](PaginatedServiceConnectionList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_service_connections_all_retrieve

> models::ServiceConnection outposts_service_connections_all_retrieve(uuid)


ServiceConnection Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Outpost Service-Connection. | [required] |

### Return type

[**models::ServiceConnection**](ServiceConnection.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_service_connections_all_state_retrieve

> models::ServiceConnectionState outposts_service_connections_all_state_retrieve(uuid)


Get the service connection's state

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Outpost Service-Connection. | [required] |

### Return type

[**models::ServiceConnectionState**](ServiceConnectionState.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_service_connections_all_types_list

> Vec<models::TypeCreate> outposts_service_connections_all_types_list()


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


## outposts_service_connections_all_used_by_list

> Vec<models::UsedBy> outposts_service_connections_all_used_by_list(uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Outpost Service-Connection. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_service_connections_docker_create

> models::DockerServiceConnection outposts_service_connections_docker_create(docker_service_connection_request)


DockerServiceConnection Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**docker_service_connection_request** | [**DockerServiceConnectionRequest**](DockerServiceConnectionRequest.md) |  | [required] |

### Return type

[**models::DockerServiceConnection**](DockerServiceConnection.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_service_connections_docker_destroy

> outposts_service_connections_docker_destroy(uuid)


DockerServiceConnection Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Docker Service-Connection. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_service_connections_docker_list

> models::PaginatedDockerServiceConnectionList outposts_service_connections_docker_list(local, name, ordering, page, page_size, search, tls_authentication, tls_verification, url)


DockerServiceConnection Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**local** | Option<**bool**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**tls_authentication** | Option<**uuid::Uuid**> |  |  |
**tls_verification** | Option<**uuid::Uuid**> |  |  |
**url** | Option<**String**> |  |  |

### Return type

[**models::PaginatedDockerServiceConnectionList**](PaginatedDockerServiceConnectionList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_service_connections_docker_partial_update

> models::DockerServiceConnection outposts_service_connections_docker_partial_update(uuid, patched_docker_service_connection_request)


DockerServiceConnection Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Docker Service-Connection. | [required] |
**patched_docker_service_connection_request** | Option<[**PatchedDockerServiceConnectionRequest**](PatchedDockerServiceConnectionRequest.md)> |  |  |

### Return type

[**models::DockerServiceConnection**](DockerServiceConnection.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_service_connections_docker_retrieve

> models::DockerServiceConnection outposts_service_connections_docker_retrieve(uuid)


DockerServiceConnection Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Docker Service-Connection. | [required] |

### Return type

[**models::DockerServiceConnection**](DockerServiceConnection.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_service_connections_docker_update

> models::DockerServiceConnection outposts_service_connections_docker_update(uuid, docker_service_connection_request)


DockerServiceConnection Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Docker Service-Connection. | [required] |
**docker_service_connection_request** | [**DockerServiceConnectionRequest**](DockerServiceConnectionRequest.md) |  | [required] |

### Return type

[**models::DockerServiceConnection**](DockerServiceConnection.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_service_connections_docker_used_by_list

> Vec<models::UsedBy> outposts_service_connections_docker_used_by_list(uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Docker Service-Connection. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_service_connections_kubernetes_create

> models::KubernetesServiceConnection outposts_service_connections_kubernetes_create(kubernetes_service_connection_request)


KubernetesServiceConnection Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kubernetes_service_connection_request** | [**KubernetesServiceConnectionRequest**](KubernetesServiceConnectionRequest.md) |  | [required] |

### Return type

[**models::KubernetesServiceConnection**](KubernetesServiceConnection.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_service_connections_kubernetes_destroy

> outposts_service_connections_kubernetes_destroy(uuid)


KubernetesServiceConnection Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Kubernetes Service-Connection. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_service_connections_kubernetes_list

> models::PaginatedKubernetesServiceConnectionList outposts_service_connections_kubernetes_list(local, name, ordering, page, page_size, search)


KubernetesServiceConnection Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**local** | Option<**bool**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedKubernetesServiceConnectionList**](PaginatedKubernetesServiceConnectionList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_service_connections_kubernetes_partial_update

> models::KubernetesServiceConnection outposts_service_connections_kubernetes_partial_update(uuid, patched_kubernetes_service_connection_request)


KubernetesServiceConnection Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Kubernetes Service-Connection. | [required] |
**patched_kubernetes_service_connection_request** | Option<[**PatchedKubernetesServiceConnectionRequest**](PatchedKubernetesServiceConnectionRequest.md)> |  |  |

### Return type

[**models::KubernetesServiceConnection**](KubernetesServiceConnection.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_service_connections_kubernetes_retrieve

> models::KubernetesServiceConnection outposts_service_connections_kubernetes_retrieve(uuid)


KubernetesServiceConnection Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Kubernetes Service-Connection. | [required] |

### Return type

[**models::KubernetesServiceConnection**](KubernetesServiceConnection.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_service_connections_kubernetes_update

> models::KubernetesServiceConnection outposts_service_connections_kubernetes_update(uuid, kubernetes_service_connection_request)


KubernetesServiceConnection Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Kubernetes Service-Connection. | [required] |
**kubernetes_service_connection_request** | [**KubernetesServiceConnectionRequest**](KubernetesServiceConnectionRequest.md) |  | [required] |

### Return type

[**models::KubernetesServiceConnection**](KubernetesServiceConnection.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outposts_service_connections_kubernetes_used_by_list

> Vec<models::UsedBy> outposts_service_connections_kubernetes_used_by_list(uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Kubernetes Service-Connection. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

