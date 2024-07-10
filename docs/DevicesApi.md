# \DevicesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_device**](DevicesApi.md#delete_device) | **DELETE** /Devices | Deletes a device.
[**get_device_info**](DevicesApi.md#get_device_info) | **GET** /Devices/Info | Get info for a device.
[**get_device_options**](DevicesApi.md#get_device_options) | **GET** /Devices/Options | Get options for a device.
[**get_devices**](DevicesApi.md#get_devices) | **GET** /Devices | Get Devices.
[**update_device_options**](DevicesApi.md#update_device_options) | **POST** /Devices/Options | Update device options.



## delete_device

> delete_device(id)
Deletes a device.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Device Id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_device_info

> models::DeviceInfo get_device_info(id)
Get info for a device.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Device Id. | [required] |

### Return type

[**models::DeviceInfo**](DeviceInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_device_options

> models::DeviceOptions get_device_options(id)
Get options for a device.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Device Id. | [required] |

### Return type

[**models::DeviceOptions**](DeviceOptions.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_devices

> models::DeviceInfoQueryResult get_devices(supports_sync, user_id)
Get Devices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**supports_sync** | Option<**bool**> | Gets or sets a value indicating whether [supports synchronize]. |  |
**user_id** | Option<**uuid::Uuid**> | Gets or sets the user identifier. |  |

### Return type

[**models::DeviceInfoQueryResult**](DeviceInfoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_device_options

> update_device_options(id, device_options_dto)
Update device options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Device Id. | [required] |
**device_options_dto** | [**DeviceOptionsDto**](DeviceOptionsDto.md) | Device Options. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

