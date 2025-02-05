# PlaybackStartInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can_seek** | Option<**bool**> | Gets or sets a value indicating whether this instance can seek. | [optional]
**item** | Option<[**models::BaseItemDto**](BaseItemDto.md)> |  | [optional]
**item_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Gets or sets the item identifier. | [optional]
**session_id** | Option<**String**> | Gets or sets the session id. | [optional]
**media_source_id** | Option<**String**> | Gets or sets the media version identifier. | [optional]
**audio_stream_index** | Option<**i32**> | Gets or sets the index of the audio stream. | [optional]
**subtitle_stream_index** | Option<**i32**> | Gets or sets the index of the subtitle stream. | [optional]
**is_paused** | Option<**bool**> | Gets or sets a value indicating whether this instance is paused. | [optional]
**is_muted** | Option<**bool**> | Gets or sets a value indicating whether this instance is muted. | [optional]
**position_ticks** | Option<**i64**> | Gets or sets the position ticks. | [optional]
**playback_start_time_ticks** | Option<**i64**> |  | [optional]
**volume_level** | Option<**i32**> | Gets or sets the volume level. | [optional]
**brightness** | Option<**i32**> |  | [optional]
**aspect_ratio** | Option<**String**> |  | [optional]
**play_method** | Option<[**models::PlayMethod**](PlayMethod.md)> |  | [optional]
**live_stream_id** | Option<**String**> | Gets or sets the live stream identifier. | [optional]
**play_session_id** | Option<**String**> | Gets or sets the play session identifier. | [optional]
**repeat_mode** | Option<[**models::RepeatMode**](RepeatMode.md)> |  | [optional]
**now_playing_queue** | Option<[**Vec<models::QueueItem>**](QueueItem.md)> |  | [optional]
**playlist_item_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


