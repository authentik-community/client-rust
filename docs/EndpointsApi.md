# \EndpointsApi

All URIs are relative to */api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**endpoints_agents_connectors_agent_config_retrieve**](EndpointsApi.md#endpoints_agents_connectors_agent_config_retrieve) | **GET** /endpoints/agents/connectors/agent_config/ | 
[**endpoints_agents_connectors_auth_fed_create**](EndpointsApi.md#endpoints_agents_connectors_auth_fed_create) | **POST** /endpoints/agents/connectors/auth_fed/ | 
[**endpoints_agents_connectors_auth_ia_create**](EndpointsApi.md#endpoints_agents_connectors_auth_ia_create) | **POST** /endpoints/agents/connectors/auth_ia/ | 
[**endpoints_agents_connectors_check_in_create**](EndpointsApi.md#endpoints_agents_connectors_check_in_create) | **POST** /endpoints/agents/connectors/check_in/ | 
[**endpoints_agents_connectors_create**](EndpointsApi.md#endpoints_agents_connectors_create) | **POST** /endpoints/agents/connectors/ | 
[**endpoints_agents_connectors_destroy**](EndpointsApi.md#endpoints_agents_connectors_destroy) | **DELETE** /endpoints/agents/connectors/{connector_uuid}/ | 
[**endpoints_agents_connectors_enroll_create**](EndpointsApi.md#endpoints_agents_connectors_enroll_create) | **POST** /endpoints/agents/connectors/enroll/ | 
[**endpoints_agents_connectors_list**](EndpointsApi.md#endpoints_agents_connectors_list) | **GET** /endpoints/agents/connectors/ | 
[**endpoints_agents_connectors_mdm_config_create**](EndpointsApi.md#endpoints_agents_connectors_mdm_config_create) | **POST** /endpoints/agents/connectors/{connector_uuid}/mdm_config/ | 
[**endpoints_agents_connectors_partial_update**](EndpointsApi.md#endpoints_agents_connectors_partial_update) | **PATCH** /endpoints/agents/connectors/{connector_uuid}/ | 
[**endpoints_agents_connectors_retrieve**](EndpointsApi.md#endpoints_agents_connectors_retrieve) | **GET** /endpoints/agents/connectors/{connector_uuid}/ | 
[**endpoints_agents_connectors_update**](EndpointsApi.md#endpoints_agents_connectors_update) | **PUT** /endpoints/agents/connectors/{connector_uuid}/ | 
[**endpoints_agents_connectors_used_by_list**](EndpointsApi.md#endpoints_agents_connectors_used_by_list) | **GET** /endpoints/agents/connectors/{connector_uuid}/used_by/ | 
[**endpoints_agents_enrollment_tokens_create**](EndpointsApi.md#endpoints_agents_enrollment_tokens_create) | **POST** /endpoints/agents/enrollment_tokens/ | 
[**endpoints_agents_enrollment_tokens_destroy**](EndpointsApi.md#endpoints_agents_enrollment_tokens_destroy) | **DELETE** /endpoints/agents/enrollment_tokens/{token_uuid}/ | 
[**endpoints_agents_enrollment_tokens_list**](EndpointsApi.md#endpoints_agents_enrollment_tokens_list) | **GET** /endpoints/agents/enrollment_tokens/ | 
[**endpoints_agents_enrollment_tokens_partial_update**](EndpointsApi.md#endpoints_agents_enrollment_tokens_partial_update) | **PATCH** /endpoints/agents/enrollment_tokens/{token_uuid}/ | 
[**endpoints_agents_enrollment_tokens_retrieve**](EndpointsApi.md#endpoints_agents_enrollment_tokens_retrieve) | **GET** /endpoints/agents/enrollment_tokens/{token_uuid}/ | 
[**endpoints_agents_enrollment_tokens_update**](EndpointsApi.md#endpoints_agents_enrollment_tokens_update) | **PUT** /endpoints/agents/enrollment_tokens/{token_uuid}/ | 
[**endpoints_agents_enrollment_tokens_used_by_list**](EndpointsApi.md#endpoints_agents_enrollment_tokens_used_by_list) | **GET** /endpoints/agents/enrollment_tokens/{token_uuid}/used_by/ | 
[**endpoints_agents_enrollment_tokens_view_key_retrieve**](EndpointsApi.md#endpoints_agents_enrollment_tokens_view_key_retrieve) | **GET** /endpoints/agents/enrollment_tokens/{token_uuid}/view_key/ | 
[**endpoints_agents_psso_register_device_create**](EndpointsApi.md#endpoints_agents_psso_register_device_create) | **POST** /endpoints/agents/psso/register/device/ | 
[**endpoints_agents_psso_register_user_create**](EndpointsApi.md#endpoints_agents_psso_register_user_create) | **POST** /endpoints/agents/psso/register/user/ | 
[**endpoints_connectors_destroy**](EndpointsApi.md#endpoints_connectors_destroy) | **DELETE** /endpoints/connectors/{connector_uuid}/ | 
[**endpoints_connectors_list**](EndpointsApi.md#endpoints_connectors_list) | **GET** /endpoints/connectors/ | 
[**endpoints_connectors_retrieve**](EndpointsApi.md#endpoints_connectors_retrieve) | **GET** /endpoints/connectors/{connector_uuid}/ | 
[**endpoints_connectors_types_list**](EndpointsApi.md#endpoints_connectors_types_list) | **GET** /endpoints/connectors/types/ | 
[**endpoints_connectors_used_by_list**](EndpointsApi.md#endpoints_connectors_used_by_list) | **GET** /endpoints/connectors/{connector_uuid}/used_by/ | 
[**endpoints_device_access_groups_create**](EndpointsApi.md#endpoints_device_access_groups_create) | **POST** /endpoints/device_access_groups/ | 
[**endpoints_device_access_groups_destroy**](EndpointsApi.md#endpoints_device_access_groups_destroy) | **DELETE** /endpoints/device_access_groups/{pbm_uuid}/ | 
[**endpoints_device_access_groups_list**](EndpointsApi.md#endpoints_device_access_groups_list) | **GET** /endpoints/device_access_groups/ | 
[**endpoints_device_access_groups_partial_update**](EndpointsApi.md#endpoints_device_access_groups_partial_update) | **PATCH** /endpoints/device_access_groups/{pbm_uuid}/ | 
[**endpoints_device_access_groups_retrieve**](EndpointsApi.md#endpoints_device_access_groups_retrieve) | **GET** /endpoints/device_access_groups/{pbm_uuid}/ | 
[**endpoints_device_access_groups_update**](EndpointsApi.md#endpoints_device_access_groups_update) | **PUT** /endpoints/device_access_groups/{pbm_uuid}/ | 
[**endpoints_device_access_groups_used_by_list**](EndpointsApi.md#endpoints_device_access_groups_used_by_list) | **GET** /endpoints/device_access_groups/{pbm_uuid}/used_by/ | 
[**endpoints_device_bindings_create**](EndpointsApi.md#endpoints_device_bindings_create) | **POST** /endpoints/device_bindings/ | 
[**endpoints_device_bindings_destroy**](EndpointsApi.md#endpoints_device_bindings_destroy) | **DELETE** /endpoints/device_bindings/{policy_binding_uuid}/ | 
[**endpoints_device_bindings_list**](EndpointsApi.md#endpoints_device_bindings_list) | **GET** /endpoints/device_bindings/ | 
[**endpoints_device_bindings_partial_update**](EndpointsApi.md#endpoints_device_bindings_partial_update) | **PATCH** /endpoints/device_bindings/{policy_binding_uuid}/ | 
[**endpoints_device_bindings_retrieve**](EndpointsApi.md#endpoints_device_bindings_retrieve) | **GET** /endpoints/device_bindings/{policy_binding_uuid}/ | 
[**endpoints_device_bindings_update**](EndpointsApi.md#endpoints_device_bindings_update) | **PUT** /endpoints/device_bindings/{policy_binding_uuid}/ | 
[**endpoints_device_bindings_used_by_list**](EndpointsApi.md#endpoints_device_bindings_used_by_list) | **GET** /endpoints/device_bindings/{policy_binding_uuid}/used_by/ | 
[**endpoints_devices_destroy**](EndpointsApi.md#endpoints_devices_destroy) | **DELETE** /endpoints/devices/{device_uuid}/ | 
[**endpoints_devices_list**](EndpointsApi.md#endpoints_devices_list) | **GET** /endpoints/devices/ | 
[**endpoints_devices_partial_update**](EndpointsApi.md#endpoints_devices_partial_update) | **PATCH** /endpoints/devices/{device_uuid}/ | 
[**endpoints_devices_retrieve**](EndpointsApi.md#endpoints_devices_retrieve) | **GET** /endpoints/devices/{device_uuid}/ | 
[**endpoints_devices_summary_retrieve**](EndpointsApi.md#endpoints_devices_summary_retrieve) | **GET** /endpoints/devices/summary/ | 
[**endpoints_devices_update**](EndpointsApi.md#endpoints_devices_update) | **PUT** /endpoints/devices/{device_uuid}/ | 
[**endpoints_devices_used_by_list**](EndpointsApi.md#endpoints_devices_used_by_list) | **GET** /endpoints/devices/{device_uuid}/used_by/ | 
[**endpoints_fleet_connectors_create**](EndpointsApi.md#endpoints_fleet_connectors_create) | **POST** /endpoints/fleet/connectors/ | 
[**endpoints_fleet_connectors_destroy**](EndpointsApi.md#endpoints_fleet_connectors_destroy) | **DELETE** /endpoints/fleet/connectors/{connector_uuid}/ | 
[**endpoints_fleet_connectors_list**](EndpointsApi.md#endpoints_fleet_connectors_list) | **GET** /endpoints/fleet/connectors/ | 
[**endpoints_fleet_connectors_partial_update**](EndpointsApi.md#endpoints_fleet_connectors_partial_update) | **PATCH** /endpoints/fleet/connectors/{connector_uuid}/ | 
[**endpoints_fleet_connectors_retrieve**](EndpointsApi.md#endpoints_fleet_connectors_retrieve) | **GET** /endpoints/fleet/connectors/{connector_uuid}/ | 
[**endpoints_fleet_connectors_update**](EndpointsApi.md#endpoints_fleet_connectors_update) | **PUT** /endpoints/fleet/connectors/{connector_uuid}/ | 
[**endpoints_fleet_connectors_used_by_list**](EndpointsApi.md#endpoints_fleet_connectors_used_by_list) | **GET** /endpoints/fleet/connectors/{connector_uuid}/used_by/ | 



## endpoints_agents_connectors_agent_config_retrieve

> models::AgentConfig endpoints_agents_connectors_agent_config_retrieve()


Mixin to add a used_by endpoint to return a list of all objects using this object

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AgentConfig**](AgentConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_agents_connectors_auth_fed_create

> models::AgentTokenResponse endpoints_agents_connectors_auth_fed_create(device)


Mixin to add a used_by endpoint to return a list of all objects using this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device** | **String** |  | [required] |

### Return type

[**models::AgentTokenResponse**](AgentTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_agents_connectors_auth_ia_create

> models::AgentAuthenticationResponse endpoints_agents_connectors_auth_ia_create()


Mixin to add a used_by endpoint to return a list of all objects using this object

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AgentAuthenticationResponse**](AgentAuthenticationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_agents_connectors_check_in_create

> endpoints_agents_connectors_check_in_create(device_facts_request)


Mixin to add a used_by endpoint to return a list of all objects using this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_facts_request** | Option<[**DeviceFactsRequest**](DeviceFactsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_agents_connectors_create

> models::AgentConnector endpoints_agents_connectors_create(agent_connector_request)


Mixin to add a used_by endpoint to return a list of all objects using this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_connector_request** | [**AgentConnectorRequest**](AgentConnectorRequest.md) |  | [required] |

### Return type

[**models::AgentConnector**](AgentConnector.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_agents_connectors_destroy

> endpoints_agents_connectors_destroy(connector_uuid)


Mixin to add a used_by endpoint to return a list of all objects using this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_uuid** | **uuid::Uuid** | A UUID string identifying this Agent Connector. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_agents_connectors_enroll_create

> models::AgentTokenResponse endpoints_agents_connectors_enroll_create(enroll_request)


Mixin to add a used_by endpoint to return a list of all objects using this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enroll_request** | [**EnrollRequest**](EnrollRequest.md) |  | [required] |

### Return type

[**models::AgentTokenResponse**](AgentTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_agents_connectors_list

> models::PaginatedAgentConnectorList endpoints_agents_connectors_list(enabled, name, ordering, page, page_size, search)


Mixin to add a used_by endpoint to return a list of all objects using this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enabled** | Option<**bool**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedAgentConnectorList**](PaginatedAgentConnectorList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_agents_connectors_mdm_config_create

> models::MdmConfigResponse endpoints_agents_connectors_mdm_config_create(connector_uuid, mdm_config_request)


Generate configuration for MDM systems to deploy authentik Agent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_uuid** | **uuid::Uuid** | A UUID string identifying this Agent Connector. | [required] |
**mdm_config_request** | [**MdmConfigRequest**](MdmConfigRequest.md) |  | [required] |

### Return type

[**models::MdmConfigResponse**](MDMConfigResponse.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_agents_connectors_partial_update

> models::AgentConnector endpoints_agents_connectors_partial_update(connector_uuid, patched_agent_connector_request)


Mixin to add a used_by endpoint to return a list of all objects using this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_uuid** | **uuid::Uuid** | A UUID string identifying this Agent Connector. | [required] |
**patched_agent_connector_request** | Option<[**PatchedAgentConnectorRequest**](PatchedAgentConnectorRequest.md)> |  |  |

### Return type

[**models::AgentConnector**](AgentConnector.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_agents_connectors_retrieve

> models::AgentConnector endpoints_agents_connectors_retrieve(connector_uuid)


Mixin to add a used_by endpoint to return a list of all objects using this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_uuid** | **uuid::Uuid** | A UUID string identifying this Agent Connector. | [required] |

### Return type

[**models::AgentConnector**](AgentConnector.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_agents_connectors_update

> models::AgentConnector endpoints_agents_connectors_update(connector_uuid, agent_connector_request)


Mixin to add a used_by endpoint to return a list of all objects using this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_uuid** | **uuid::Uuid** | A UUID string identifying this Agent Connector. | [required] |
**agent_connector_request** | [**AgentConnectorRequest**](AgentConnectorRequest.md) |  | [required] |

### Return type

[**models::AgentConnector**](AgentConnector.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_agents_connectors_used_by_list

> Vec<models::UsedBy> endpoints_agents_connectors_used_by_list(connector_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_uuid** | **uuid::Uuid** | A UUID string identifying this Agent Connector. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_agents_enrollment_tokens_create

> models::EnrollmentToken endpoints_agents_enrollment_tokens_create(enrollment_token_request)


Mixin to add a used_by endpoint to return a list of all objects using this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enrollment_token_request** | [**EnrollmentTokenRequest**](EnrollmentTokenRequest.md) |  | [required] |

### Return type

[**models::EnrollmentToken**](EnrollmentToken.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_agents_enrollment_tokens_destroy

> endpoints_agents_enrollment_tokens_destroy(token_uuid)


Mixin to add a used_by endpoint to return a list of all objects using this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_uuid** | **uuid::Uuid** | A UUID string identifying this Enrollment Token. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_agents_enrollment_tokens_list

> models::PaginatedEnrollmentTokenList endpoints_agents_enrollment_tokens_list(connector, ordering, page, page_size, search, token_uuid)


Mixin to add a used_by endpoint to return a list of all objects using this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector** | Option<**uuid::Uuid**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |
**token_uuid** | Option<**uuid::Uuid**> |  |  |

### Return type

[**models::PaginatedEnrollmentTokenList**](PaginatedEnrollmentTokenList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_agents_enrollment_tokens_partial_update

> models::EnrollmentToken endpoints_agents_enrollment_tokens_partial_update(token_uuid, patched_enrollment_token_request)


Mixin to add a used_by endpoint to return a list of all objects using this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_uuid** | **uuid::Uuid** | A UUID string identifying this Enrollment Token. | [required] |
**patched_enrollment_token_request** | Option<[**PatchedEnrollmentTokenRequest**](PatchedEnrollmentTokenRequest.md)> |  |  |

### Return type

[**models::EnrollmentToken**](EnrollmentToken.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_agents_enrollment_tokens_retrieve

> models::EnrollmentToken endpoints_agents_enrollment_tokens_retrieve(token_uuid)


Mixin to add a used_by endpoint to return a list of all objects using this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_uuid** | **uuid::Uuid** | A UUID string identifying this Enrollment Token. | [required] |

### Return type

[**models::EnrollmentToken**](EnrollmentToken.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_agents_enrollment_tokens_update

> models::EnrollmentToken endpoints_agents_enrollment_tokens_update(token_uuid, enrollment_token_request)


Mixin to add a used_by endpoint to return a list of all objects using this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_uuid** | **uuid::Uuid** | A UUID string identifying this Enrollment Token. | [required] |
**enrollment_token_request** | [**EnrollmentTokenRequest**](EnrollmentTokenRequest.md) |  | [required] |

### Return type

[**models::EnrollmentToken**](EnrollmentToken.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_agents_enrollment_tokens_used_by_list

> Vec<models::UsedBy> endpoints_agents_enrollment_tokens_used_by_list(token_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_uuid** | **uuid::Uuid** | A UUID string identifying this Enrollment Token. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_agents_enrollment_tokens_view_key_retrieve

> models::TokenView endpoints_agents_enrollment_tokens_view_key_retrieve(token_uuid)


Return token key and log access

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_uuid** | **uuid::Uuid** | A UUID string identifying this Enrollment Token. | [required] |

### Return type

[**models::TokenView**](TokenView.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_agents_psso_register_device_create

> models::AgentPssoDeviceRegistrationResponse endpoints_agents_psso_register_device_create(agent_psso_device_registration_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_psso_device_registration_request** | [**AgentPssoDeviceRegistrationRequest**](AgentPssoDeviceRegistrationRequest.md) |  | [required] |

### Return type

[**models::AgentPssoDeviceRegistrationResponse**](AgentPSSODeviceRegistrationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_agents_psso_register_user_create

> models::UserSelf endpoints_agents_psso_register_user_create(agent_psso_user_registration_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_psso_user_registration_request** | [**AgentPssoUserRegistrationRequest**](AgentPssoUserRegistrationRequest.md) |  | [required] |

### Return type

[**models::UserSelf**](UserSelf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_connectors_destroy

> endpoints_connectors_destroy(connector_uuid)


Connector Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_uuid** | **uuid::Uuid** | A UUID string identifying this connector. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_connectors_list

> models::PaginatedConnectorList endpoints_connectors_list(ordering, page, page_size, search)


Connector Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedConnectorList**](PaginatedConnectorList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_connectors_retrieve

> models::Connector endpoints_connectors_retrieve(connector_uuid)


Connector Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_uuid** | **uuid::Uuid** | A UUID string identifying this connector. | [required] |

### Return type

[**models::Connector**](Connector.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_connectors_types_list

> Vec<models::TypeCreate> endpoints_connectors_types_list()


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


## endpoints_connectors_used_by_list

> Vec<models::UsedBy> endpoints_connectors_used_by_list(connector_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_uuid** | **uuid::Uuid** | A UUID string identifying this connector. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_device_access_groups_create

> models::DeviceAccessGroup endpoints_device_access_groups_create(device_access_group_request)


DeviceAccessGroup Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_access_group_request** | [**DeviceAccessGroupRequest**](DeviceAccessGroupRequest.md) |  | [required] |

### Return type

[**models::DeviceAccessGroup**](DeviceAccessGroup.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_device_access_groups_destroy

> endpoints_device_access_groups_destroy(pbm_uuid)


DeviceAccessGroup Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pbm_uuid** | **uuid::Uuid** | A UUID string identifying this Device access group. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_device_access_groups_list

> models::PaginatedDeviceAccessGroupList endpoints_device_access_groups_list(name, ordering, page, page_size, pbm_uuid, search)


DeviceAccessGroup Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**pbm_uuid** | Option<**uuid::Uuid**> |  |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedDeviceAccessGroupList**](PaginatedDeviceAccessGroupList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_device_access_groups_partial_update

> models::DeviceAccessGroup endpoints_device_access_groups_partial_update(pbm_uuid, patched_device_access_group_request)


DeviceAccessGroup Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pbm_uuid** | **uuid::Uuid** | A UUID string identifying this Device access group. | [required] |
**patched_device_access_group_request** | Option<[**PatchedDeviceAccessGroupRequest**](PatchedDeviceAccessGroupRequest.md)> |  |  |

### Return type

[**models::DeviceAccessGroup**](DeviceAccessGroup.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_device_access_groups_retrieve

> models::DeviceAccessGroup endpoints_device_access_groups_retrieve(pbm_uuid)


DeviceAccessGroup Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pbm_uuid** | **uuid::Uuid** | A UUID string identifying this Device access group. | [required] |

### Return type

[**models::DeviceAccessGroup**](DeviceAccessGroup.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_device_access_groups_update

> models::DeviceAccessGroup endpoints_device_access_groups_update(pbm_uuid, device_access_group_request)


DeviceAccessGroup Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pbm_uuid** | **uuid::Uuid** | A UUID string identifying this Device access group. | [required] |
**device_access_group_request** | [**DeviceAccessGroupRequest**](DeviceAccessGroupRequest.md) |  | [required] |

### Return type

[**models::DeviceAccessGroup**](DeviceAccessGroup.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_device_access_groups_used_by_list

> Vec<models::UsedBy> endpoints_device_access_groups_used_by_list(pbm_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pbm_uuid** | **uuid::Uuid** | A UUID string identifying this Device access group. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_device_bindings_create

> models::DeviceUserBinding endpoints_device_bindings_create(device_user_binding_request)


PolicyBinding Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_user_binding_request** | [**DeviceUserBindingRequest**](DeviceUserBindingRequest.md) |  | [required] |

### Return type

[**models::DeviceUserBinding**](DeviceUserBinding.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_device_bindings_destroy

> endpoints_device_bindings_destroy(policy_binding_uuid)


PolicyBinding Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_binding_uuid** | **uuid::Uuid** | A UUID string identifying this Device User binding. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_device_bindings_list

> models::PaginatedDeviceUserBindingList endpoints_device_bindings_list(enabled, order, ordering, page, page_size, policy, policy__isnull, search, target, target_in, timeout)


PolicyBinding Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enabled** | Option<**bool**> |  |  |
**order** | Option<**i32**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**policy** | Option<**uuid::Uuid**> |  |  |
**policy__isnull** | Option<**bool**> |  |  |
**search** | Option<**String**> | A search term. |  |
**target** | Option<**uuid::Uuid**> |  |  |
**target_in** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> |  |  |
**timeout** | Option<**i32**> |  |  |

### Return type

[**models::PaginatedDeviceUserBindingList**](PaginatedDeviceUserBindingList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_device_bindings_partial_update

> models::DeviceUserBinding endpoints_device_bindings_partial_update(policy_binding_uuid, patched_device_user_binding_request)


PolicyBinding Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_binding_uuid** | **uuid::Uuid** | A UUID string identifying this Device User binding. | [required] |
**patched_device_user_binding_request** | Option<[**PatchedDeviceUserBindingRequest**](PatchedDeviceUserBindingRequest.md)> |  |  |

### Return type

[**models::DeviceUserBinding**](DeviceUserBinding.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_device_bindings_retrieve

> models::DeviceUserBinding endpoints_device_bindings_retrieve(policy_binding_uuid)


PolicyBinding Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_binding_uuid** | **uuid::Uuid** | A UUID string identifying this Device User binding. | [required] |

### Return type

[**models::DeviceUserBinding**](DeviceUserBinding.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_device_bindings_update

> models::DeviceUserBinding endpoints_device_bindings_update(policy_binding_uuid, device_user_binding_request)


PolicyBinding Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_binding_uuid** | **uuid::Uuid** | A UUID string identifying this Device User binding. | [required] |
**device_user_binding_request** | [**DeviceUserBindingRequest**](DeviceUserBindingRequest.md) |  | [required] |

### Return type

[**models::DeviceUserBinding**](DeviceUserBinding.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_device_bindings_used_by_list

> Vec<models::UsedBy> endpoints_device_bindings_used_by_list(policy_binding_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_binding_uuid** | **uuid::Uuid** | A UUID string identifying this Device User binding. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_devices_destroy

> endpoints_devices_destroy(device_uuid)


Mixin to add a used_by endpoint to return a list of all objects using this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_uuid** | **uuid::Uuid** | A UUID string identifying this Device. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_devices_list

> models::PaginatedEndpointDeviceList endpoints_devices_list(identifier, name, ordering, page, page_size, search)


Mixin to add a used_by endpoint to return a list of all objects using this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedEndpointDeviceList**](PaginatedEndpointDeviceList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_devices_partial_update

> models::EndpointDevice endpoints_devices_partial_update(device_uuid, patched_endpoint_device_request)


Mixin to add a used_by endpoint to return a list of all objects using this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_uuid** | **uuid::Uuid** | A UUID string identifying this Device. | [required] |
**patched_endpoint_device_request** | Option<[**PatchedEndpointDeviceRequest**](PatchedEndpointDeviceRequest.md)> |  |  |

### Return type

[**models::EndpointDevice**](EndpointDevice.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_devices_retrieve

> models::EndpointDeviceDetails endpoints_devices_retrieve(device_uuid)


Mixin to add a used_by endpoint to return a list of all objects using this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_uuid** | **uuid::Uuid** | A UUID string identifying this Device. | [required] |

### Return type

[**models::EndpointDeviceDetails**](EndpointDeviceDetails.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_devices_summary_retrieve

> models::DeviceSummary endpoints_devices_summary_retrieve()


Mixin to add a used_by endpoint to return a list of all objects using this object

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DeviceSummary**](DeviceSummary.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_devices_update

> models::EndpointDevice endpoints_devices_update(device_uuid, endpoint_device_request)


Mixin to add a used_by endpoint to return a list of all objects using this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_uuid** | **uuid::Uuid** | A UUID string identifying this Device. | [required] |
**endpoint_device_request** | [**EndpointDeviceRequest**](EndpointDeviceRequest.md) |  | [required] |

### Return type

[**models::EndpointDevice**](EndpointDevice.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_devices_used_by_list

> Vec<models::UsedBy> endpoints_devices_used_by_list(device_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_uuid** | **uuid::Uuid** | A UUID string identifying this Device. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_fleet_connectors_create

> models::FleetConnector endpoints_fleet_connectors_create(fleet_connector_request)


FleetConnector Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fleet_connector_request** | [**FleetConnectorRequest**](FleetConnectorRequest.md) |  | [required] |

### Return type

[**models::FleetConnector**](FleetConnector.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_fleet_connectors_destroy

> endpoints_fleet_connectors_destroy(connector_uuid)


FleetConnector Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_uuid** | **uuid::Uuid** | A UUID string identifying this Fleet Connector. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_fleet_connectors_list

> models::PaginatedFleetConnectorList endpoints_fleet_connectors_list(name, ordering, page, page_size, search)


FleetConnector Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedFleetConnectorList**](PaginatedFleetConnectorList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_fleet_connectors_partial_update

> models::FleetConnector endpoints_fleet_connectors_partial_update(connector_uuid, patched_fleet_connector_request)


FleetConnector Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_uuid** | **uuid::Uuid** | A UUID string identifying this Fleet Connector. | [required] |
**patched_fleet_connector_request** | Option<[**PatchedFleetConnectorRequest**](PatchedFleetConnectorRequest.md)> |  |  |

### Return type

[**models::FleetConnector**](FleetConnector.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_fleet_connectors_retrieve

> models::FleetConnector endpoints_fleet_connectors_retrieve(connector_uuid)


FleetConnector Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_uuid** | **uuid::Uuid** | A UUID string identifying this Fleet Connector. | [required] |

### Return type

[**models::FleetConnector**](FleetConnector.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_fleet_connectors_update

> models::FleetConnector endpoints_fleet_connectors_update(connector_uuid, fleet_connector_request)


FleetConnector Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_uuid** | **uuid::Uuid** | A UUID string identifying this Fleet Connector. | [required] |
**fleet_connector_request** | [**FleetConnectorRequest**](FleetConnectorRequest.md) |  | [required] |

### Return type

[**models::FleetConnector**](FleetConnector.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_fleet_connectors_used_by_list

> Vec<models::UsedBy> endpoints_fleet_connectors_used_by_list(connector_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_uuid** | **uuid::Uuid** | A UUID string identifying this Fleet Connector. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

