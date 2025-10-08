# \RbacApi

All URIs are relative to */api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rbac_initial_permissions_create**](RbacApi.md#rbac_initial_permissions_create) | **POST** /rbac/initial_permissions/ | 
[**rbac_initial_permissions_destroy**](RbacApi.md#rbac_initial_permissions_destroy) | **DELETE** /rbac/initial_permissions/{id}/ | 
[**rbac_initial_permissions_list**](RbacApi.md#rbac_initial_permissions_list) | **GET** /rbac/initial_permissions/ | 
[**rbac_initial_permissions_partial_update**](RbacApi.md#rbac_initial_permissions_partial_update) | **PATCH** /rbac/initial_permissions/{id}/ | 
[**rbac_initial_permissions_retrieve**](RbacApi.md#rbac_initial_permissions_retrieve) | **GET** /rbac/initial_permissions/{id}/ | 
[**rbac_initial_permissions_update**](RbacApi.md#rbac_initial_permissions_update) | **PUT** /rbac/initial_permissions/{id}/ | 
[**rbac_initial_permissions_used_by_list**](RbacApi.md#rbac_initial_permissions_used_by_list) | **GET** /rbac/initial_permissions/{id}/used_by/ | 
[**rbac_permissions_assigned_by_roles_assign**](RbacApi.md#rbac_permissions_assigned_by_roles_assign) | **POST** /rbac/permissions/assigned_by_roles/{uuid}/assign/ | 
[**rbac_permissions_assigned_by_roles_list**](RbacApi.md#rbac_permissions_assigned_by_roles_list) | **GET** /rbac/permissions/assigned_by_roles/ | 
[**rbac_permissions_assigned_by_roles_unassign_partial_update**](RbacApi.md#rbac_permissions_assigned_by_roles_unassign_partial_update) | **PATCH** /rbac/permissions/assigned_by_roles/{uuid}/unassign/ | 
[**rbac_permissions_assigned_by_users_assign**](RbacApi.md#rbac_permissions_assigned_by_users_assign) | **POST** /rbac/permissions/assigned_by_users/{id}/assign/ | 
[**rbac_permissions_assigned_by_users_list**](RbacApi.md#rbac_permissions_assigned_by_users_list) | **GET** /rbac/permissions/assigned_by_users/ | 
[**rbac_permissions_assigned_by_users_unassign_partial_update**](RbacApi.md#rbac_permissions_assigned_by_users_unassign_partial_update) | **PATCH** /rbac/permissions/assigned_by_users/{id}/unassign/ | 
[**rbac_permissions_list**](RbacApi.md#rbac_permissions_list) | **GET** /rbac/permissions/ | 
[**rbac_permissions_retrieve**](RbacApi.md#rbac_permissions_retrieve) | **GET** /rbac/permissions/{id}/ | 
[**rbac_permissions_roles_destroy**](RbacApi.md#rbac_permissions_roles_destroy) | **DELETE** /rbac/permissions/roles/{id}/ | 
[**rbac_permissions_roles_list**](RbacApi.md#rbac_permissions_roles_list) | **GET** /rbac/permissions/roles/ | 
[**rbac_permissions_roles_partial_update**](RbacApi.md#rbac_permissions_roles_partial_update) | **PATCH** /rbac/permissions/roles/{id}/ | 
[**rbac_permissions_roles_retrieve**](RbacApi.md#rbac_permissions_roles_retrieve) | **GET** /rbac/permissions/roles/{id}/ | 
[**rbac_permissions_roles_update**](RbacApi.md#rbac_permissions_roles_update) | **PUT** /rbac/permissions/roles/{id}/ | 
[**rbac_permissions_users_destroy**](RbacApi.md#rbac_permissions_users_destroy) | **DELETE** /rbac/permissions/users/{id}/ | 
[**rbac_permissions_users_list**](RbacApi.md#rbac_permissions_users_list) | **GET** /rbac/permissions/users/ | 
[**rbac_permissions_users_partial_update**](RbacApi.md#rbac_permissions_users_partial_update) | **PATCH** /rbac/permissions/users/{id}/ | 
[**rbac_permissions_users_retrieve**](RbacApi.md#rbac_permissions_users_retrieve) | **GET** /rbac/permissions/users/{id}/ | 
[**rbac_permissions_users_update**](RbacApi.md#rbac_permissions_users_update) | **PUT** /rbac/permissions/users/{id}/ | 
[**rbac_roles_create**](RbacApi.md#rbac_roles_create) | **POST** /rbac/roles/ | 
[**rbac_roles_destroy**](RbacApi.md#rbac_roles_destroy) | **DELETE** /rbac/roles/{uuid}/ | 
[**rbac_roles_list**](RbacApi.md#rbac_roles_list) | **GET** /rbac/roles/ | 
[**rbac_roles_partial_update**](RbacApi.md#rbac_roles_partial_update) | **PATCH** /rbac/roles/{uuid}/ | 
[**rbac_roles_retrieve**](RbacApi.md#rbac_roles_retrieve) | **GET** /rbac/roles/{uuid}/ | 
[**rbac_roles_update**](RbacApi.md#rbac_roles_update) | **PUT** /rbac/roles/{uuid}/ | 
[**rbac_roles_used_by_list**](RbacApi.md#rbac_roles_used_by_list) | **GET** /rbac/roles/{uuid}/used_by/ | 



## rbac_initial_permissions_create

> models::InitialPermissions rbac_initial_permissions_create(initial_permissions_request)


InitialPermissions viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**initial_permissions_request** | [**InitialPermissionsRequest**](InitialPermissionsRequest.md) |  | [required] |

### Return type

[**models::InitialPermissions**](InitialPermissions.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_initial_permissions_destroy

> rbac_initial_permissions_destroy(id)


InitialPermissions viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Initial Permissions. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_initial_permissions_list

> models::PaginatedInitialPermissionsList rbac_initial_permissions_list(name, ordering, page, page_size, search)


InitialPermissions viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedInitialPermissionsList**](PaginatedInitialPermissionsList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_initial_permissions_partial_update

> models::InitialPermissions rbac_initial_permissions_partial_update(id, patched_initial_permissions_request)


InitialPermissions viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Initial Permissions. | [required] |
**patched_initial_permissions_request** | Option<[**PatchedInitialPermissionsRequest**](PatchedInitialPermissionsRequest.md)> |  |  |

### Return type

[**models::InitialPermissions**](InitialPermissions.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_initial_permissions_retrieve

> models::InitialPermissions rbac_initial_permissions_retrieve(id)


InitialPermissions viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Initial Permissions. | [required] |

### Return type

[**models::InitialPermissions**](InitialPermissions.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_initial_permissions_update

> models::InitialPermissions rbac_initial_permissions_update(id, initial_permissions_request)


InitialPermissions viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Initial Permissions. | [required] |
**initial_permissions_request** | [**InitialPermissionsRequest**](InitialPermissionsRequest.md) |  | [required] |

### Return type

[**models::InitialPermissions**](InitialPermissions.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_initial_permissions_used_by_list

> Vec<models::UsedBy> rbac_initial_permissions_used_by_list(id)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Initial Permissions. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_assigned_by_roles_assign

> Vec<models::PermissionAssignResult> rbac_permissions_assigned_by_roles_assign(uuid, permission_assign_request)


Assign permission(s) to role. When `object_pk` is set, the permissions are only assigned to the specific object, otherwise they are assigned globally.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Role. | [required] |
**permission_assign_request** | [**PermissionAssignRequest**](PermissionAssignRequest.md) |  | [required] |

### Return type

[**Vec<models::PermissionAssignResult>**](PermissionAssignResult.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_assigned_by_roles_list

> models::PaginatedRoleAssignedObjectPermissionList rbac_permissions_assigned_by_roles_list(model, object_pk, ordering, page, page_size, search)


Get assigned object permissions for a single object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model** | **String** |  | [required] |
**object_pk** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedRoleAssignedObjectPermissionList**](PaginatedRoleAssignedObjectPermissionList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_assigned_by_roles_unassign_partial_update

> rbac_permissions_assigned_by_roles_unassign_partial_update(uuid, patched_permission_assign_request)


Unassign permission(s) to role. When `object_pk` is set, the permissions are only assigned to the specific object, otherwise they are assigned globally.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Role. | [required] |
**patched_permission_assign_request** | Option<[**PatchedPermissionAssignRequest**](PatchedPermissionAssignRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_assigned_by_users_assign

> Vec<models::PermissionAssignResult> rbac_permissions_assigned_by_users_assign(id, permission_assign_request)


Assign permission(s) to user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this User. | [required] |
**permission_assign_request** | [**PermissionAssignRequest**](PermissionAssignRequest.md) |  | [required] |

### Return type

[**Vec<models::PermissionAssignResult>**](PermissionAssignResult.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_assigned_by_users_list

> models::PaginatedUserAssignedObjectPermissionList rbac_permissions_assigned_by_users_list(model, object_pk, ordering, page, page_size, search)


Get assigned object permissions for a single object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model** | **String** |  | [required] |
**object_pk** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedUserAssignedObjectPermissionList**](PaginatedUserAssignedObjectPermissionList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_assigned_by_users_unassign_partial_update

> rbac_permissions_assigned_by_users_unassign_partial_update(id, patched_permission_assign_request)


Unassign permission(s) to user. When `object_pk` is set, the permissions are only assigned to the specific object, otherwise they are assigned globally.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this User. | [required] |
**patched_permission_assign_request** | Option<[**PatchedPermissionAssignRequest**](PatchedPermissionAssignRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_list

> models::PaginatedPermissionList rbac_permissions_list(codename, content_type__app_label, content_type__model, ordering, page, page_size, role, search, user)


Read-only list of all permissions, filterable by model and app

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**codename** | Option<**String**> |  |  |
**content_type__app_label** | Option<**String**> |  |  |
**content_type__model** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**role** | Option<**String**> |  |  |
**search** | Option<**String**> | A search term. |  |
**user** | Option<**i32**> |  |  |

### Return type

[**models::PaginatedPermissionList**](PaginatedPermissionList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_retrieve

> models::Permission rbac_permissions_retrieve(id)


Read-only list of all permissions, filterable by model and app

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this permission. | [required] |

### Return type

[**models::Permission**](Permission.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_roles_destroy

> rbac_permissions_roles_destroy(id)


Get a role's assigned object permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this group object permission. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_roles_list

> models::PaginatedExtraRoleObjectPermissionList rbac_permissions_roles_list(ordering, page, page_size, search, uuid)


Get a role's assigned object permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**uuid** | Option<**uuid::Uuid**> |  |  |

### Return type

[**models::PaginatedExtraRoleObjectPermissionList**](PaginatedExtraRoleObjectPermissionList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_roles_partial_update

> models::ExtraRoleObjectPermission rbac_permissions_roles_partial_update(id, patched_extra_role_object_permission_request)


Get a role's assigned object permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this group object permission. | [required] |
**patched_extra_role_object_permission_request** | Option<[**PatchedExtraRoleObjectPermissionRequest**](PatchedExtraRoleObjectPermissionRequest.md)> |  |  |

### Return type

[**models::ExtraRoleObjectPermission**](ExtraRoleObjectPermission.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_roles_retrieve

> models::ExtraRoleObjectPermission rbac_permissions_roles_retrieve(id)


Get a role's assigned object permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this group object permission. | [required] |

### Return type

[**models::ExtraRoleObjectPermission**](ExtraRoleObjectPermission.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_roles_update

> models::ExtraRoleObjectPermission rbac_permissions_roles_update(id, extra_role_object_permission_request)


Get a role's assigned object permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this group object permission. | [required] |
**extra_role_object_permission_request** | [**ExtraRoleObjectPermissionRequest**](ExtraRoleObjectPermissionRequest.md) |  | [required] |

### Return type

[**models::ExtraRoleObjectPermission**](ExtraRoleObjectPermission.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_users_destroy

> rbac_permissions_users_destroy(id)


Get a users's assigned object permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this user object permission. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_users_list

> models::PaginatedExtraUserObjectPermissionList rbac_permissions_users_list(ordering, page, page_size, search, user_id)


Get a users's assigned object permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**user_id** | Option<**i32**> |  |  |

### Return type

[**models::PaginatedExtraUserObjectPermissionList**](PaginatedExtraUserObjectPermissionList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_users_partial_update

> models::ExtraUserObjectPermission rbac_permissions_users_partial_update(id, patched_extra_user_object_permission_request)


Get a users's assigned object permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this user object permission. | [required] |
**patched_extra_user_object_permission_request** | Option<[**PatchedExtraUserObjectPermissionRequest**](PatchedExtraUserObjectPermissionRequest.md)> |  |  |

### Return type

[**models::ExtraUserObjectPermission**](ExtraUserObjectPermission.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_users_retrieve

> models::ExtraUserObjectPermission rbac_permissions_users_retrieve(id)


Get a users's assigned object permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this user object permission. | [required] |

### Return type

[**models::ExtraUserObjectPermission**](ExtraUserObjectPermission.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_users_update

> models::ExtraUserObjectPermission rbac_permissions_users_update(id, extra_user_object_permission_request)


Get a users's assigned object permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this user object permission. | [required] |
**extra_user_object_permission_request** | [**ExtraUserObjectPermissionRequest**](ExtraUserObjectPermissionRequest.md) |  | [required] |

### Return type

[**models::ExtraUserObjectPermission**](ExtraUserObjectPermission.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_roles_create

> models::Role rbac_roles_create(role_request)


Role viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_request** | [**RoleRequest**](RoleRequest.md) |  | [required] |

### Return type

[**models::Role**](Role.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_roles_destroy

> rbac_roles_destroy(uuid)


Role viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Role. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_roles_list

> models::PaginatedRoleList rbac_roles_list(name, ordering, page, page_size, search)


Role viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedRoleList**](PaginatedRoleList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_roles_partial_update

> models::Role rbac_roles_partial_update(uuid, patched_role_request)


Role viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Role. | [required] |
**patched_role_request** | Option<[**PatchedRoleRequest**](PatchedRoleRequest.md)> |  |  |

### Return type

[**models::Role**](Role.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_roles_retrieve

> models::Role rbac_roles_retrieve(uuid)


Role viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Role. | [required] |

### Return type

[**models::Role**](Role.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_roles_update

> models::Role rbac_roles_update(uuid, role_request)


Role viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Role. | [required] |
**role_request** | [**RoleRequest**](RoleRequest.md) |  | [required] |

### Return type

[**models::Role**](Role.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_roles_used_by_list

> Vec<models::UsedBy> rbac_roles_used_by_list(uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Role. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

