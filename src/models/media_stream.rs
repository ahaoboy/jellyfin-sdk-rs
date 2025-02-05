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

/// MediaStream : Class MediaStream.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaStream {
    /// Gets or sets the codec.
    #[serde(
        rename = "Codec",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub codec: Option<Option<String>>,
    /// Gets or sets the codec tag.
    #[serde(
        rename = "CodecTag",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub codec_tag: Option<Option<String>>,
    /// Gets or sets the language.
    #[serde(
        rename = "Language",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub language: Option<Option<String>>,
    /// Gets or sets the color range.
    #[serde(
        rename = "ColorRange",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub color_range: Option<Option<String>>,
    /// Gets or sets the color space.
    #[serde(
        rename = "ColorSpace",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub color_space: Option<Option<String>>,
    /// Gets or sets the color transfer.
    #[serde(
        rename = "ColorTransfer",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub color_transfer: Option<Option<String>>,
    /// Gets or sets the color primaries.
    #[serde(
        rename = "ColorPrimaries",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub color_primaries: Option<Option<String>>,
    /// Gets or sets the Dolby Vision version major.
    #[serde(
        rename = "DvVersionMajor",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub dv_version_major: Option<Option<i32>>,
    /// Gets or sets the Dolby Vision version minor.
    #[serde(
        rename = "DvVersionMinor",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub dv_version_minor: Option<Option<i32>>,
    /// Gets or sets the Dolby Vision profile.
    #[serde(
        rename = "DvProfile",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub dv_profile: Option<Option<i32>>,
    /// Gets or sets the Dolby Vision level.
    #[serde(
        rename = "DvLevel",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub dv_level: Option<Option<i32>>,
    /// Gets or sets the Dolby Vision rpu present flag.
    #[serde(
        rename = "RpuPresentFlag",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub rpu_present_flag: Option<Option<i32>>,
    /// Gets or sets the Dolby Vision el present flag.
    #[serde(
        rename = "ElPresentFlag",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub el_present_flag: Option<Option<i32>>,
    /// Gets or sets the Dolby Vision bl present flag.
    #[serde(
        rename = "BlPresentFlag",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bl_present_flag: Option<Option<i32>>,
    /// Gets or sets the Dolby Vision bl signal compatibility id.
    #[serde(
        rename = "DvBlSignalCompatibilityId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub dv_bl_signal_compatibility_id: Option<Option<i32>>,
    /// Gets or sets the comment.
    #[serde(
        rename = "Comment",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub comment: Option<Option<String>>,
    /// Gets or sets the time base.
    #[serde(
        rename = "TimeBase",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub time_base: Option<Option<String>>,
    /// Gets or sets the codec time base.
    #[serde(
        rename = "CodecTimeBase",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub codec_time_base: Option<Option<String>>,
    /// Gets or sets the title.
    #[serde(
        rename = "Title",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<Option<String>>,
    /// Gets the video range.
    #[serde(
        rename = "VideoRange",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub video_range: Option<Option<String>>,
    /// Gets the video range type.
    #[serde(
        rename = "VideoRangeType",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub video_range_type: Option<Option<String>>,
    /// Gets the video dovi title.
    #[serde(
        rename = "VideoDoViTitle",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub video_do_vi_title: Option<Option<String>>,
    #[serde(
        rename = "LocalizedUndefined",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub localized_undefined: Option<Option<String>>,
    #[serde(
        rename = "LocalizedDefault",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub localized_default: Option<Option<String>>,
    #[serde(
        rename = "LocalizedForced",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub localized_forced: Option<Option<String>>,
    #[serde(
        rename = "LocalizedExternal",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub localized_external: Option<Option<String>>,
    #[serde(
        rename = "DisplayTitle",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_title: Option<Option<String>>,
    #[serde(
        rename = "NalLengthSize",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub nal_length_size: Option<Option<String>>,
    /// Gets or sets a value indicating whether this instance is interlaced.
    #[serde(rename = "IsInterlaced", skip_serializing_if = "Option::is_none")]
    pub is_interlaced: Option<bool>,
    #[serde(
        rename = "IsAVC",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_avc: Option<Option<bool>>,
    /// Gets or sets the channel layout.
    #[serde(
        rename = "ChannelLayout",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_layout: Option<Option<String>>,
    /// Gets or sets the bit rate.
    #[serde(
        rename = "BitRate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bit_rate: Option<Option<i32>>,
    /// Gets or sets the bit depth.
    #[serde(
        rename = "BitDepth",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bit_depth: Option<Option<i32>>,
    /// Gets or sets the reference frames.
    #[serde(
        rename = "RefFrames",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ref_frames: Option<Option<i32>>,
    /// Gets or sets the length of the packet.
    #[serde(
        rename = "PacketLength",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub packet_length: Option<Option<i32>>,
    /// Gets or sets the channels.
    #[serde(
        rename = "Channels",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub channels: Option<Option<i32>>,
    /// Gets or sets the sample rate.
    #[serde(
        rename = "SampleRate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sample_rate: Option<Option<i32>>,
    /// Gets or sets a value indicating whether this instance is default.
    #[serde(rename = "IsDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// Gets or sets a value indicating whether this instance is forced.
    #[serde(rename = "IsForced", skip_serializing_if = "Option::is_none")]
    pub is_forced: Option<bool>,
    /// Gets or sets the height.
    #[serde(
        rename = "Height",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub height: Option<Option<i32>>,
    /// Gets or sets the width.
    #[serde(
        rename = "Width",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub width: Option<Option<i32>>,
    /// Gets or sets the average frame rate.
    #[serde(
        rename = "AverageFrameRate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub average_frame_rate: Option<Option<f32>>,
    /// Gets or sets the real frame rate.
    #[serde(
        rename = "RealFrameRate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub real_frame_rate: Option<Option<f32>>,
    /// Gets or sets the profile.
    #[serde(
        rename = "Profile",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub profile: Option<Option<String>>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::MediaStreamType>,
    /// Gets or sets the aspect ratio.
    #[serde(
        rename = "AspectRatio",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub aspect_ratio: Option<Option<String>>,
    /// Gets or sets the index.
    #[serde(rename = "Index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// Gets or sets the score.
    #[serde(
        rename = "Score",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub score: Option<Option<i32>>,
    /// Gets or sets a value indicating whether this instance is external.
    #[serde(rename = "IsExternal", skip_serializing_if = "Option::is_none")]
    pub is_external: Option<bool>,
    #[serde(rename = "DeliveryMethod", skip_serializing_if = "Option::is_none")]
    pub delivery_method: Option<models::SubtitleDeliveryMethod>,
    /// Gets or sets the delivery URL.
    #[serde(
        rename = "DeliveryUrl",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_url: Option<Option<String>>,
    /// Gets or sets a value indicating whether this instance is external URL.
    #[serde(
        rename = "IsExternalUrl",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_external_url: Option<Option<bool>>,
    #[serde(
        rename = "IsTextSubtitleStream",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_text_subtitle_stream: Option<bool>,
    /// Gets or sets a value indicating whether [supports external stream].
    #[serde(
        rename = "SupportsExternalStream",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_external_stream: Option<bool>,
    /// Gets or sets the filename.
    #[serde(
        rename = "Path",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub path: Option<Option<String>>,
    /// Gets or sets the pixel format.
    #[serde(
        rename = "PixelFormat",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pixel_format: Option<Option<String>>,
    /// Gets or sets the level.
    #[serde(
        rename = "Level",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub level: Option<Option<f64>>,
    /// Gets or sets whether this instance is anamorphic.
    #[serde(
        rename = "IsAnamorphic",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_anamorphic: Option<Option<bool>>,
}

