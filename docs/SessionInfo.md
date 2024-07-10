# SessionInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**play_state** | Option<[**models::PlayerStateInfo**](PlayerStateInfo.md)> |  | [optional]
**additional_users** | Option<[**Vec<models::SessionUserInfo>**](SessionUserInfo.md)> |  | [optional]
**capabilities** | Option<[**models::ClientCapabilities**](ClientCapabilities.md)> |  | [optional]
**remote_end_point** | Option<**String**> | Gets or sets the remote end point. | [optional]
**playable_media_types** | Option<**Vec<String>**> | Gets the playable media types. | [optional][readonly]
**id** | Option<**String**> | Gets or sets the id. | [optional]
**user_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Gets or sets the user id. | [optional]
**user_name** | Option<**String**> | Gets or sets the username. | [optional]
**client** | Option<**String**> | Gets or sets the type of the client. | [optional]
**last_activity_date** | Option<**String**> | Gets or sets the last activity date. | [optional]
**last_playback_check_in** | Option<**String**> | Gets or sets the last playback check in. | [optional]
**device_name** | Option<**String**> | Gets or sets the name of the device. | [optional]
**device_type** | Option<**String**> | Gets or sets the type of the device. | [optional]
**now_playing_item** | Option<[**models::BaseItemDto**](BaseItemDto.md)> |  | [optional]
**full_now_playing_item** | Option<[**models::BaseItem**](BaseItem.md)> |  | [optional]
**now_viewing_item** | Option<[**models::BaseItemDto**](BaseItemDto.md)> |  | [optional]
**device_id** | Option<**String**> | Gets or sets the device id. | [optional]
**application_version** | Option<**String**> | Gets or sets the application version. | [optional]
**transcoding_info** | Option<[**models::TranscodingInfo**](TranscodingInfo.md)> |  | [optional]
**is_active** | Option<**bool**> | Gets a value indicating whether this instance is active. | [optional][readonly]
**supports_media_control** | Option<**bool**> |  | [optional][readonly]
**supports_remote_control** | Option<**bool**> |  | [optional][readonly]
**now_playing_queue** | Option<[**Vec<models::QueueItem>**](QueueItem.md)> |  | [optional]
**now_playing_queue_full_items** | Option<[**Vec<models::BaseItemDto>**](BaseItemDto.md)> |  | [optional]
**has_custom_device_name** | Option<**bool**> |  | [optional]
**playlist_item_id** | Option<**String**> |  | [optional]
**server_id** | Option<**String**> |  | [optional]
**user_primary_image_tag** | Option<**String**> |  | [optional]
**supported_commands** | Option<[**Vec<models::GeneralCommandType>**](GeneralCommandType.md)> | Gets the supported commands. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


