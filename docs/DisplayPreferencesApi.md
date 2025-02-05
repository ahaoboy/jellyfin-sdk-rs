# \DisplayPreferencesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_display_preferences**](DisplayPreferencesApi.md#get_display_preferences) | **GET** /DisplayPreferences/{displayPreferencesId} | Get Display Preferences.
[**update_display_preferences**](DisplayPreferencesApi.md#update_display_preferences) | **POST** /DisplayPreferences/{displayPreferencesId} | Update Display Preferences.



## get_display_preferences

> models::DisplayPreferencesDto get_display_preferences(display_preferences_id, user_id, client)
Get Display Preferences.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**display_preferences_id** | **String** | Display preferences id. | [required] |
**user_id** | **uuid::Uuid** | User id. | [required] |
**client** | **String** | Client. | [required] |

### Return type

[**models::DisplayPreferencesDto**](DisplayPreferencesDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_display_preferences

> update_display_preferences(display_preferences_id, user_id, client, display_preferences_dto)
Update Display Preferences.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**display_preferences_id** | **String** | Display preferences id. | [required] |
**user_id** | **uuid::Uuid** | User Id. | [required] |
**client** | **String** | Client. | [required] |
**display_preferences_dto** | [**DisplayPreferencesDto**](DisplayPreferencesDto.md) | New Display Preferences object. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

