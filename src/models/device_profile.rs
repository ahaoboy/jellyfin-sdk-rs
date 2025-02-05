/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.13
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// DeviceProfile : A MediaBrowser.Model.Dlna.DeviceProfile represents a set of metadata which determines which content a certain device is able to play.  <br />  Specifically, it defines the supported <see cref=\"P:MediaBrowser.Model.Dlna.DeviceProfile.ContainerProfiles\">containers</see> and  <see cref=\"P:MediaBrowser.Model.Dlna.DeviceProfile.CodecProfiles\">codecs</see> (video and/or audio, including codec profiles and levels)  the device is able to direct play (without transcoding or remuxing),  as well as which <see cref=\"P:MediaBrowser.Model.Dlna.DeviceProfile.TranscodingProfiles\">containers/codecs to transcode to</see> in case it isn't.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceProfile {
    /// Gets or sets the name of this device profile.
    #[serde(
        rename = "Name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<Option<String>>,
    /// Gets or sets the Id.
    #[serde(
        rename = "Id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<Option<String>>,
    #[serde(rename = "Identification", skip_serializing_if = "Option::is_none")]
    pub identification: Option<Box<models::DeviceIdentification>>,
    /// Gets or sets the friendly name of the device profile, which can be shown to users.
    #[serde(
        rename = "FriendlyName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub friendly_name: Option<Option<String>>,
    /// Gets or sets the manufacturer of the device which this profile represents.
    #[serde(
        rename = "Manufacturer",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub manufacturer: Option<Option<String>>,
    /// Gets or sets an url for the manufacturer of the device which this profile represents.
    #[serde(
        rename = "ManufacturerUrl",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub manufacturer_url: Option<Option<String>>,
    /// Gets or sets the model name of the device which this profile represents.
    #[serde(
        rename = "ModelName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub model_name: Option<Option<String>>,
    /// Gets or sets the model description of the device which this profile represents.
    #[serde(
        rename = "ModelDescription",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub model_description: Option<Option<String>>,
    /// Gets or sets the model number of the device which this profile represents.
    #[serde(
        rename = "ModelNumber",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub model_number: Option<Option<String>>,
    /// Gets or sets the ModelUrl.
    #[serde(
        rename = "ModelUrl",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub model_url: Option<Option<String>>,
    /// Gets or sets the serial number of the device which this profile represents.
    #[serde(
        rename = "SerialNumber",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub serial_number: Option<Option<String>>,
    /// Gets or sets a value indicating whether EnableAlbumArtInDidl.
    #[serde(
        rename = "EnableAlbumArtInDidl",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_album_art_in_didl: Option<bool>,
    /// Gets or sets a value indicating whether EnableSingleAlbumArtLimit.
    #[serde(
        rename = "EnableSingleAlbumArtLimit",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_single_album_art_limit: Option<bool>,
    /// Gets or sets a value indicating whether EnableSingleSubtitleLimit.
    #[serde(
        rename = "EnableSingleSubtitleLimit",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_single_subtitle_limit: Option<bool>,
    /// Gets or sets the SupportedMediaTypes.
    #[serde(
        rename = "SupportedMediaTypes",
        skip_serializing_if = "Option::is_none"
    )]
    pub supported_media_types: Option<String>,
    /// Gets or sets the UserId.
    #[serde(
        rename = "UserId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_id: Option<Option<String>>,
    /// Gets or sets the AlbumArtPn.
    #[serde(
        rename = "AlbumArtPn",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub album_art_pn: Option<Option<String>>,
    /// Gets or sets the MaxAlbumArtWidth.
    #[serde(
        rename = "MaxAlbumArtWidth",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_album_art_width: Option<Option<i32>>,
    /// Gets or sets the MaxAlbumArtHeight.
    #[serde(
        rename = "MaxAlbumArtHeight",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_album_art_height: Option<Option<i32>>,
    /// Gets or sets the maximum allowed width of embedded icons.
    #[serde(
        rename = "MaxIconWidth",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_icon_width: Option<Option<i32>>,
    /// Gets or sets the maximum allowed height of embedded icons.
    #[serde(
        rename = "MaxIconHeight",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_icon_height: Option<Option<i32>>,
    /// Gets or sets the maximum allowed bitrate for all streamed content.
    #[serde(
        rename = "MaxStreamingBitrate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_streaming_bitrate: Option<Option<i32>>,
    /// Gets or sets the maximum allowed bitrate for statically streamed content (= direct played files).
    #[serde(
        rename = "MaxStaticBitrate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_static_bitrate: Option<Option<i32>>,
    /// Gets or sets the maximum allowed bitrate for transcoded music streams.
    #[serde(
        rename = "MusicStreamingTranscodingBitrate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub music_streaming_transcoding_bitrate: Option<Option<i32>>,
    /// Gets or sets the maximum allowed bitrate for statically streamed (= direct played) music files.
    #[serde(
        rename = "MaxStaticMusicBitrate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_static_music_bitrate: Option<Option<i32>>,
    /// Gets or sets the content of the aggregationFlags element in the urn:schemas-sonycom:av namespace.
    #[serde(
        rename = "SonyAggregationFlags",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sony_aggregation_flags: Option<Option<String>>,
    /// Gets or sets the ProtocolInfo.
    #[serde(
        rename = "ProtocolInfo",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub protocol_info: Option<Option<String>>,
    /// Gets or sets the TimelineOffsetSeconds.
    #[serde(
        rename = "TimelineOffsetSeconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub timeline_offset_seconds: Option<i32>,
    /// Gets or sets a value indicating whether RequiresPlainVideoItems.
    #[serde(
        rename = "RequiresPlainVideoItems",
        skip_serializing_if = "Option::is_none"
    )]
    pub requires_plain_video_items: Option<bool>,
    /// Gets or sets a value indicating whether RequiresPlainFolders.
    #[serde(
        rename = "RequiresPlainFolders",
        skip_serializing_if = "Option::is_none"
    )]
    pub requires_plain_folders: Option<bool>,
    /// Gets or sets a value indicating whether EnableMSMediaReceiverRegistrar.
    #[serde(
        rename = "EnableMSMediaReceiverRegistrar",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_ms_media_receiver_registrar: Option<bool>,
    /// Gets or sets a value indicating whether IgnoreTranscodeByteRangeRequests.
    #[serde(
        rename = "IgnoreTranscodeByteRangeRequests",
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_transcode_byte_range_requests: Option<bool>,
    /// Gets or sets the XmlRootAttributes.
    #[serde(rename = "XmlRootAttributes", skip_serializing_if = "Option::is_none")]
    pub xml_root_attributes: Option<Vec<models::XmlAttribute>>,
    /// Gets or sets the direct play profiles.
    #[serde(rename = "DirectPlayProfiles", skip_serializing_if = "Option::is_none")]
    pub direct_play_profiles: Option<Vec<models::DirectPlayProfile>>,
    /// Gets or sets the transcoding profiles.
    #[serde(
        rename = "TranscodingProfiles",
        skip_serializing_if = "Option::is_none"
    )]
    pub transcoding_profiles: Option<Vec<models::TranscodingProfile>>,
    /// Gets or sets the container profiles.
    #[serde(rename = "ContainerProfiles", skip_serializing_if = "Option::is_none")]
    pub container_profiles: Option<Vec<models::ContainerProfile>>,
    /// Gets or sets the codec profiles.
    #[serde(rename = "CodecProfiles", skip_serializing_if = "Option::is_none")]
    pub codec_profiles: Option<Vec<models::CodecProfile>>,
    /// Gets or sets the ResponseProfiles.
    #[serde(rename = "ResponseProfiles", skip_serializing_if = "Option::is_none")]
    pub response_profiles: Option<Vec<models::ResponseProfile>>,
    /// Gets or sets the subtitle profiles.
    #[serde(rename = "SubtitleProfiles", skip_serializing_if = "Option::is_none")]
    pub subtitle_profiles: Option<Vec<models::SubtitleProfile>>,
}

