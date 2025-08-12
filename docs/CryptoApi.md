# \CryptoApi

All URIs are relative to */api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**crypto_certificatekeypairs_create**](CryptoApi.md#crypto_certificatekeypairs_create) | **POST** /crypto/certificatekeypairs/ | 
[**crypto_certificatekeypairs_destroy**](CryptoApi.md#crypto_certificatekeypairs_destroy) | **DELETE** /crypto/certificatekeypairs/{kp_uuid}/ | 
[**crypto_certificatekeypairs_generate_create**](CryptoApi.md#crypto_certificatekeypairs_generate_create) | **POST** /crypto/certificatekeypairs/generate/ | 
[**crypto_certificatekeypairs_list**](CryptoApi.md#crypto_certificatekeypairs_list) | **GET** /crypto/certificatekeypairs/ | 
[**crypto_certificatekeypairs_partial_update**](CryptoApi.md#crypto_certificatekeypairs_partial_update) | **PATCH** /crypto/certificatekeypairs/{kp_uuid}/ | 
[**crypto_certificatekeypairs_retrieve**](CryptoApi.md#crypto_certificatekeypairs_retrieve) | **GET** /crypto/certificatekeypairs/{kp_uuid}/ | 
[**crypto_certificatekeypairs_update**](CryptoApi.md#crypto_certificatekeypairs_update) | **PUT** /crypto/certificatekeypairs/{kp_uuid}/ | 
[**crypto_certificatekeypairs_used_by_list**](CryptoApi.md#crypto_certificatekeypairs_used_by_list) | **GET** /crypto/certificatekeypairs/{kp_uuid}/used_by/ | 
[**crypto_certificatekeypairs_view_certificate_retrieve**](CryptoApi.md#crypto_certificatekeypairs_view_certificate_retrieve) | **GET** /crypto/certificatekeypairs/{kp_uuid}/view_certificate/ | 
[**crypto_certificatekeypairs_view_private_key_retrieve**](CryptoApi.md#crypto_certificatekeypairs_view_private_key_retrieve) | **GET** /crypto/certificatekeypairs/{kp_uuid}/view_private_key/ | 



## crypto_certificatekeypairs_create

> models::CertificateKeyPair crypto_certificatekeypairs_create(certificate_key_pair_request)


CertificateKeyPair Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_key_pair_request** | [**CertificateKeyPairRequest**](CertificateKeyPairRequest.md) |  | [required] |

### Return type

[**models::CertificateKeyPair**](CertificateKeyPair.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## crypto_certificatekeypairs_destroy

> crypto_certificatekeypairs_destroy(kp_uuid)


CertificateKeyPair Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kp_uuid** | **uuid::Uuid** | A UUID string identifying this Certificate-Key Pair. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## crypto_certificatekeypairs_generate_create

> models::CertificateKeyPair crypto_certificatekeypairs_generate_create(certificate_generation_request)


Generate a new, self-signed certificate-key pair

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_generation_request** | [**CertificateGenerationRequest**](CertificateGenerationRequest.md) |  | [required] |

### Return type

[**models::CertificateKeyPair**](CertificateKeyPair.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## crypto_certificatekeypairs_list

> models::PaginatedCertificateKeyPairList crypto_certificatekeypairs_list(has_key, include_details, managed, name, ordering, page, page_size, search)


CertificateKeyPair Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**has_key** | Option<**bool**> | Only return certificate-key pairs with keys |  |
**include_details** | Option<**bool**> |  |  |[default to true]
**managed** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedCertificateKeyPairList**](PaginatedCertificateKeyPairList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## crypto_certificatekeypairs_partial_update

> models::CertificateKeyPair crypto_certificatekeypairs_partial_update(kp_uuid, patched_certificate_key_pair_request)


CertificateKeyPair Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kp_uuid** | **uuid::Uuid** | A UUID string identifying this Certificate-Key Pair. | [required] |
**patched_certificate_key_pair_request** | Option<[**PatchedCertificateKeyPairRequest**](PatchedCertificateKeyPairRequest.md)> |  |  |

### Return type

[**models::CertificateKeyPair**](CertificateKeyPair.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## crypto_certificatekeypairs_retrieve

> models::CertificateKeyPair crypto_certificatekeypairs_retrieve(kp_uuid)


CertificateKeyPair Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kp_uuid** | **uuid::Uuid** | A UUID string identifying this Certificate-Key Pair. | [required] |

### Return type

[**models::CertificateKeyPair**](CertificateKeyPair.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## crypto_certificatekeypairs_update

> models::CertificateKeyPair crypto_certificatekeypairs_update(kp_uuid, certificate_key_pair_request)


CertificateKeyPair Viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kp_uuid** | **uuid::Uuid** | A UUID string identifying this Certificate-Key Pair. | [required] |
**certificate_key_pair_request** | [**CertificateKeyPairRequest**](CertificateKeyPairRequest.md) |  | [required] |

### Return type

[**models::CertificateKeyPair**](CertificateKeyPair.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## crypto_certificatekeypairs_used_by_list

> Vec<models::UsedBy> crypto_certificatekeypairs_used_by_list(kp_uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kp_uuid** | **uuid::Uuid** | A UUID string identifying this Certificate-Key Pair. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## crypto_certificatekeypairs_view_certificate_retrieve

> models::CertificateData crypto_certificatekeypairs_view_certificate_retrieve(kp_uuid, download)


Return certificate-key pairs certificate and log access

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kp_uuid** | **uuid::Uuid** | A UUID string identifying this Certificate-Key Pair. | [required] |
**download** | Option<**bool**> |  |  |

### Return type

[**models::CertificateData**](CertificateData.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## crypto_certificatekeypairs_view_private_key_retrieve

> models::CertificateData crypto_certificatekeypairs_view_private_key_retrieve(kp_uuid, download)


Return certificate-key pairs private key and log access

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kp_uuid** | **uuid::Uuid** | A UUID string identifying this Certificate-Key Pair. | [required] |
**download** | Option<**bool**> |  |  |

### Return type

[**models::CertificateData**](CertificateData.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

