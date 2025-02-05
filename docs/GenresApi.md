# \GenresApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_genre**](GenresApi.md#get_genre) | **GET** /Genres/{genreName} | Gets a genre, by name.
[**get_genres**](GenresApi.md#get_genres) | **GET** /Genres | Gets all genres from a given item, folder, or the entire library.



## get_genre

> models::BaseItemDto get_genre(genre_name, user_id)
Gets a genre, by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**genre_name** | **String** | The genre name. | [required] |
**user_id** | Option<**uuid::Uuid**> | The user id. |  |

### Return type

[**models::BaseItemDto**](BaseItemDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_genres

> models::BaseItemDtoQueryResult get_genres(start_index, limit, search_term, parent_id, fields, exclude_item_types, include_item_types, is_favorite, image_type_limit, enable_image_types, user_id, name_starts_with_or_greater, name_starts_with, name_less_than, sort_by, sort_order, enable_images, enable_total_record_count)
Gets all genres from a given item, folder, or the entire library.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_index** | Option<**i32**> | Optional. The record index to start at. All items with a lower index will be dropped from the results. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**search_term** | Option<**String**> | The search term. |  |
**parent_id** | Option<**uuid::Uuid**> | Specify this to localize the search to a specific item or folder. Omit to use the root. |  |
**fields** | Option<[**Vec<models::ItemFields>**](models::ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**exclude_item_types** | Option<[**Vec<models::BaseItemKind>**](models::BaseItemKind.md)> | Optional. If specified, results will be filtered out based on item type. This allows multiple, comma delimited. |  |
**include_item_types** | Option<[**Vec<models::BaseItemKind>**](models::BaseItemKind.md)> | Optional. If specified, results will be filtered in based on item type. This allows multiple, comma delimited. |  |
**is_favorite** | Option<**bool**> | Optional filter by items that are marked as favorite, or not. |  |
**image_type_limit** | Option<**i32**> | Optional, the max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<models::ImageType>**](models::ImageType.md)> | Optional. The image types to include in the output. |  |
**user_id** | Option<**uuid::Uuid**> | User id. |  |
**name_starts_with_or_greater** | Option<**String**> | Optional filter by items whose name is sorted equally or greater than a given input string. |  |
**name_starts_with** | Option<**String**> | Optional filter by items whose name is sorted equally than a given input string. |  |
**name_less_than** | Option<**String**> | Optional filter by items whose name is equally or lesser than a given input string. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Optional. Specify one or more sort orders, comma delimited. |  |
**sort_order** | Option<[**Vec<models::SortOrder>**](models::SortOrder.md)> | Sort Order - Ascending,Descending. |  |
**enable_images** | Option<**bool**> | Optional, include image information in output. |  |[default to true]
**enable_total_record_count** | Option<**bool**> | Optional. Include total record count. |  |[default to true]

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