impl DeviceProfile {
    /// A MediaBrowser.Model.Dlna.DeviceProfile represents a set of metadata which determines which content a certain device is able to play.  <br />  Specifically, it defines the supported <see cref=\"P:MediaBrowser.Model.Dlna.DeviceProfile.ContainerProfiles\">containers</see> and  <see cref=\"P:MediaBrowser.Model.Dlna.DeviceProfile.CodecProfiles\">codecs</see> (video and/or audio, including codec profiles and levels)  the device is able to direct play (without transcoding or remuxing),  as well as which <see cref=\"P:MediaBrowser.Model.Dlna.DeviceProfile.TranscodingProfiles\">containers/codecs to transcode to</see> in case it isn't.
    pub fn new() -> DeviceProfile {
        DeviceProfile {
            name: None,
            id: None,
            identification: None,
            friendly_name: None,
            manufacturer: None,
            manufacturer_url: None,
            model_name: None,
            model_description: None,
            model_number: None,
            model_url: None,
            serial_number: None,
            enable_album_art_in_didl: None,
            enable_single_album_art_limit: None,
            enable_single_subtitle_limit: None,
            supported_media_types: None,
            user_id: None,
            album_art_pn: None,
            max_album_art_width: None,
            max_album_art_height: None,
            max_icon_width: None,
            max_icon_height: None,
            max_streaming_bitrate: None,
            max_static_bitrate: None,
            music_streaming_transcoding_bitrate: None,
            max_static_music_bitrate: None,
            sony_aggregation_flags: None,
            protocol_info: None,
            timeline_offset_seconds: None,
            requires_plain_video_items: None,
            requires_plain_folders: None,
            enable_ms_media_receiver_registrar: None,
            ignore_transcode_byte_range_requests: None,
            xml_root_attributes: None,
            direct_play_profiles: None,
            transcoding_profiles: None,
            container_profiles: None,
            codec_profiles: None,
            response_profiles: None,
            subtitle_profiles: None,
        }
    }
}