impl MediaStream {
    /// Class MediaStream.
    pub fn new() -> MediaStream {
        MediaStream {
            codec: None,
            codec_tag: None,
            language: None,
            color_range: None,
            color_space: None,
            color_transfer: None,
            color_primaries: None,
            dv_version_major: None,
            dv_version_minor: None,
            dv_profile: None,
            dv_level: None,
            rpu_present_flag: None,
            el_present_flag: None,
            bl_present_flag: None,
            dv_bl_signal_compatibility_id: None,
            comment: None,
            time_base: None,
            codec_time_base: None,
            title: None,
            video_range: None,
            video_range_type: None,
            video_do_vi_title: None,
            localized_undefined: None,
            localized_default: None,
            localized_forced: None,
            localized_external: None,
            display_title: None,
            nal_length_size: None,
            is_interlaced: None,
            is_avc: None,
            channel_layout: None,
            bit_rate: None,
            bit_depth: None,
            ref_frames: None,
            packet_length: None,
            channels: None,
            sample_rate: None,
            is_default: None,
            is_forced: None,
            height: None,
            width: None,
            average_frame_rate: None,
            real_frame_rate: None,
            profile: None,
            r#type: None,
            aspect_ratio: None,
            index: None,
            score: None,
            is_external: None,
            delivery_method: None,
            delivery_url: None,
            is_external_url: None,
            is_text_subtitle_stream: None,
            supports_external_stream: None,
            path: None,
            pixel_format: None,
            level: None,
            is_anamorphic: None,
        }
    }
}
