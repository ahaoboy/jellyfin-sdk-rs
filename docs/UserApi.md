# \UserApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**authenticate_user**](UserApi.md#authenticate_user) | **POST** /Users/{userId}/Authenticate | Authenticates a user.
[**authenticate_user_by_name**](UserApi.md#authenticate_user_by_name) | **POST** /Users/AuthenticateByName | Authenticates a user by name.
[**authenticate_with_quick_connect**](UserApi.md#authenticate_with_quick_connect) | **POST** /Users/AuthenticateWithQuickConnect | Authenticates a user with quick connect.
[**create_user_by_name**](UserApi.md#create_user_by_name) | **POST** /Users/New | Creates a user.
[**delete_user**](UserApi.md#delete_user) | **DELETE** /Users/{userId} | Deletes a user.
[**forgot_password**](UserApi.md#forgot_password) | **POST** /Users/ForgotPassword | Initiates the forgot password process for a local user.
[**forgot_password_pin**](UserApi.md#forgot_password_pin) | **POST** /Users/ForgotPassword/Pin | Redeems a forgot password pin.
[**get_current_user**](UserApi.md#get_current_user) | **GET** /Users/Me | Gets the user based on auth token.
[**get_public_users**](UserApi.md#get_public_users) | **GET** /Users/Public | Gets a list of publicly visible users for display on a login screen.
[**get_user_by_id**](UserApi.md#get_user_by_id) | **GET** /Users/{userId} | Gets a user by Id.
[**get_users**](UserApi.md#get_users) | **GET** /Users | Gets a list of users.
[**update_user**](UserApi.md#update_user) | **POST** /Users/{userId} | Updates a user.
[**update_user_configuration**](UserApi.md#update_user_configuration) | **POST** /Users/{userId}/Configuration | Updates a user configuration.
[**update_user_easy_password**](UserApi.md#update_user_easy_password) | **POST** /Users/{userId}/EasyPassword | Updates a user's easy password.
[**update_user_password**](UserApi.md#update_user_password) | **POST** /Users/{userId}/Password | Updates a user's password.
[**update_user_policy**](UserApi.md#update_user_policy) | **POST** /Users/{userId}/Policy | Updates a user policy.



## authenticate_user

> models::AuthenticationResult authenticate_user(user_id, pw, password)
Authenticates a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | The user id. | [required] |
**pw** | **String** | The password as plain text. | [required] |
**password** | Option<**String**> | The password sha1-hash. |  |

### Return type

[**models::AuthenticationResult**](AuthenticationResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## authenticate_user_by_name

> models::AuthenticationResult authenticate_user_by_name(authenticate_user_by_name)
Authenticates a user by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authenticate_user_by_name** | [**AuthenticateUserByName**](AuthenticateUserByName.md) | The M:Jellyfin.Api.Controllers.UserController.AuthenticateUserByName(Jellyfin.Api.Models.UserDtos.AuthenticateUserByName) request. | [required] |

### Return type

[**models::AuthenticationResult**](AuthenticationResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## authenticate_with_quick_connect

> models::AuthenticationResult authenticate_with_quick_connect(quick_connect_dto)
Authenticates a user with quick connect.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quick_connect_dto** | [**QuickConnectDto**](QuickConnectDto.md) | The Jellyfin.Api.Models.UserDtos.QuickConnectDto request. | [required] |

### Return type

[**models::AuthenticationResult**](AuthenticationResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_by_name

> models::UserDto create_user_by_name(create_user_by_name)
Creates a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user_by_name** | [**CreateUserByName**](CreateUserByName.md) | The create user by name request body. | [required] |

### Return type

[**models::UserDto**](UserDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> delete_user(user_id)
Deletes a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | The user id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forgot_password

> models::ForgotPasswordResult forgot_password(forgot_password_dto)
Initiates the forgot password process for a local user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**forgot_password_dto** | [**ForgotPasswordDto**](ForgotPasswordDto.md) | The forgot password request containing the entered username. | [required] |

### Return type

[**models::ForgotPasswordResult**](ForgotPasswordResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forgot_password_pin

> models::PinRedeemResult forgot_password_pin(forgot_password_pin_dto)
Redeems a forgot password pin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**forgot_password_pin_dto** | [**ForgotPasswordPinDto**](ForgotPasswordPinDto.md) | The forgot password pin request containing the entered pin. | [required] |

### Return type

[**models::PinRedeemResult**](PinRedeemResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_user

> models::UserDto get_current_user()
Gets the user based on auth token.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserDto**](UserDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_public_users

> Vec<models::UserDto> get_public_users()
Gets a list of publicly visible users for display on a login screen.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::UserDto>**](UserDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_by_id

> models::UserDto get_user_by_id(user_id)
Gets a user by Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | The user id. | [required] |

### Return type

[**models::UserDto**](UserDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users

> Vec<models::UserDto> get_users(is_hidden, is_disabled)
Gets a list of users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**is_hidden** | Option<**bool**> | Optional filter by IsHidden=true or false. |  |
**is_disabled** | Option<**bool**> | Optional filter by IsDisabled=true or false. |  |

### Return type

[**Vec<models::UserDto>**](UserDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> update_user(user_id, user_dto)
Updates a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | The user id. | [required] |
**user_dto** | [**UserDto**](UserDto.md) | The updated user model. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_configuration

> update_user_configuration(user_id, user_configuration)
Updates a user configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | The user id. | [required] |
**user_configuration** | [**UserConfiguration**](UserConfiguration.md) | The new user configuration. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_easy_password

> update_user_easy_password(user_id, update_user_easy_password)
Updates a user's easy password.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | The user id. | [required] |
**update_user_easy_password** | [**UpdateUserEasyPassword**](UpdateUserEasyPassword.md) | The M:Jellyfin.Api.Controllers.UserController.UpdateUserEasyPassword(System.Guid,Jellyfin.Api.Models.UserDtos.UpdateUserEasyPassword) request. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_password

> update_user_password(user_id, update_user_password)
Updates a user's password.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | The user id. | [required] |
**update_user_password** | [**UpdateUserPassword**](UpdateUserPassword.md) | The M:Jellyfin.Api.Controllers.UserController.UpdateUserPassword(System.Guid,Jellyfin.Api.Models.UserDtos.UpdateUserPassword) request. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_policy

> update_user_policy(user_id, user_policy)
Updates a user policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | The user id. | [required] |
**user_policy** | [**UserPolicy**](UserPolicy.md) | The new user policy. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

