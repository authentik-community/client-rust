# \ProvidersApi

All URIs are relative to */api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**providers_all_destroy**](ProvidersApi.md#providers_all_destroy) | **DELETE** /providers/all/{id}/ | 
[**providers_all_list**](ProvidersApi.md#providers_all_list) | **GET** /providers/all/ | 
[**providers_all_retrieve**](ProvidersApi.md#providers_all_retrieve) | **GET** /providers/all/{id}/ | 
[**providers_all_types_list**](ProvidersApi.md#providers_all_types_list) | **GET** /providers/all/types/ | 
[**providers_all_used_by_list**](ProvidersApi.md#providers_all_used_by_list) | **GET** /providers/all/{id}/used_by/ | 
[**providers_google_workspace_create**](ProvidersApi.md#providers_google_workspace_create) | **POST** /providers/google_workspace/ | 
[**providers_google_workspace_destroy**](ProvidersApi.md#providers_google_workspace_destroy) | **DELETE** /providers/google_workspace/{id}/ | 
[**providers_google_workspace_groups_create**](ProvidersApi.md#providers_google_workspace_groups_create) | **POST** /providers/google_workspace_groups/ | 
[**providers_google_workspace_groups_destroy**](ProvidersApi.md#providers_google_workspace_groups_destroy) | **DELETE** /providers/google_workspace_groups/{id}/ | 
[**providers_google_workspace_groups_list**](ProvidersApi.md#providers_google_workspace_groups_list) | **GET** /providers/google_workspace_groups/ | 
[**providers_google_workspace_groups_retrieve**](ProvidersApi.md#providers_google_workspace_groups_retrieve) | **GET** /providers/google_workspace_groups/{id}/ | 
[**providers_google_workspace_groups_used_by_list**](ProvidersApi.md#providers_google_workspace_groups_used_by_list) | **GET** /providers/google_workspace_groups/{id}/used_by/ | 
[**providers_google_workspace_list**](ProvidersApi.md#providers_google_workspace_list) | **GET** /providers/google_workspace/ | 
[**providers_google_workspace_partial_update**](ProvidersApi.md#providers_google_workspace_partial_update) | **PATCH** /providers/google_workspace/{id}/ | 
[**providers_google_workspace_retrieve**](ProvidersApi.md#providers_google_workspace_retrieve) | **GET** /providers/google_workspace/{id}/ | 
[**providers_google_workspace_sync_object_create**](ProvidersApi.md#providers_google_workspace_sync_object_create) | **POST** /providers/google_workspace/{id}/sync/object/ | 
[**providers_google_workspace_sync_status_retrieve**](ProvidersApi.md#providers_google_workspace_sync_status_retrieve) | **GET** /providers/google_workspace/{id}/sync/status/ | 
[**providers_google_workspace_update**](ProvidersApi.md#providers_google_workspace_update) | **PUT** /providers/google_workspace/{id}/ | 
[**providers_google_workspace_used_by_list**](ProvidersApi.md#providers_google_workspace_used_by_list) | **GET** /providers/google_workspace/{id}/used_by/ | 
[**providers_google_workspace_users_create**](ProvidersApi.md#providers_google_workspace_users_create) | **POST** /providers/google_workspace_users/ | 
[**providers_google_workspace_users_destroy**](ProvidersApi.md#providers_google_workspace_users_destroy) | **DELETE** /providers/google_workspace_users/{id}/ | 
[**providers_google_workspace_users_list**](ProvidersApi.md#providers_google_workspace_users_list) | **GET** /providers/google_workspace_users/ | 
[**providers_google_workspace_users_retrieve**](ProvidersApi.md#providers_google_workspace_users_retrieve) | **GET** /providers/google_workspace_users/{id}/ | 
[**providers_google_workspace_users_used_by_list**](ProvidersApi.md#providers_google_workspace_users_used_by_list) | **GET** /providers/google_workspace_users/{id}/used_by/ | 
[**providers_ldap_create**](ProvidersApi.md#providers_ldap_create) | **POST** /providers/ldap/ | 
[**providers_ldap_destroy**](ProvidersApi.md#providers_ldap_destroy) | **DELETE** /providers/ldap/{id}/ | 
[**providers_ldap_list**](ProvidersApi.md#providers_ldap_list) | **GET** /providers/ldap/ | 
[**providers_ldap_partial_update**](ProvidersApi.md#providers_ldap_partial_update) | **PATCH** /providers/ldap/{id}/ | 
[**providers_ldap_retrieve**](ProvidersApi.md#providers_ldap_retrieve) | **GET** /providers/ldap/{id}/ | 
[**providers_ldap_update**](ProvidersApi.md#providers_ldap_update) | **PUT** /providers/ldap/{id}/ | 
[**providers_ldap_used_by_list**](ProvidersApi.md#providers_ldap_used_by_list) | **GET** /providers/ldap/{id}/used_by/ | 
[**providers_microsoft_entra_create**](ProvidersApi.md#providers_microsoft_entra_create) | **POST** /providers/microsoft_entra/ | 
[**providers_microsoft_entra_destroy**](ProvidersApi.md#providers_microsoft_entra_destroy) | **DELETE** /providers/microsoft_entra/{id}/ | 
[**providers_microsoft_entra_groups_create**](ProvidersApi.md#providers_microsoft_entra_groups_create) | **POST** /providers/microsoft_entra_groups/ | 
[**providers_microsoft_entra_groups_destroy**](ProvidersApi.md#providers_microsoft_entra_groups_destroy) | **DELETE** /providers/microsoft_entra_groups/{id}/ | 
[**providers_microsoft_entra_groups_list**](ProvidersApi.md#providers_microsoft_entra_groups_list) | **GET** /providers/microsoft_entra_groups/ | 
[**providers_microsoft_entra_groups_retrieve**](ProvidersApi.md#providers_microsoft_entra_groups_retrieve) | **GET** /providers/microsoft_entra_groups/{id}/ | 
[**providers_microsoft_entra_groups_used_by_list**](ProvidersApi.md#providers_microsoft_entra_groups_used_by_list) | **GET** /providers/microsoft_entra_groups/{id}/used_by/ | 
[**providers_microsoft_entra_list**](ProvidersApi.md#providers_microsoft_entra_list) | **GET** /providers/microsoft_entra/ | 
[**providers_microsoft_entra_partial_update**](ProvidersApi.md#providers_microsoft_entra_partial_update) | **PATCH** /providers/microsoft_entra/{id}/ | 
[**providers_microsoft_entra_retrieve**](ProvidersApi.md#providers_microsoft_entra_retrieve) | **GET** /providers/microsoft_entra/{id}/ | 
[**providers_microsoft_entra_sync_object_create**](ProvidersApi.md#providers_microsoft_entra_sync_object_create) | **POST** /providers/microsoft_entra/{id}/sync/object/ | 
[**providers_microsoft_entra_sync_status_retrieve**](ProvidersApi.md#providers_microsoft_entra_sync_status_retrieve) | **GET** /providers/microsoft_entra/{id}/sync/status/ | 
[**providers_microsoft_entra_update**](ProvidersApi.md#providers_microsoft_entra_update) | **PUT** /providers/microsoft_entra/{id}/ | 
[**providers_microsoft_entra_used_by_list**](ProvidersApi.md#providers_microsoft_entra_used_by_list) | **GET** /providers/microsoft_entra/{id}/used_by/ | 
[**providers_microsoft_entra_users_create**](ProvidersApi.md#providers_microsoft_entra_users_create) | **POST** /providers/microsoft_entra_users/ | 
[**providers_microsoft_entra_users_destroy**](ProvidersApi.md#providers_microsoft_entra_users_destroy) | **DELETE** /providers/microsoft_entra_users/{id}/ | 
[**providers_microsoft_entra_users_list**](ProvidersApi.md#providers_microsoft_entra_users_list) | **GET** /providers/microsoft_entra_users/ | 
[**providers_microsoft_entra_users_retrieve**](ProvidersApi.md#providers_microsoft_entra_users_retrieve) | **GET** /providers/microsoft_entra_users/{id}/ | 
[**providers_microsoft_entra_users_used_by_list**](ProvidersApi.md#providers_microsoft_entra_users_used_by_list) | **GET** /providers/microsoft_entra_users/{id}/used_by/ | 
[**providers_oauth2_create**](ProvidersApi.md#providers_oauth2_create) | **POST** /providers/oauth2/ | 
[**providers_oauth2_destroy**](ProvidersApi.md#providers_oauth2_destroy) | **DELETE** /providers/oauth2/{id}/ | 
[**providers_oauth2_list**](ProvidersApi.md#providers_oauth2_list) | **GET** /providers/oauth2/ | 
[**providers_oauth2_partial_update**](ProvidersApi.md#providers_oauth2_partial_update) | **PATCH** /providers/oauth2/{id}/ | 
[**providers_oauth2_preview_user_retrieve**](ProvidersApi.md#providers_oauth2_preview_user_retrieve) | **GET** /providers/oauth2/{id}/preview_user/ | 
[**providers_oauth2_retrieve**](ProvidersApi.md#providers_oauth2_retrieve) | **GET** /providers/oauth2/{id}/ | 
[**providers_oauth2_setup_urls_retrieve**](ProvidersApi.md#providers_oauth2_setup_urls_retrieve) | **GET** /providers/oauth2/{id}/setup_urls/ | 
[**providers_oauth2_update**](ProvidersApi.md#providers_oauth2_update) | **PUT** /providers/oauth2/{id}/ | 
[**providers_oauth2_used_by_list**](ProvidersApi.md#providers_oauth2_used_by_list) | **GET** /providers/oauth2/{id}/used_by/ | 
[**providers_proxy_create**](ProvidersApi.md#providers_proxy_create) | **POST** /providers/proxy/ | 
[**providers_proxy_destroy**](ProvidersApi.md#providers_proxy_destroy) | **DELETE** /providers/proxy/{id}/ | 
[**providers_proxy_list**](ProvidersApi.md#providers_proxy_list) | **GET** /providers/proxy/ | 
[**providers_proxy_partial_update**](ProvidersApi.md#providers_proxy_partial_update) | **PATCH** /providers/proxy/{id}/ | 
[**providers_proxy_retrieve**](ProvidersApi.md#providers_proxy_retrieve) | **GET** /providers/proxy/{id}/ | 
[**providers_proxy_update**](ProvidersApi.md#providers_proxy_update) | **PUT** /providers/proxy/{id}/ | 
[**providers_proxy_used_by_list**](ProvidersApi.md#providers_proxy_used_by_list) | **GET** /providers/proxy/{id}/used_by/ | 
[**providers_rac_create**](ProvidersApi.md#providers_rac_create) | **POST** /providers/rac/ | 
[**providers_rac_destroy**](ProvidersApi.md#providers_rac_destroy) | **DELETE** /providers/rac/{id}/ | 
[**providers_rac_list**](ProvidersApi.md#providers_rac_list) | **GET** /providers/rac/ | 
[**providers_rac_partial_update**](ProvidersApi.md#providers_rac_partial_update) | **PATCH** /providers/rac/{id}/ | 
[**providers_rac_retrieve**](ProvidersApi.md#providers_rac_retrieve) | **GET** /providers/rac/{id}/ | 
[**providers_rac_update**](ProvidersApi.md#providers_rac_update) | **PUT** /providers/rac/{id}/ | 
[**providers_rac_used_by_list**](ProvidersApi.md#providers_rac_used_by_list) | **GET** /providers/rac/{id}/used_by/ | 
[**providers_radius_create**](ProvidersApi.md#providers_radius_create) | **POST** /providers/radius/ | 
[**providers_radius_destroy**](ProvidersApi.md#providers_radius_destroy) | **DELETE** /providers/radius/{id}/ | 
[**providers_radius_list**](ProvidersApi.md#providers_radius_list) | **GET** /providers/radius/ | 
[**providers_radius_partial_update**](ProvidersApi.md#providers_radius_partial_update) | **PATCH** /providers/radius/{id}/ | 
[**providers_radius_retrieve**](ProvidersApi.md#providers_radius_retrieve) | **GET** /providers/radius/{id}/ | 
[**providers_radius_update**](ProvidersApi.md#providers_radius_update) | **PUT** /providers/radius/{id}/ | 
[**providers_radius_used_by_list**](ProvidersApi.md#providers_radius_used_by_list) | **GET** /providers/radius/{id}/used_by/ | 
[**providers_saml_create**](ProvidersApi.md#providers_saml_create) | **POST** /providers/saml/ | 
[**providers_saml_destroy**](ProvidersApi.md#providers_saml_destroy) | **DELETE** /providers/saml/{id}/ | 
[**providers_saml_import_metadata_create**](ProvidersApi.md#providers_saml_import_metadata_create) | **POST** /providers/saml/import_metadata/ | 
[**providers_saml_list**](ProvidersApi.md#providers_saml_list) | **GET** /providers/saml/ | 
[**providers_saml_metadata_retrieve**](ProvidersApi.md#providers_saml_metadata_retrieve) | **GET** /providers/saml/{id}/metadata/ | 
[**providers_saml_partial_update**](ProvidersApi.md#providers_saml_partial_update) | **PATCH** /providers/saml/{id}/ | 
[**providers_saml_preview_user_retrieve**](ProvidersApi.md#providers_saml_preview_user_retrieve) | **GET** /providers/saml/{id}/preview_user/ | 
[**providers_saml_retrieve**](ProvidersApi.md#providers_saml_retrieve) | **GET** /providers/saml/{id}/ | 
[**providers_saml_update**](ProvidersApi.md#providers_saml_update) | **PUT** /providers/saml/{id}/ | 
[**providers_saml_used_by_list**](ProvidersApi.md#providers_saml_used_by_list) | **GET** /providers/saml/{id}/used_by/ | 
[**providers_scim_create**](ProvidersApi.md#providers_scim_create) | **POST** /providers/scim/ | 
[**providers_scim_destroy**](ProvidersApi.md#providers_scim_destroy) | **DELETE** /providers/scim/{id}/ | 
[**providers_scim_groups_create**](ProvidersApi.md#providers_scim_groups_create) | **POST** /providers/scim_groups/ | 
[**providers_scim_groups_destroy**](ProvidersApi.md#providers_scim_groups_destroy) | **DELETE** /providers/scim_groups/{id}/ | 
[**providers_scim_groups_list**](ProvidersApi.md#providers_scim_groups_list) | **GET** /providers/scim_groups/ | 
[**providers_scim_groups_retrieve**](ProvidersApi.md#providers_scim_groups_retrieve) | **GET** /providers/scim_groups/{id}/ | 
[**providers_scim_groups_used_by_list**](ProvidersApi.md#providers_scim_groups_used_by_list) | **GET** /providers/scim_groups/{id}/used_by/ | 
[**providers_scim_list**](ProvidersApi.md#providers_scim_list) | **GET** /providers/scim/ | 
[**providers_scim_partial_update**](ProvidersApi.md#providers_scim_partial_update) | **PATCH** /providers/scim/{id}/ | 
[**providers_scim_retrieve**](ProvidersApi.md#providers_scim_retrieve) | **GET** /providers/scim/{id}/ | 
[**providers_scim_sync_object_create**](ProvidersApi.md#providers_scim_sync_object_create) | **POST** /providers/scim/{id}/sync/object/ | 
[**providers_scim_sync_status_retrieve**](ProvidersApi.md#providers_scim_sync_status_retrieve) | **GET** /providers/scim/{id}/sync/status/ | 
[**providers_scim_update**](ProvidersApi.md#providers_scim_update) | **PUT** /providers/scim/{id}/ | 
[**providers_scim_used_by_list**](ProvidersApi.md#providers_scim_used_by_list) | **GET** /providers/scim/{id}/used_by/ | 
[**providers_scim_users_create**](ProvidersApi.md#providers_scim_users_create) | **POST** /providers/scim_users/ | 
[**providers_scim_users_destroy**](ProvidersApi.md#providers_scim_users_destroy) | **DELETE** /providers/scim_users/{id}/ | 
[**providers_scim_users_list**](ProvidersApi.md#providers_scim_users_list) | **GET** /providers/scim_users/ | 
[**providers_scim_users_retrieve**](ProvidersApi.md#providers_scim_users_retrieve) | **GET** /providers/scim_users/{id}/ | 
[**providers_scim_users_used_by_list**](ProvidersApi.md#providers_scim_users_used_by_list) | **GET** /providers/scim_users/{id}/used_by/ | 
[**providers_ssf_create**](ProvidersApi.md#providers_ssf_create) | **POST** /providers/ssf/ | 
[**providers_ssf_destroy**](ProvidersApi.md#providers_ssf_destroy) | **DELETE** /providers/ssf/{id}/ | 
[**providers_ssf_list**](ProvidersApi.md#providers_ssf_list) | **GET** /providers/ssf/ | 
[**providers_ssf_partial_update**](ProvidersApi.md#providers_ssf_partial_update) | **PATCH** /providers/ssf/{id}/ | 
[**providers_ssf_retrieve**](ProvidersApi.md#providers_ssf_retrieve) | **GET** /providers/ssf/{id}/ | 
[**providers_ssf_update**](ProvidersApi.md#providers_ssf_update) | **PUT** /providers/ssf/{id}/ | 
[**providers_ssf_used_by_list**](ProvidersApi.md#providers_ssf_used_by_list) | **GET** /providers/ssf/{id}/used_by/ | 



## providers_all_destroy

> providers_all_destroy(id)


Provider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this provider. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_all_list

> models::PaginatedProviderList providers_all_list(application__isnull, backchannel, ordering, page, page_size, search)


Provider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application__isnull** | Option<**bool**> |  |  |
**backchannel** | Option<**bool**> | When not set all providers are returned. When set to true, only backchannel providers are returned. When set to false, backchannel providers are excluded |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedProviderList**](PaginatedProviderList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_all_retrieve

> models::Provider providers_all_retrieve(id)


Provider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this provider. | [required] |

### Return type

[**models::Provider**](Provider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_all_types_list

> Vec<models::TypeCreate> providers_all_types_list()


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


## providers_all_used_by_list

> Vec<models::UsedBy> providers_all_used_by_list(id)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this provider. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_google_workspace_create

> models::GoogleWorkspaceProvider providers_google_workspace_create(google_workspace_provider_request)


GoogleWorkspaceProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**google_workspace_provider_request** | [**GoogleWorkspaceProviderRequest**](GoogleWorkspaceProviderRequest.md) |  | [required] |

### Return type

[**models::GoogleWorkspaceProvider**](GoogleWorkspaceProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_google_workspace_destroy

> providers_google_workspace_destroy(id)


GoogleWorkspaceProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Google Workspace Provider. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_google_workspace_groups_create

> models::GoogleWorkspaceProviderGroup providers_google_workspace_groups_create(google_workspace_provider_group_request)


GoogleWorkspaceProviderGroup Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**google_workspace_provider_group_request** | [**GoogleWorkspaceProviderGroupRequest**](GoogleWorkspaceProviderGroupRequest.md) |  | [required] |

### Return type

[**models::GoogleWorkspaceProviderGroup**](GoogleWorkspaceProviderGroup.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_google_workspace_groups_destroy

> providers_google_workspace_groups_destroy(id)


GoogleWorkspaceProviderGroup Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this Google Workspace Provider Group. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_google_workspace_groups_list

> models::PaginatedGoogleWorkspaceProviderGroupList providers_google_workspace_groups_list(group__group_uuid, group__name, ordering, page, page_size, provider__id, search)


GoogleWorkspaceProviderGroup Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group__group_uuid** | Option<**uuid::Uuid**> |  |  |
**group__name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**provider__id** | Option<**i32**> |  |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedGoogleWorkspaceProviderGroupList**](PaginatedGoogleWorkspaceProviderGroupList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_google_workspace_groups_retrieve

> models::GoogleWorkspaceProviderGroup providers_google_workspace_groups_retrieve(id)


GoogleWorkspaceProviderGroup Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this Google Workspace Provider Group. | [required] |

### Return type

[**models::GoogleWorkspaceProviderGroup**](GoogleWorkspaceProviderGroup.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_google_workspace_groups_used_by_list

> Vec<models::UsedBy> providers_google_workspace_groups_used_by_list(id)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this Google Workspace Provider Group. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_google_workspace_list

> models::PaginatedGoogleWorkspaceProviderList providers_google_workspace_list(delegated_subject, exclude_users_service_account, filter_group, name, ordering, page, page_size, search)


GoogleWorkspaceProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delegated_subject** | Option<**String**> |  |  |
**exclude_users_service_account** | Option<**bool**> |  |  |
**filter_group** | Option<**uuid::Uuid**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedGoogleWorkspaceProviderList**](PaginatedGoogleWorkspaceProviderList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_google_workspace_partial_update

> models::GoogleWorkspaceProvider providers_google_workspace_partial_update(id, patched_google_workspace_provider_request)


GoogleWorkspaceProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Google Workspace Provider. | [required] |
**patched_google_workspace_provider_request** | Option<[**PatchedGoogleWorkspaceProviderRequest**](PatchedGoogleWorkspaceProviderRequest.md)> |  |  |

### Return type

[**models::GoogleWorkspaceProvider**](GoogleWorkspaceProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_google_workspace_retrieve

> models::GoogleWorkspaceProvider providers_google_workspace_retrieve(id)


GoogleWorkspaceProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Google Workspace Provider. | [required] |

### Return type

[**models::GoogleWorkspaceProvider**](GoogleWorkspaceProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_google_workspace_sync_object_create

> models::SyncObjectResult providers_google_workspace_sync_object_create(id, sync_object_request)


Sync/Re-sync a single user/group object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Google Workspace Provider. | [required] |
**sync_object_request** | [**SyncObjectRequest**](SyncObjectRequest.md) |  | [required] |

### Return type

[**models::SyncObjectResult**](SyncObjectResult.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_google_workspace_sync_status_retrieve

> models::SyncStatus providers_google_workspace_sync_status_retrieve(id)


Get provider's sync status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Google Workspace Provider. | [required] |

### Return type

[**models::SyncStatus**](SyncStatus.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_google_workspace_update

> models::GoogleWorkspaceProvider providers_google_workspace_update(id, google_workspace_provider_request)


GoogleWorkspaceProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Google Workspace Provider. | [required] |
**google_workspace_provider_request** | [**GoogleWorkspaceProviderRequest**](GoogleWorkspaceProviderRequest.md) |  | [required] |

### Return type

[**models::GoogleWorkspaceProvider**](GoogleWorkspaceProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_google_workspace_used_by_list

> Vec<models::UsedBy> providers_google_workspace_used_by_list(id)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Google Workspace Provider. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_google_workspace_users_create

> models::GoogleWorkspaceProviderUser providers_google_workspace_users_create(google_workspace_provider_user_request)


GoogleWorkspaceProviderUser Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**google_workspace_provider_user_request** | [**GoogleWorkspaceProviderUserRequest**](GoogleWorkspaceProviderUserRequest.md) |  | [required] |

### Return type

[**models::GoogleWorkspaceProviderUser**](GoogleWorkspaceProviderUser.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_google_workspace_users_destroy

> providers_google_workspace_users_destroy(id)


GoogleWorkspaceProviderUser Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this Google Workspace Provider User. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_google_workspace_users_list

> models::PaginatedGoogleWorkspaceProviderUserList providers_google_workspace_users_list(ordering, page, page_size, provider__id, search, user__id, user__username)


GoogleWorkspaceProviderUser Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**provider__id** | Option<**i32**> |  |  |
**search** | Option<**String**> | A search term. |  |
**user__id** | Option<**i32**> |  |  |
**user__username** | Option<**String**> |  |  |

### Return type

[**models::PaginatedGoogleWorkspaceProviderUserList**](PaginatedGoogleWorkspaceProviderUserList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_google_workspace_users_retrieve

> models::GoogleWorkspaceProviderUser providers_google_workspace_users_retrieve(id)


GoogleWorkspaceProviderUser Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this Google Workspace Provider User. | [required] |

### Return type

[**models::GoogleWorkspaceProviderUser**](GoogleWorkspaceProviderUser.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_google_workspace_users_used_by_list

> Vec<models::UsedBy> providers_google_workspace_users_used_by_list(id)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this Google Workspace Provider User. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_ldap_create

> models::LdapProvider providers_ldap_create(ldap_provider_request)


LDAPProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ldap_provider_request** | [**LdapProviderRequest**](LdapProviderRequest.md) |  | [required] |

### Return type

[**models::LdapProvider**](LDAPProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_ldap_destroy

> providers_ldap_destroy(id)


LDAPProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this LDAP Provider. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_ldap_list

> models::PaginatedLdapProviderList providers_ldap_list(application__isnull, authorization_flow__slug__iexact, base_dn__iexact, certificate__kp_uuid__iexact, certificate__name__iexact, gid_start_number__iexact, name__iexact, ordering, page, page_size, search, tls_server_name__iexact, uid_start_number__iexact)


LDAPProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application__isnull** | Option<**bool**> |  |  |
**authorization_flow__slug__iexact** | Option<**String**> |  |  |
**base_dn__iexact** | Option<**String**> |  |  |
**certificate__kp_uuid__iexact** | Option<**uuid::Uuid**> |  |  |
**certificate__name__iexact** | Option<**String**> |  |  |
**gid_start_number__iexact** | Option<**i32**> |  |  |
**name__iexact** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**tls_server_name__iexact** | Option<**String**> |  |  |
**uid_start_number__iexact** | Option<**i32**> |  |  |

### Return type

[**models::PaginatedLdapProviderList**](PaginatedLDAPProviderList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_ldap_partial_update

> models::LdapProvider providers_ldap_partial_update(id, patched_ldap_provider_request)


LDAPProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this LDAP Provider. | [required] |
**patched_ldap_provider_request** | Option<[**PatchedLdapProviderRequest**](PatchedLdapProviderRequest.md)> |  |  |

### Return type

[**models::LdapProvider**](LDAPProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_ldap_retrieve

> models::LdapProvider providers_ldap_retrieve(id)


LDAPProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this LDAP Provider. | [required] |

### Return type

[**models::LdapProvider**](LDAPProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_ldap_update

> models::LdapProvider providers_ldap_update(id, ldap_provider_request)


LDAPProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this LDAP Provider. | [required] |
**ldap_provider_request** | [**LdapProviderRequest**](LdapProviderRequest.md) |  | [required] |

### Return type

[**models::LdapProvider**](LDAPProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_ldap_used_by_list

> Vec<models::UsedBy> providers_ldap_used_by_list(id)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this LDAP Provider. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_microsoft_entra_create

> models::MicrosoftEntraProvider providers_microsoft_entra_create(microsoft_entra_provider_request)


MicrosoftEntraProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**microsoft_entra_provider_request** | [**MicrosoftEntraProviderRequest**](MicrosoftEntraProviderRequest.md) |  | [required] |

### Return type

[**models::MicrosoftEntraProvider**](MicrosoftEntraProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_microsoft_entra_destroy

> providers_microsoft_entra_destroy(id)


MicrosoftEntraProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Microsoft Entra Provider. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_microsoft_entra_groups_create

> models::MicrosoftEntraProviderGroup providers_microsoft_entra_groups_create(microsoft_entra_provider_group_request)


MicrosoftEntraProviderGroup Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**microsoft_entra_provider_group_request** | [**MicrosoftEntraProviderGroupRequest**](MicrosoftEntraProviderGroupRequest.md) |  | [required] |

### Return type

[**models::MicrosoftEntraProviderGroup**](MicrosoftEntraProviderGroup.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_microsoft_entra_groups_destroy

> providers_microsoft_entra_groups_destroy(id)


MicrosoftEntraProviderGroup Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this Microsoft Entra Provider Group. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_microsoft_entra_groups_list

> models::PaginatedMicrosoftEntraProviderGroupList providers_microsoft_entra_groups_list(group__group_uuid, group__name, ordering, page, page_size, provider__id, search)


MicrosoftEntraProviderGroup Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group__group_uuid** | Option<**uuid::Uuid**> |  |  |
**group__name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**provider__id** | Option<**i32**> |  |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedMicrosoftEntraProviderGroupList**](PaginatedMicrosoftEntraProviderGroupList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_microsoft_entra_groups_retrieve

> models::MicrosoftEntraProviderGroup providers_microsoft_entra_groups_retrieve(id)


MicrosoftEntraProviderGroup Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this Microsoft Entra Provider Group. | [required] |

### Return type

[**models::MicrosoftEntraProviderGroup**](MicrosoftEntraProviderGroup.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_microsoft_entra_groups_used_by_list

> Vec<models::UsedBy> providers_microsoft_entra_groups_used_by_list(id)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this Microsoft Entra Provider Group. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_microsoft_entra_list

> models::PaginatedMicrosoftEntraProviderList providers_microsoft_entra_list(exclude_users_service_account, filter_group, name, ordering, page, page_size, search)


MicrosoftEntraProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exclude_users_service_account** | Option<**bool**> |  |  |
**filter_group** | Option<**uuid::Uuid**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedMicrosoftEntraProviderList**](PaginatedMicrosoftEntraProviderList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_microsoft_entra_partial_update

> models::MicrosoftEntraProvider providers_microsoft_entra_partial_update(id, patched_microsoft_entra_provider_request)


MicrosoftEntraProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Microsoft Entra Provider. | [required] |
**patched_microsoft_entra_provider_request** | Option<[**PatchedMicrosoftEntraProviderRequest**](PatchedMicrosoftEntraProviderRequest.md)> |  |  |

### Return type

[**models::MicrosoftEntraProvider**](MicrosoftEntraProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_microsoft_entra_retrieve

> models::MicrosoftEntraProvider providers_microsoft_entra_retrieve(id)


MicrosoftEntraProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Microsoft Entra Provider. | [required] |

### Return type

[**models::MicrosoftEntraProvider**](MicrosoftEntraProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_microsoft_entra_sync_object_create

> models::SyncObjectResult providers_microsoft_entra_sync_object_create(id, sync_object_request)


Sync/Re-sync a single user/group object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Microsoft Entra Provider. | [required] |
**sync_object_request** | [**SyncObjectRequest**](SyncObjectRequest.md) |  | [required] |

### Return type

[**models::SyncObjectResult**](SyncObjectResult.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_microsoft_entra_sync_status_retrieve

> models::SyncStatus providers_microsoft_entra_sync_status_retrieve(id)


Get provider's sync status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Microsoft Entra Provider. | [required] |

### Return type

[**models::SyncStatus**](SyncStatus.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_microsoft_entra_update

> models::MicrosoftEntraProvider providers_microsoft_entra_update(id, microsoft_entra_provider_request)


MicrosoftEntraProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Microsoft Entra Provider. | [required] |
**microsoft_entra_provider_request** | [**MicrosoftEntraProviderRequest**](MicrosoftEntraProviderRequest.md) |  | [required] |

### Return type

[**models::MicrosoftEntraProvider**](MicrosoftEntraProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_microsoft_entra_used_by_list

> Vec<models::UsedBy> providers_microsoft_entra_used_by_list(id)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Microsoft Entra Provider. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_microsoft_entra_users_create

> models::MicrosoftEntraProviderUser providers_microsoft_entra_users_create(microsoft_entra_provider_user_request)


MicrosoftEntraProviderUser Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**microsoft_entra_provider_user_request** | [**MicrosoftEntraProviderUserRequest**](MicrosoftEntraProviderUserRequest.md) |  | [required] |

### Return type

[**models::MicrosoftEntraProviderUser**](MicrosoftEntraProviderUser.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_microsoft_entra_users_destroy

> providers_microsoft_entra_users_destroy(id)


MicrosoftEntraProviderUser Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this Microsoft Entra Provider User. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_microsoft_entra_users_list

> models::PaginatedMicrosoftEntraProviderUserList providers_microsoft_entra_users_list(ordering, page, page_size, provider__id, search, user__id, user__username)


MicrosoftEntraProviderUser Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**provider__id** | Option<**i32**> |  |  |
**search** | Option<**String**> | A search term. |  |
**user__id** | Option<**i32**> |  |  |
**user__username** | Option<**String**> |  |  |

### Return type

[**models::PaginatedMicrosoftEntraProviderUserList**](PaginatedMicrosoftEntraProviderUserList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_microsoft_entra_users_retrieve

> models::MicrosoftEntraProviderUser providers_microsoft_entra_users_retrieve(id)


MicrosoftEntraProviderUser Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this Microsoft Entra Provider User. | [required] |

### Return type

[**models::MicrosoftEntraProviderUser**](MicrosoftEntraProviderUser.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_microsoft_entra_users_used_by_list

> Vec<models::UsedBy> providers_microsoft_entra_users_used_by_list(id)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this Microsoft Entra Provider User. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_oauth2_create

> models::OAuth2Provider providers_oauth2_create(o_auth2_provider_request)


OAuth2Provider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**o_auth2_provider_request** | [**OAuth2ProviderRequest**](OAuth2ProviderRequest.md) |  | [required] |

### Return type

[**models::OAuth2Provider**](OAuth2Provider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_oauth2_destroy

> providers_oauth2_destroy(id)


OAuth2Provider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this OAuth2/OpenID Provider. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_oauth2_list

> models::PaginatedOAuth2ProviderList providers_oauth2_list(access_code_validity, access_token_validity, application, authorization_flow, client_id, client_type, include_claims_in_id_token, issuer_mode, name, ordering, page, page_size, property_mappings, refresh_token_validity, search, signing_key, sub_mode)


OAuth2Provider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_code_validity** | Option<**String**> |  |  |
**access_token_validity** | Option<**String**> |  |  |
**application** | Option<**uuid::Uuid**> |  |  |
**authorization_flow** | Option<**uuid::Uuid**> |  |  |
**client_id** | Option<**String**> |  |  |
**client_type** | Option<**String**> | Confidential clients are capable of maintaining the confidentiality of their credentials. Public clients are incapable   |  |
**include_claims_in_id_token** | Option<**bool**> |  |  |
**issuer_mode** | Option<**String**> | Configure how the issuer field of the ID Token should be filled.   |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  |  |
**refresh_token_validity** | Option<**String**> |  |  |
**search** | Option<**String**> | A search term. |  |
**signing_key** | Option<**uuid::Uuid**> |  |  |
**sub_mode** | Option<**String**> | Configure what data should be used as unique User Identifier. For most cases, the default should be fine.   |  |

### Return type

[**models::PaginatedOAuth2ProviderList**](PaginatedOAuth2ProviderList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_oauth2_partial_update

> models::OAuth2Provider providers_oauth2_partial_update(id, patched_o_auth2_provider_request)


OAuth2Provider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this OAuth2/OpenID Provider. | [required] |
**patched_o_auth2_provider_request** | Option<[**PatchedOAuth2ProviderRequest**](PatchedOAuth2ProviderRequest.md)> |  |  |

### Return type

[**models::OAuth2Provider**](OAuth2Provider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_oauth2_preview_user_retrieve

> models::PropertyMappingPreview providers_oauth2_preview_user_retrieve(id, for_user)


Preview user data for provider

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this OAuth2/OpenID Provider. | [required] |
**for_user** | Option<**i32**> |  |  |

### Return type

[**models::PropertyMappingPreview**](PropertyMappingPreview.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_oauth2_retrieve

> models::OAuth2Provider providers_oauth2_retrieve(id)


OAuth2Provider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this OAuth2/OpenID Provider. | [required] |

### Return type

[**models::OAuth2Provider**](OAuth2Provider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_oauth2_setup_urls_retrieve

> models::OAuth2ProviderSetupUrls providers_oauth2_setup_urls_retrieve(id)


Get Providers setup URLs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this OAuth2/OpenID Provider. | [required] |

### Return type

[**models::OAuth2ProviderSetupUrls**](OAuth2ProviderSetupURLs.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_oauth2_update

> models::OAuth2Provider providers_oauth2_update(id, o_auth2_provider_request)


OAuth2Provider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this OAuth2/OpenID Provider. | [required] |
**o_auth2_provider_request** | [**OAuth2ProviderRequest**](OAuth2ProviderRequest.md) |  | [required] |

### Return type

[**models::OAuth2Provider**](OAuth2Provider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_oauth2_used_by_list

> Vec<models::UsedBy> providers_oauth2_used_by_list(id)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this OAuth2/OpenID Provider. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_proxy_create

> models::ProxyProvider providers_proxy_create(proxy_provider_request)


ProxyProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proxy_provider_request** | [**ProxyProviderRequest**](ProxyProviderRequest.md) |  | [required] |

### Return type

[**models::ProxyProvider**](ProxyProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_proxy_destroy

> providers_proxy_destroy(id)


ProxyProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Proxy Provider. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_proxy_list

> models::PaginatedProxyProviderList providers_proxy_list(application__isnull, authorization_flow__slug__iexact, basic_auth_enabled__iexact, basic_auth_password_attribute__iexact, basic_auth_user_attribute__iexact, certificate__kp_uuid__iexact, certificate__name__iexact, cookie_domain__iexact, external_host__iexact, internal_host__iexact, internal_host_ssl_validation__iexact, mode__iexact, name__iexact, ordering, page, page_size, property_mappings__iexact, search, skip_path_regex__iexact)


ProxyProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application__isnull** | Option<**bool**> |  |  |
**authorization_flow__slug__iexact** | Option<**String**> |  |  |
**basic_auth_enabled__iexact** | Option<**bool**> |  |  |
**basic_auth_password_attribute__iexact** | Option<**String**> |  |  |
**basic_auth_user_attribute__iexact** | Option<**String**> |  |  |
**certificate__kp_uuid__iexact** | Option<**uuid::Uuid**> |  |  |
**certificate__name__iexact** | Option<**String**> |  |  |
**cookie_domain__iexact** | Option<**String**> |  |  |
**external_host__iexact** | Option<**String**> |  |  |
**internal_host__iexact** | Option<**String**> |  |  |
**internal_host_ssl_validation__iexact** | Option<**bool**> |  |  |
**mode__iexact** | Option<**String**> |  |  |
**name__iexact** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**property_mappings__iexact** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  |  |
**search** | Option<**String**> | A search term. |  |
**skip_path_regex__iexact** | Option<**String**> |  |  |

### Return type

[**models::PaginatedProxyProviderList**](PaginatedProxyProviderList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_proxy_partial_update

> models::ProxyProvider providers_proxy_partial_update(id, patched_proxy_provider_request)


ProxyProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Proxy Provider. | [required] |
**patched_proxy_provider_request** | Option<[**PatchedProxyProviderRequest**](PatchedProxyProviderRequest.md)> |  |  |

### Return type

[**models::ProxyProvider**](ProxyProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_proxy_retrieve

> models::ProxyProvider providers_proxy_retrieve(id)


ProxyProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Proxy Provider. | [required] |

### Return type

[**models::ProxyProvider**](ProxyProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_proxy_update

> models::ProxyProvider providers_proxy_update(id, proxy_provider_request)


ProxyProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Proxy Provider. | [required] |
**proxy_provider_request** | [**ProxyProviderRequest**](ProxyProviderRequest.md) |  | [required] |

### Return type

[**models::ProxyProvider**](ProxyProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_proxy_used_by_list

> Vec<models::UsedBy> providers_proxy_used_by_list(id)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Proxy Provider. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_rac_create

> models::RacProvider providers_rac_create(rac_provider_request)


RACProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rac_provider_request** | [**RacProviderRequest**](RacProviderRequest.md) |  | [required] |

### Return type

[**models::RacProvider**](RACProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_rac_destroy

> providers_rac_destroy(id)


RACProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this RAC Provider. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_rac_list

> models::PaginatedRacProviderList providers_rac_list(application__isnull, name__iexact, ordering, page, page_size, search)


RACProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application__isnull** | Option<**bool**> |  |  |
**name__iexact** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedRacProviderList**](PaginatedRACProviderList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_rac_partial_update

> models::RacProvider providers_rac_partial_update(id, patched_rac_provider_request)


RACProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this RAC Provider. | [required] |
**patched_rac_provider_request** | Option<[**PatchedRacProviderRequest**](PatchedRacProviderRequest.md)> |  |  |

### Return type

[**models::RacProvider**](RACProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_rac_retrieve

> models::RacProvider providers_rac_retrieve(id)


RACProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this RAC Provider. | [required] |

### Return type

[**models::RacProvider**](RACProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_rac_update

> models::RacProvider providers_rac_update(id, rac_provider_request)


RACProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this RAC Provider. | [required] |
**rac_provider_request** | [**RacProviderRequest**](RacProviderRequest.md) |  | [required] |

### Return type

[**models::RacProvider**](RACProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_rac_used_by_list

> Vec<models::UsedBy> providers_rac_used_by_list(id)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this RAC Provider. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_radius_create

> models::RadiusProvider providers_radius_create(radius_provider_request)


RadiusProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**radius_provider_request** | [**RadiusProviderRequest**](RadiusProviderRequest.md) |  | [required] |

### Return type

[**models::RadiusProvider**](RadiusProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_radius_destroy

> providers_radius_destroy(id)


RadiusProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Radius Provider. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_radius_list

> models::PaginatedRadiusProviderList providers_radius_list(application__isnull, authorization_flow__slug__iexact, client_networks__iexact, name__iexact, ordering, page, page_size, search)


RadiusProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application__isnull** | Option<**bool**> |  |  |
**authorization_flow__slug__iexact** | Option<**String**> |  |  |
**client_networks__iexact** | Option<**String**> |  |  |
**name__iexact** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedRadiusProviderList**](PaginatedRadiusProviderList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_radius_partial_update

> models::RadiusProvider providers_radius_partial_update(id, patched_radius_provider_request)


RadiusProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Radius Provider. | [required] |
**patched_radius_provider_request** | Option<[**PatchedRadiusProviderRequest**](PatchedRadiusProviderRequest.md)> |  |  |

### Return type

[**models::RadiusProvider**](RadiusProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_radius_retrieve

> models::RadiusProvider providers_radius_retrieve(id)


RadiusProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Radius Provider. | [required] |

### Return type

[**models::RadiusProvider**](RadiusProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_radius_update

> models::RadiusProvider providers_radius_update(id, radius_provider_request)


RadiusProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Radius Provider. | [required] |
**radius_provider_request** | [**RadiusProviderRequest**](RadiusProviderRequest.md) |  | [required] |

### Return type

[**models::RadiusProvider**](RadiusProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_radius_used_by_list

> Vec<models::UsedBy> providers_radius_used_by_list(id)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Radius Provider. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_saml_create

> models::SamlProvider providers_saml_create(saml_provider_request)


SAMLProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**saml_provider_request** | [**SamlProviderRequest**](SamlProviderRequest.md) |  | [required] |

### Return type

[**models::SamlProvider**](SAMLProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_saml_destroy

> providers_saml_destroy(id)


SAMLProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this SAML Provider. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_saml_import_metadata_create

> models::SamlProvider providers_saml_import_metadata_create(name, authorization_flow, invalidation_flow, file)


Create provider from SAML Metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**authorization_flow** | **uuid::Uuid** |  | [required] |
**invalidation_flow** | **uuid::Uuid** |  | [required] |
**file** | **std::path::PathBuf** |  | [required] |

### Return type

[**models::SamlProvider**](SAMLProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_saml_list

> models::PaginatedSamlProviderList providers_saml_list(acs_url, assertion_valid_not_before, assertion_valid_not_on_or_after, audience, authentication_flow, authn_context_class_ref_mapping, authorization_flow, backchannel_application, default_name_id_policy, default_relay_state, digest_algorithm, encryption_kp, invalidation_flow, is_backchannel, issuer, logout_method, name, name_id_mapping, ordering, page, page_size, property_mappings, search, session_valid_not_on_or_after, sign_assertion, sign_logout_request, sign_response, signature_algorithm, signing_kp, sls_binding, sls_url, sp_binding, verification_kp)


SAMLProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**acs_url** | Option<**String**> |  |  |
**assertion_valid_not_before** | Option<**String**> |  |  |
**assertion_valid_not_on_or_after** | Option<**String**> |  |  |
**audience** | Option<**String**> |  |  |
**authentication_flow** | Option<**uuid::Uuid**> |  |  |
**authn_context_class_ref_mapping** | Option<**uuid::Uuid**> |  |  |
**authorization_flow** | Option<**uuid::Uuid**> |  |  |
**backchannel_application** | Option<**uuid::Uuid**> |  |  |
**default_name_id_policy** | Option<**String**> |  |  |
**default_relay_state** | Option<**String**> |  |  |
**digest_algorithm** | Option<**String**> |  |  |
**encryption_kp** | Option<**uuid::Uuid**> |  |  |
**invalidation_flow** | Option<**uuid::Uuid**> |  |  |
**is_backchannel** | Option<**bool**> |  |  |
**issuer** | Option<**String**> |  |  |
**logout_method** | Option<**String**> | Method to use for logout. Front-channel iframe loads all logout URLs simultaneously in hidden iframes. Front-channel native uses your active browser tab to send post requests and redirect to providers. Back-channel sends logout requests directly from the server without user interaction (requires POST SLS binding).   |  |
**name** | Option<**String**> |  |  |
**name_id_mapping** | Option<**uuid::Uuid**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**property_mappings** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  |  |
**search** | Option<**String**> | A search term. |  |
**session_valid_not_on_or_after** | Option<**String**> |  |  |
**sign_assertion** | Option<**bool**> |  |  |
**sign_logout_request** | Option<**bool**> |  |  |
**sign_response** | Option<**bool**> |  |  |
**signature_algorithm** | Option<**String**> |  |  |
**signing_kp** | Option<**uuid::Uuid**> |  |  |
**sls_binding** | Option<**String**> | This determines how authentik sends the logout response back to the Service Provider.   |  |
**sls_url** | Option<**String**> |  |  |
**sp_binding** | Option<**String**> | This determines how authentik sends the response back to the Service Provider.   |  |
**verification_kp** | Option<**uuid::Uuid**> |  |  |

### Return type

[**models::PaginatedSamlProviderList**](PaginatedSAMLProviderList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_saml_metadata_retrieve

> models::SamlMetadata providers_saml_metadata_retrieve(id, download, force_binding)


Return metadata as XML string

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this SAML Provider. | [required] |
**download** | Option<**bool**> |  |  |
**force_binding** | Option<**String**> | Optionally force the metadata to only include one binding. |  |

### Return type

[**models::SamlMetadata**](SAMLMetadata.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_saml_partial_update

> models::SamlProvider providers_saml_partial_update(id, patched_saml_provider_request)


SAMLProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this SAML Provider. | [required] |
**patched_saml_provider_request** | Option<[**PatchedSamlProviderRequest**](PatchedSamlProviderRequest.md)> |  |  |

### Return type

[**models::SamlProvider**](SAMLProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_saml_preview_user_retrieve

> models::PropertyMappingPreview providers_saml_preview_user_retrieve(id, for_user)


Preview user data for provider

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this SAML Provider. | [required] |
**for_user** | Option<**i32**> |  |  |

### Return type

[**models::PropertyMappingPreview**](PropertyMappingPreview.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_saml_retrieve

> models::SamlProvider providers_saml_retrieve(id)


SAMLProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this SAML Provider. | [required] |

### Return type

[**models::SamlProvider**](SAMLProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_saml_update

> models::SamlProvider providers_saml_update(id, saml_provider_request)


SAMLProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this SAML Provider. | [required] |
**saml_provider_request** | [**SamlProviderRequest**](SamlProviderRequest.md) |  | [required] |

### Return type

[**models::SamlProvider**](SAMLProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_saml_used_by_list

> Vec<models::UsedBy> providers_saml_used_by_list(id)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this SAML Provider. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_scim_create

> models::ScimProvider providers_scim_create(scim_provider_request)


SCIMProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scim_provider_request** | [**ScimProviderRequest**](ScimProviderRequest.md) |  | [required] |

### Return type

[**models::ScimProvider**](SCIMProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_scim_destroy

> providers_scim_destroy(id)


SCIMProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this SCIM Provider. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_scim_groups_create

> models::ScimProviderGroup providers_scim_groups_create(scim_provider_group_request)


SCIMProviderGroup Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scim_provider_group_request** | [**ScimProviderGroupRequest**](ScimProviderGroupRequest.md) |  | [required] |

### Return type

[**models::ScimProviderGroup**](SCIMProviderGroup.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_scim_groups_destroy

> providers_scim_groups_destroy(id)


SCIMProviderGroup Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this scim provider group. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_scim_groups_list

> models::PaginatedScimProviderGroupList providers_scim_groups_list(group__group_uuid, group__name, ordering, page, page_size, provider__id, search)


SCIMProviderGroup Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group__group_uuid** | Option<**uuid::Uuid**> |  |  |
**group__name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**provider__id** | Option<**i32**> |  |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedScimProviderGroupList**](PaginatedSCIMProviderGroupList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_scim_groups_retrieve

> models::ScimProviderGroup providers_scim_groups_retrieve(id)


SCIMProviderGroup Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this scim provider group. | [required] |

### Return type

[**models::ScimProviderGroup**](SCIMProviderGroup.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_scim_groups_used_by_list

> Vec<models::UsedBy> providers_scim_groups_used_by_list(id)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this scim provider group. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_scim_list

> models::PaginatedScimProviderList providers_scim_list(exclude_users_service_account, filter_group, name, ordering, page, page_size, search, url)


SCIMProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exclude_users_service_account** | Option<**bool**> |  |  |
**filter_group** | Option<**uuid::Uuid**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**url** | Option<**String**> |  |  |

### Return type

[**models::PaginatedScimProviderList**](PaginatedSCIMProviderList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_scim_partial_update

> models::ScimProvider providers_scim_partial_update(id, patched_scim_provider_request)


SCIMProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this SCIM Provider. | [required] |
**patched_scim_provider_request** | Option<[**PatchedScimProviderRequest**](PatchedScimProviderRequest.md)> |  |  |

### Return type

[**models::ScimProvider**](SCIMProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_scim_retrieve

> models::ScimProvider providers_scim_retrieve(id)


SCIMProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this SCIM Provider. | [required] |

### Return type

[**models::ScimProvider**](SCIMProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_scim_sync_object_create

> models::SyncObjectResult providers_scim_sync_object_create(id, sync_object_request)


Sync/Re-sync a single user/group object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this SCIM Provider. | [required] |
**sync_object_request** | [**SyncObjectRequest**](SyncObjectRequest.md) |  | [required] |

### Return type

[**models::SyncObjectResult**](SyncObjectResult.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_scim_sync_status_retrieve

> models::SyncStatus providers_scim_sync_status_retrieve(id)


Get provider's sync status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this SCIM Provider. | [required] |

### Return type

[**models::SyncStatus**](SyncStatus.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_scim_update

> models::ScimProvider providers_scim_update(id, scim_provider_request)


SCIMProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this SCIM Provider. | [required] |
**scim_provider_request** | [**ScimProviderRequest**](ScimProviderRequest.md) |  | [required] |

### Return type

[**models::ScimProvider**](SCIMProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_scim_used_by_list

> Vec<models::UsedBy> providers_scim_used_by_list(id)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this SCIM Provider. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_scim_users_create

> models::ScimProviderUser providers_scim_users_create(scim_provider_user_request)


SCIMProviderUser Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scim_provider_user_request** | [**ScimProviderUserRequest**](ScimProviderUserRequest.md) |  | [required] |

### Return type

[**models::ScimProviderUser**](SCIMProviderUser.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_scim_users_destroy

> providers_scim_users_destroy(id)


SCIMProviderUser Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this scim provider user. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_scim_users_list

> models::PaginatedScimProviderUserList providers_scim_users_list(ordering, page, page_size, provider__id, search, user__id, user__username)


SCIMProviderUser Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**provider__id** | Option<**i32**> |  |  |
**search** | Option<**String**> | A search term. |  |
**user__id** | Option<**i32**> |  |  |
**user__username** | Option<**String**> |  |  |

### Return type

[**models::PaginatedScimProviderUserList**](PaginatedSCIMProviderUserList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_scim_users_retrieve

> models::ScimProviderUser providers_scim_users_retrieve(id)


SCIMProviderUser Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this scim provider user. | [required] |

### Return type

[**models::ScimProviderUser**](SCIMProviderUser.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_scim_users_used_by_list

> Vec<models::UsedBy> providers_scim_users_used_by_list(id)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this scim provider user. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_ssf_create

> models::SsfProvider providers_ssf_create(ssf_provider_request)


SSFProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ssf_provider_request** | [**SsfProviderRequest**](SsfProviderRequest.md) |  | [required] |

### Return type

[**models::SsfProvider**](SSFProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_ssf_destroy

> providers_ssf_destroy(id)


SSFProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Shared Signals Framework Provider. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_ssf_list

> models::PaginatedSsfProviderList providers_ssf_list(application__isnull, name__iexact, ordering, page, page_size, search)


SSFProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application__isnull** | Option<**bool**> |  |  |
**name__iexact** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedSsfProviderList**](PaginatedSSFProviderList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_ssf_partial_update

> models::SsfProvider providers_ssf_partial_update(id, patched_ssf_provider_request)


SSFProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Shared Signals Framework Provider. | [required] |
**patched_ssf_provider_request** | Option<[**PatchedSsfProviderRequest**](PatchedSsfProviderRequest.md)> |  |  |

### Return type

[**models::SsfProvider**](SSFProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_ssf_retrieve

> models::SsfProvider providers_ssf_retrieve(id)


SSFProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Shared Signals Framework Provider. | [required] |

### Return type

[**models::SsfProvider**](SSFProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_ssf_update

> models::SsfProvider providers_ssf_update(id, ssf_provider_request)


SSFProvider Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Shared Signals Framework Provider. | [required] |
**ssf_provider_request** | [**SsfProviderRequest**](SsfProviderRequest.md) |  | [required] |

### Return type

[**models::SsfProvider**](SSFProvider.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_ssf_used_by_list

> Vec<models::UsedBy> providers_ssf_used_by_list(id)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Shared Signals Framework Provider. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

