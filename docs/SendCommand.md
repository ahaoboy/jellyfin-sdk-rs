# SendCommand

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**group_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Gets the group identifier. | [optional]
**playlist_item_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Gets the playlist identifier of the playing item. | [optional]
**when** | Option<**String**> | Gets or sets the UTC time when to execute the command. | [optional]
**position_ticks** | Option<**i64**> | Gets the position ticks. | [optional]
**command** | Option<[**models::SendCommandType**](SendCommandType.md)> |  | [optional]
**emitted_at** | Option<**String**> | Gets the UTC time when this command has been emitted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


