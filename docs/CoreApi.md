# \CoreApi

All URIs are relative to */api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**core_application_entitlements_create**](CoreApi.md#core_application_entitlements_create) | **POST** /core/application_entitlements/ | 
[**core_application_entitlements_destroy**](CoreApi.md#core_application_entitlements_destroy) | **DELETE** /core/application_entitlements/{pbm_uuid}/ | 
[**core_application_entitlements_list**](CoreApi.md#core_application_entitlements_list) | **GET** /core/application_entitlements/ | 
[**core_application_entitlements_partial_update**](CoreApi.md#core_application_entitlements_partial_update) | **PATCH** /core/application_entitlements/{pbm_uuid}/ | 
[**core_application_entitlements_retrieve**](CoreApi.md#core_application_entitlements_retrieve) | **GET** /core/application_entitlements/{pbm_uuid}/ | 
[**core_application_entitlements_update**](CoreApi.md#core_application_entitlements_update) | **PUT** /core/application_entitlements/{pbm_uuid}/ | 
[**core_application_entitlements_used_by_list**](CoreApi.md#core_application_entitlements_used_by_list) | **GET** /core/application_entitlements/{pbm_uuid}/used_by/ | 
[**core_applications_check_access_retrieve**](CoreApi.md#core_applications_check_access_retrieve) | **GET** /core/applications/{slug}/check_access/ | 
[**core_applications_create**](CoreApi.md#core_applications_create) | **POST** /core/applications/ | 
[**core_applications_destroy**](CoreApi.md#core_applications_destroy) | **DELETE** /core/applications/{slug}/ | 
[**core_applications_list**](CoreApi.md#core_applications_list) | **GET** /core/applications/ | 
[**core_applications_partial_update**](CoreApi.md#core_applications_partial_update) | **PATCH** /core/applications/{slug}/ | 
[**core_applications_retrieve**](CoreApi.md#core_applications_retrieve) | **GET** /core/applications/{slug}/ | 
[**core_applications_update**](CoreApi.md#core_applications_update) | **PUT** /core/applications/{slug}/ | 
[**core_applications_used_by_list**](CoreApi.md#core_applications_used_by_list) | **GET** /core/applications/{slug}/used_by/ | 
[**core_authenticated_sessions_destroy**](CoreApi.md#core_authenticated_sessions_destroy) | **DELETE** /core/authenticated_sessions/{uuid}/ | 
[**core_authenticated_sessions_list**](CoreApi.md#core_authenticated_sessions_list) | **GET** /core/authenticated_sessions/ | 
[**core_authenticated_sessions_retrieve**](CoreApi.md#core_authenticated_sessions_retrieve) | **GET** /core/authenticated_sessions/{uuid}/ | 
[**core_authenticated_sessions_used_by_list**](CoreApi.md#core_authenticated_sessions_used_by_list) | **GET** /core/authenticated_sessions/{uuid}/used_by/ | 
[**core_brands_create**](CoreApi.md#core_brands_create) | **POST** /core/brands/ | 
[**core_brands_current_retrieve**](CoreApi.md#core_brands_current_retrieve) | **GET** /core/brands/current/ | 
[**core_brands_destroy**](CoreApi.md#core_brands_destroy) | **DELETE** /core/brands/{brand_uuid}/ | 
[**core_brands_list**](CoreApi.md#core_brands_list) | **GET** /core/brands/ | 
[**core_brands_partial_update**](CoreApi.md#core_brands_partial_update) | **PATCH** /core/brands/{brand_uuid}/ | 
[**core_brands_retrieve**](CoreApi.md#core_brands_retrieve) | **GET** /core/brands/{brand_uuid}/ | 
[**core_brands_update**](CoreApi.md#core_brands_update) | **PUT** /core/brands/{brand_uuid}/ | 
[**core_brands_used_by_list**](CoreApi.md#core_brands_used_by_list) | **GET** /core/brands/{brand_uuid}/used_by/ | 
[**core_groups_add_user_create**](CoreApi.md#core_groups_add_user_create) | **POST** /core/groups/{group_uuid}/add_user/ | 
[**core_groups_create**](CoreApi.md#core_groups_create) | **POST** /core/groups/ | 
[**core_groups_destroy**](CoreApi.md#core_groups_destroy) | **DELETE** /core/groups/{group_uuid}/ | 
[**core_groups_list**](CoreApi.md#core_groups_list) | **GET** /core/groups/ | 
[**core_groups_partial_update**](CoreApi.md#core_groups_partial_update) | **PATCH** /core/groups/{group_uuid}/ | 
[**core_groups_remove_user_create**](CoreApi.md#core_groups_remove_user_create) | **POST** /core/groups/{group_uuid}/remove_user/ | 
[**core_groups_retrieve**](CoreApi.md#core_groups_retrieve) | **GET** /core/groups/{group_uuid}/ | 
[**core_groups_update**](CoreApi.md#core_groups_update) | **PUT** /core/groups/{group_uuid}/ | 
[**core_groups_used_by_list**](CoreApi.md#core_groups_used_by_list) | **GET** /core/groups/{group_uuid}/used_by/ | 
[**core_tokens_create**](CoreApi.md#core_tokens_create) | **POST** /core/tokens/ | 
[**core_tokens_destroy**](CoreApi.md#core_tokens_destroy) | **DELETE** /core/tokens/{identifier}/ | 
[**core_tokens_list**](CoreApi.md#core_tokens_list) | **GET** /core/tokens/ | 
[**core_tokens_partial_update**](CoreApi.md#core_tokens_partial_update) | **PATCH** /core/tokens/{identifier}/ | 
[**core_tokens_retrieve**](CoreApi.md#core_tokens_retrieve) | **GET** /core/tokens/{identifier}/ | 
[**core_tokens_set_key_create**](CoreApi.md#core_tokens_set_key_create) | **POST** /core/tokens/{identifier}/set_key/ | 
[**core_tokens_update**](CoreApi.md#core_tokens_update) | **PUT** /core/tokens/{identifier}/ | 
[**core_tokens_used_by_list**](CoreApi.md#core_tokens_used_by_list) | **GET** /core/tokens/{identifier}/used_by/ | 
[**core_tokens_view_key_retrieve**](CoreApi.md#core_tokens_view_key_retrieve) | **GET** /core/tokens/{identifier}/view_key/ | 
[**core_transactional_applications_update**](CoreApi.md#core_transactional_applications_update) | **PUT** /core/transactional/applications/ | 
[**core_user_consent_destroy**](CoreApi.md#core_user_consent_destroy) | **DELETE** /core/user_consent/{id}/ | 
[**core_user_consent_list**](CoreApi.md#core_user_consent_list) | **GET** /core/user_consent/ | 
[**core_user_consent_retrieve**](CoreApi.md#core_user_consent_retrieve) | **GET** /core/user_consent/{id}/ | 
[**core_user_consent_used_by_list**](CoreApi.md#core_user_consent_used_by_list) | **GET** /core/user_consent/{id}/used_by/ | 
[**core_users_create**](CoreApi.md#core_users_create) | **POST** /core/users/ | 
[**core_users_destroy**](CoreApi.md#core_users_destroy) | **DELETE** /core/users/{id}/ | 
[**core_users_export_create**](CoreApi.md#core_users_export_create) | **POST** /core/users/export/ | 
[**core_users_impersonate_create**](CoreApi.md#core_users_impersonate_create) | **POST** /core/users/{id}/impersonate/ | 
[**core_users_impersonate_end_retrieve**](CoreApi.md#core_users_impersonate_end_retrieve) | **GET** /core/users/impersonate_end/ | 
[**core_users_list**](CoreApi.md#core_users_list) | **GET** /core/users/ | 
[**core_users_me_retrieve**](CoreApi.md#core_users_me_retrieve) | **GET** /core/users/me/ | 
[**core_users_partial_update**](CoreApi.md#core_users_partial_update) | **PATCH** /core/users/{id}/ | 
[**core_users_paths_retrieve**](CoreApi.md#core_users_paths_retrieve) | **GET** /core/users/paths/ | 
[**core_users_recovery_create**](CoreApi.md#core_users_recovery_create) | **POST** /core/users/{id}/recovery/ | 
[**core_users_recovery_email_create**](CoreApi.md#core_users_recovery_email_create) | **POST** /core/users/{id}/recovery_email/ | 
[**core_users_retrieve**](CoreApi.md#core_users_retrieve) | **GET** /core/users/{id}/ | 
[**core_users_service_account_create**](CoreApi.md#core_users_service_account_create) | **POST** /core/users/service_account/ | 
[**core_users_set_password_create**](CoreApi.md#core_users_set_password_create) | **POST** /core/users/{id}/set_password/ | 
[**core_users_update**](CoreApi.md#core_users_update) | **PUT** /core/users/{id}/ | 
[**core_users_used_by_list**](CoreApi.md#core_users_used_by_list) | **GET** /core/users/{id}/used_by/ | 



## core_application_entitlements_create

> models::ApplicationEntitlement core_application_entitlements_create(application_entitlement_request)


ApplicationEntitlement Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_entitlement_request** | [**ApplicationEntitlementRequest**](ApplicationEntitlementRequest.md) |  | [required] |

### Return type

[**models::ApplicationEntitlement**](ApplicationEntitlement.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_application_entitlements_destroy

> core_application_entitlements_destroy(pbm_uuid)


ApplicationEntitlement Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pbm_uuid** | **uuid::Uuid** | A UUID string identifying this Application Entitlement. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_application_entitlements_list

> models::PaginatedApplicationEntitlementList core_application_entitlements_list(app, name, ordering, page, page_size, pbm_uuid, search)


ApplicationEntitlement Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app** | Option<**uuid::Uuid**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**pbm_uuid** | Option<**uuid::Uuid**> |  |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedApplicationEntitlementList**](PaginatedApplicationEntitlementList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_application_entitlements_partial_update

> models::ApplicationEntitlement core_application_entitlements_partial_update(pbm_uuid, patched_application_entitlement_request)


ApplicationEntitlement Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pbm_uuid** | **uuid::Uuid** | A UUID string identifying this Application Entitlement. | [required] |
**patched_application_entitlement_request** | Option<[**PatchedApplicationEntitlementRequest**](PatchedApplicationEntitlementRequest.md)> |  |  |

### Return type

[**models::ApplicationEntitlement**](ApplicationEntitlement.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_application_entitlements_retrieve

> models::ApplicationEntitlement core_application_entitlements_retrieve(pbm_uuid)


ApplicationEntitlement Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pbm_uuid** | **uuid::Uuid** | A UUID string identifying this Application Entitlement. | [required] |

### Return type

[**models::ApplicationEntitlement**](ApplicationEntitlement.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_application_entitlements_update

> models::ApplicationEntitlement core_application_entitlements_update(pbm_uuid, application_entitlement_request)


ApplicationEntitlement Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pbm_uuid** | **uuid::Uuid** | A UUID string identifying this Application Entitlement. | [required] |
**application_entitlement_request** | [**ApplicationEntitlementRequest**](ApplicationEntitlementRequest.md) |  | [required] |

### Return type

[**models::ApplicationEntitlement**](ApplicationEntitlement.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_application_entitlements_used_by_list

> Vec<models::UsedBy> core_application_entitlements_used_by_list(pbm_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pbm_uuid** | **uuid::Uuid** | A UUID string identifying this Application Entitlement. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_applications_check_access_retrieve

> models::PolicyTestResult core_applications_check_access_retrieve(slug, for_user)


Check access to a single application by slug

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**for_user** | Option<**i32**> |  |  |

### Return type

[**models::PolicyTestResult**](PolicyTestResult.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_applications_create

> models::Application core_applications_create(application_request)


Application Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_request** | [**ApplicationRequest**](ApplicationRequest.md) |  | [required] |

### Return type

[**models::Application**](Application.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_applications_destroy

> core_applications_destroy(slug)


Application Viewset

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


## core_applications_list

> models::PaginatedApplicationList core_applications_list(for_user, group, meta_description, meta_launch_url, meta_publisher, name, only_with_launch_url, ordering, page, page_size, search, slug, superuser_full_list)


Custom list method that checks Policy based access instead of guardian

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**for_user** | Option<**i32**> |  |  |
**group** | Option<**String**> |  |  |
**meta_description** | Option<**String**> |  |  |
**meta_launch_url** | Option<**String**> |  |  |
**meta_publisher** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**only_with_launch_url** | Option<**bool**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**slug** | Option<**String**> |  |  |
**superuser_full_list** | Option<**bool**> |  |  |

### Return type

[**models::PaginatedApplicationList**](PaginatedApplicationList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_applications_partial_update

> models::Application core_applications_partial_update(slug, patched_application_request)


Application Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**patched_application_request** | Option<[**PatchedApplicationRequest**](PatchedApplicationRequest.md)> |  |  |

### Return type

[**models::Application**](Application.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_applications_retrieve

> models::Application core_applications_retrieve(slug)


Application Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

[**models::Application**](Application.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_applications_update

> models::Application core_applications_update(slug, application_request)


Application Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**application_request** | [**ApplicationRequest**](ApplicationRequest.md) |  | [required] |

### Return type

[**models::Application**](Application.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_applications_used_by_list

> Vec<models::UsedBy> core_applications_used_by_list(slug)


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


## core_authenticated_sessions_destroy

> core_authenticated_sessions_destroy(uuid)


AuthenticatedSession Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_authenticated_sessions_list

> models::PaginatedAuthenticatedSessionList core_authenticated_sessions_list(ordering, page, page_size, search, session__last_ip, session__last_user_agent, user__username)


AuthenticatedSession Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**session__last_ip** | Option<**String**> |  |  |
**session__last_user_agent** | Option<**String**> |  |  |
**user__username** | Option<**String**> |  |  |

### Return type

[**models::PaginatedAuthenticatedSessionList**](PaginatedAuthenticatedSessionList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_authenticated_sessions_retrieve

> models::AuthenticatedSession core_authenticated_sessions_retrieve(uuid)


AuthenticatedSession Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** |  | [required] |

### Return type

[**models::AuthenticatedSession**](AuthenticatedSession.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_authenticated_sessions_used_by_list

> Vec<models::UsedBy> core_authenticated_sessions_used_by_list(uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_brands_create

> models::Brand core_brands_create(brand_request)


Brand Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**brand_request** | [**BrandRequest**](BrandRequest.md) |  | [required] |

### Return type

[**models::Brand**](Brand.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_brands_current_retrieve

> models::CurrentBrand core_brands_current_retrieve()


Get current brand

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CurrentBrand**](CurrentBrand.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_brands_destroy

> core_brands_destroy(brand_uuid)


Brand Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**brand_uuid** | **uuid::Uuid** | A UUID string identifying this Brand. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_brands_list

> models::PaginatedBrandList core_brands_list(brand_uuid, branding_default_flow_background, branding_favicon, branding_logo, branding_title, client_certificates, default, domain, flow_authentication, flow_device_code, flow_invalidation, flow_recovery, flow_unenrollment, flow_user_settings, ordering, page, page_size, search, web_certificate)


Brand Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**brand_uuid** | Option<**uuid::Uuid**> |  |  |
**branding_default_flow_background** | Option<**String**> |  |  |
**branding_favicon** | Option<**String**> |  |  |
**branding_logo** | Option<**String**> |  |  |
**branding_title** | Option<**String**> |  |  |
**client_certificates** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  |  |
**default** | Option<**bool**> |  |  |
**domain** | Option<**String**> |  |  |
**flow_authentication** | Option<**uuid::Uuid**> |  |  |
**flow_device_code** | Option<**uuid::Uuid**> |  |  |
**flow_invalidation** | Option<**uuid::Uuid**> |  |  |
**flow_recovery** | Option<**uuid::Uuid**> |  |  |
**flow_unenrollment** | Option<**uuid::Uuid**> |  |  |
**flow_user_settings** | Option<**uuid::Uuid**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**web_certificate** | Option<**uuid::Uuid**> |  |  |

### Return type

[**models::PaginatedBrandList**](PaginatedBrandList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_brands_partial_update

> models::Brand core_brands_partial_update(brand_uuid, patched_brand_request)


Brand Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**brand_uuid** | **uuid::Uuid** | A UUID string identifying this Brand. | [required] |
**patched_brand_request** | Option<[**PatchedBrandRequest**](PatchedBrandRequest.md)> |  |  |

### Return type

[**models::Brand**](Brand.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_brands_retrieve

> models::Brand core_brands_retrieve(brand_uuid)


Brand Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**brand_uuid** | **uuid::Uuid** | A UUID string identifying this Brand. | [required] |

### Return type

[**models::Brand**](Brand.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_brands_update

> models::Brand core_brands_update(brand_uuid, brand_request)


Brand Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**brand_uuid** | **uuid::Uuid** | A UUID string identifying this Brand. | [required] |
**brand_request** | [**BrandRequest**](BrandRequest.md) |  | [required] |

### Return type

[**models::Brand**](Brand.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_brands_used_by_list

> Vec<models::UsedBy> core_brands_used_by_list(brand_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**brand_uuid** | **uuid::Uuid** | A UUID string identifying this Brand. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_groups_add_user_create

> core_groups_add_user_create(group_uuid, user_account_request)


Add user to group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_uuid** | **uuid::Uuid** | A UUID string identifying this Group. | [required] |
**user_account_request** | [**UserAccountRequest**](UserAccountRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_groups_create

> models::Group core_groups_create(group_request)


Group Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_request** | [**GroupRequest**](GroupRequest.md) |  | [required] |

### Return type

[**models::Group**](Group.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_groups_destroy

> core_groups_destroy(group_uuid)


Group Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_uuid** | **uuid::Uuid** | A UUID string identifying this Group. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_groups_list

> models::PaginatedGroupList core_groups_list(attributes, include_children, include_inherited_roles, include_parents, include_users, is_superuser, members_by_pk, members_by_username, name, ordering, page, page_size, search)


Group Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attributes** | Option<**String**> | Attributes |  |
**include_children** | Option<**bool**> |  |  |[default to false]
**include_inherited_roles** | Option<**bool**> |  |  |[default to false]
**include_parents** | Option<**bool**> |  |  |[default to false]
**include_users** | Option<**bool**> |  |  |[default to true]
**is_superuser** | Option<**bool**> |  |  |
**members_by_pk** | Option<[**Vec<i32>**](i32.md)> |  |  |
**members_by_username** | Option<[**Vec<String>**](String.md)> | Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only. |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedGroupList**](PaginatedGroupList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_groups_partial_update

> models::Group core_groups_partial_update(group_uuid, patched_group_request)


Group Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_uuid** | **uuid::Uuid** | A UUID string identifying this Group. | [required] |
**patched_group_request** | Option<[**PatchedGroupRequest**](PatchedGroupRequest.md)> |  |  |

### Return type

[**models::Group**](Group.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_groups_remove_user_create

> core_groups_remove_user_create(group_uuid, user_account_request)


Remove user from group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_uuid** | **uuid::Uuid** | A UUID string identifying this Group. | [required] |
**user_account_request** | [**UserAccountRequest**](UserAccountRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_groups_retrieve

> models::Group core_groups_retrieve(group_uuid, include_children, include_inherited_roles, include_parents, include_users)


Group Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_uuid** | **uuid::Uuid** | A UUID string identifying this Group. | [required] |
**include_children** | Option<**bool**> |  |  |[default to false]
**include_inherited_roles** | Option<**bool**> |  |  |[default to false]
**include_parents** | Option<**bool**> |  |  |[default to false]
**include_users** | Option<**bool**> |  |  |[default to true]

### Return type

[**models::Group**](Group.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_groups_update

> models::Group core_groups_update(group_uuid, group_request)


Group Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_uuid** | **uuid::Uuid** | A UUID string identifying this Group. | [required] |
**group_request** | [**GroupRequest**](GroupRequest.md) |  | [required] |

### Return type

[**models::Group**](Group.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_groups_used_by_list

> Vec<models::UsedBy> core_groups_used_by_list(group_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_uuid** | **uuid::Uuid** | A UUID string identifying this Group. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_tokens_create

> models::Token core_tokens_create(token_request)


Token Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_request** | [**TokenRequest**](TokenRequest.md) |  | [required] |

### Return type

[**models::Token**](Token.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_tokens_destroy

> core_tokens_destroy(identifier)


Token Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_tokens_list

> models::PaginatedTokenList core_tokens_list(description, expires, expiring, identifier, intent, managed, ordering, page, page_size, search, user__username)


Token Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**description** | Option<**String**> |  |  |
**expires** | Option<**String**> |  |  |
**expiring** | Option<**bool**> |  |  |
**identifier** | Option<**String**> |  |  |
**intent** | Option<**String**> |  |  |
**managed** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**user__username** | Option<**String**> |  |  |

### Return type

[**models::PaginatedTokenList**](PaginatedTokenList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_tokens_partial_update

> models::Token core_tokens_partial_update(identifier, patched_token_request)


Token Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** |  | [required] |
**patched_token_request** | Option<[**PatchedTokenRequest**](PatchedTokenRequest.md)> |  |  |

### Return type

[**models::Token**](Token.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_tokens_retrieve

> models::Token core_tokens_retrieve(identifier)


Token Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** |  | [required] |

### Return type

[**models::Token**](Token.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_tokens_set_key_create

> core_tokens_set_key_create(identifier, token_set_key_request)


Set token key. Action is logged as event. `authentik_core.set_token_key` permission is required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** |  | [required] |
**token_set_key_request** | [**TokenSetKeyRequest**](TokenSetKeyRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_tokens_update

> models::Token core_tokens_update(identifier, token_request)


Token Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** |  | [required] |
**token_request** | [**TokenRequest**](TokenRequest.md) |  | [required] |

### Return type

[**models::Token**](Token.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_tokens_used_by_list

> Vec<models::UsedBy> core_tokens_used_by_list(identifier)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** |  | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_tokens_view_key_retrieve

> models::TokenView core_tokens_view_key_retrieve(identifier)


Return token key and log access

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** |  | [required] |

### Return type

[**models::TokenView**](TokenView.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_transactional_applications_update

> models::TransactionApplicationResponse core_transactional_applications_update(transaction_application_request)


Convert data into a blueprint, validate it and apply it

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_application_request** | [**TransactionApplicationRequest**](TransactionApplicationRequest.md) |  | [required] |

### Return type

[**models::TransactionApplicationResponse**](TransactionApplicationResponse.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_user_consent_destroy

> core_user_consent_destroy(id)


UserConsent Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this User Consent. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_user_consent_list

> models::PaginatedUserConsentList core_user_consent_list(application, ordering, page, page_size, search, user)


UserConsent Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application** | Option<**uuid::Uuid**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**user** | Option<**i32**> |  |  |

### Return type

[**models::PaginatedUserConsentList**](PaginatedUserConsentList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_user_consent_retrieve

> models::UserConsent core_user_consent_retrieve(id)


UserConsent Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this User Consent. | [required] |

### Return type

[**models::UserConsent**](UserConsent.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_user_consent_used_by_list

> Vec<models::UsedBy> core_user_consent_used_by_list(id)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this User Consent. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_users_create

> models::User core_users_create(user_request)


User Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_request** | [**UserRequest**](UserRequest.md) |  | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_users_destroy

> core_users_destroy(id)


User Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this User. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_users_export_create

> models::DataExport core_users_export_create(attributes, date_joined, date_joined__gt, date_joined__lt, email, groups_by_name, groups_by_pk, is_active, is_superuser, last_updated, last_updated__gt, last_updated__lt, name, ordering, path, path_startswith, roles_by_name, roles_by_pk, search, r#type, username, uuid)


Create a data export for this data type. Note that the export is generated asynchronously: this method returns a `DataExport` object that will initially have `completed=false` as well as the permanent URL to that object in the `Location` header. You can poll that URL until `completed=true`, at which point the `file_url` property will contain a URL to download

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attributes** | Option<**String**> | Attributes |  |
**date_joined** | Option<**String**> |  |  |
**date_joined__gt** | Option<**String**> |  |  |
**date_joined__lt** | Option<**String**> |  |  |
**email** | Option<**String**> |  |  |
**groups_by_name** | Option<[**Vec<String>**](String.md)> |  |  |
**groups_by_pk** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  |  |
**is_active** | Option<**bool**> |  |  |
**is_superuser** | Option<**bool**> |  |  |
**last_updated** | Option<**String**> |  |  |
**last_updated__gt** | Option<**String**> |  |  |
**last_updated__lt** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**path** | Option<**String**> |  |  |
**path_startswith** | Option<**String**> |  |  |
**roles_by_name** | Option<[**Vec<String>**](String.md)> |  |  |
**roles_by_pk** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  |  |
**search** | Option<**String**> | A search term. |  |
**r#type** | Option<[**Vec<String>**](String.md)> |  |  |
**username** | Option<**String**> |  |  |
**uuid** | Option<**uuid::Uuid**> |  |  |

### Return type

[**models::DataExport**](DataExport.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_users_impersonate_create

> core_users_impersonate_create(id, impersonation_request)


Impersonate a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this User. | [required] |
**impersonation_request** | [**ImpersonationRequest**](ImpersonationRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_users_impersonate_end_retrieve

> core_users_impersonate_end_retrieve()


End Impersonation a user

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


## core_users_list

> models::PaginatedUserList core_users_list(attributes, date_joined, date_joined__gt, date_joined__lt, email, groups_by_name, groups_by_pk, include_groups, include_roles, is_active, is_superuser, last_updated, last_updated__gt, last_updated__lt, name, ordering, page, page_size, path, path_startswith, roles_by_name, roles_by_pk, search, r#type, username, uuid)


User Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attributes** | Option<**String**> | Attributes |  |
**date_joined** | Option<**String**> |  |  |
**date_joined__gt** | Option<**String**> |  |  |
**date_joined__lt** | Option<**String**> |  |  |
**email** | Option<**String**> |  |  |
**groups_by_name** | Option<[**Vec<String>**](String.md)> |  |  |
**groups_by_pk** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  |  |
**include_groups** | Option<**bool**> |  |  |[default to true]
**include_roles** | Option<**bool**> |  |  |[default to true]
**is_active** | Option<**bool**> |  |  |
**is_superuser** | Option<**bool**> |  |  |
**last_updated** | Option<**String**> |  |  |
**last_updated__gt** | Option<**String**> |  |  |
**last_updated__lt** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**path** | Option<**String**> |  |  |
**path_startswith** | Option<**String**> |  |  |
**roles_by_name** | Option<[**Vec<String>**](String.md)> |  |  |
**roles_by_pk** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  |  |
**search** | Option<**String**> | A search term. |  |
**r#type** | Option<[**Vec<String>**](String.md)> |  |  |
**username** | Option<**String**> |  |  |
**uuid** | Option<**uuid::Uuid**> |  |  |

### Return type

[**models::PaginatedUserList**](PaginatedUserList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_users_me_retrieve

> models::SessionUser core_users_me_retrieve()


Get information about current user

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SessionUser**](SessionUser.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_users_partial_update

> models::User core_users_partial_update(id, patched_user_request)


User Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this User. | [required] |
**patched_user_request** | Option<[**PatchedUserRequest**](PatchedUserRequest.md)> |  |  |

### Return type

[**models::User**](User.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_users_paths_retrieve

> models::UserPath core_users_paths_retrieve(search)


Get all user paths

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::UserPath**](UserPath.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_users_recovery_create

> models::Link core_users_recovery_create(id)


Create a temporary link that a user can use to recover their account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this User. | [required] |

### Return type

[**models::Link**](Link.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_users_recovery_email_create

> core_users_recovery_email_create(email_stage, id)


Send an email with a temporary link that a user can use to recover their account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_stage** | **String** |  | [required] |
**id** | **i32** | A unique integer value identifying this User. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_users_retrieve

> models::User core_users_retrieve(id)


User Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this User. | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_users_service_account_create

> models::UserServiceAccountResponse core_users_service_account_create(user_service_account_request)


Create a new user account that is marked as a service account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_service_account_request** | [**UserServiceAccountRequest**](UserServiceAccountRequest.md) |  | [required] |

### Return type

[**models::UserServiceAccountResponse**](UserServiceAccountResponse.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_users_set_password_create

> core_users_set_password_create(id, user_password_set_request)


Set password for user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this User. | [required] |
**user_password_set_request** | [**UserPasswordSetRequest**](UserPasswordSetRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_users_update

> models::User core_users_update(id, user_request)


User Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this User. | [required] |
**user_request** | [**UserRequest**](UserRequest.md) |  | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_users_used_by_list

> Vec<models::UsedBy> core_users_used_by_list(id)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this User. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

