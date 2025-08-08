# BrandRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domain** | **String** | Domain that activates this brand. Can be a superset, i.e. `a.b` for `aa.b` and `ba.b` | 
**default** | Option<**bool**> |  | [optional]
**branding_title** | Option<**String**> |  | [optional]
**branding_logo** | Option<**String**> |  | [optional]
**branding_favicon** | Option<**String**> |  | [optional]
**branding_custom_css** | Option<**String**> |  | [optional]
**branding_default_flow_background** | Option<**String**> |  | [optional]
**flow_authentication** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**flow_invalidation** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**flow_recovery** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**flow_unenrollment** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**flow_user_settings** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**flow_device_code** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**default_application** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | When set, external users will be redirected to this application after authenticating. | [optional]
**web_certificate** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Web Certificate used by the authentik Core webserver. | [optional]
**client_certificates** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Certificates used for client authentication. | [optional]
**attributes** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


