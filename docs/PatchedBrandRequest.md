# PatchedBrandRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domain** | Option<**String**> | Domain that activates this brand. Can be a superset, i.e. `a.b` for `aa.b` and `ba.b` | [optional]
**default** | Option<**bool**> |  | [optional]
**branding_title** | Option<**String**> |  | [optional]
**branding_logo** | Option<**String**> |  | [optional]
**branding_favicon** | Option<**String**> |  | [optional]
**branding_custom_css** | Option<**String**> |  | [optional]
**branding_default_flow_background** | Option<**String**> |  | [optional]
**flow_authentication** | Option<**uuid::Uuid**> |  | [optional]
**flow_invalidation** | Option<**uuid::Uuid**> |  | [optional]
**flow_recovery** | Option<**uuid::Uuid**> |  | [optional]
**flow_unenrollment** | Option<**uuid::Uuid**> |  | [optional]
**flow_user_settings** | Option<**uuid::Uuid**> |  | [optional]
**flow_device_code** | Option<**uuid::Uuid**> |  | [optional]
**default_application** | Option<**uuid::Uuid**> | When set, external users will be redirected to this application after authenticating. | [optional]
**web_certificate** | Option<**uuid::Uuid**> | Web Certificate used by the authentik Core webserver. | [optional]
**client_certificates** | Option<**Vec<uuid::Uuid>**> | Certificates used for client authentication. | [optional]
**attributes** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


