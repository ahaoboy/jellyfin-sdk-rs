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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TranscodeReason {
    #[serde(rename = "ContainerNotSupported")]
    ContainerNotSupported,
    #[serde(rename = "VideoCodecNotSupported")]
    VideoCodecNotSupported,
    #[serde(rename = "AudioCodecNotSupported")]
    AudioCodecNotSupported,
    #[serde(rename = "SubtitleCodecNotSupported")]
    SubtitleCodecNotSupported,
    #[serde(rename = "AudioIsExternal")]
    AudioIsExternal,
    #[serde(rename = "SecondaryAudioNotSupported")]
    SecondaryAudioNotSupported,
    #[serde(rename = "VideoProfileNotSupported")]
    VideoProfileNotSupported,
    #[serde(rename = "VideoLevelNotSupported")]
    VideoLevelNotSupported,
    #[serde(rename = "VideoResolutionNotSupported")]
    VideoResolutionNotSupported,
    #[serde(rename = "VideoBitDepthNotSupported")]
    VideoBitDepthNotSupported,
    #[serde(rename = "VideoFramerateNotSupported")]
    VideoFramerateNotSupported,
    #[serde(rename = "RefFramesNotSupported")]
    RefFramesNotSupported,
    #[serde(rename = "AnamorphicVideoNotSupported")]
    AnamorphicVideoNotSupported,
    #[serde(rename = "InterlacedVideoNotSupported")]
    InterlacedVideoNotSupported,
    #[serde(rename = "AudioChannelsNotSupported")]
    AudioChannelsNotSupported,
    #[serde(rename = "AudioProfileNotSupported")]
    AudioProfileNotSupported,
    #[serde(rename = "AudioSampleRateNotSupported")]
    AudioSampleRateNotSupported,
    #[serde(rename = "AudioBitDepthNotSupported")]
    AudioBitDepthNotSupported,
    #[serde(rename = "ContainerBitrateExceedsLimit")]
    ContainerBitrateExceedsLimit,
    #[serde(rename = "VideoBitrateNotSupported")]
    VideoBitrateNotSupported,
    #[serde(rename = "AudioBitrateNotSupported")]
    AudioBitrateNotSupported,
    #[serde(rename = "UnknownVideoStreamInfo")]
    UnknownVideoStreamInfo,
    #[serde(rename = "UnknownAudioStreamInfo")]
    UnknownAudioStreamInfo,
    #[serde(rename = "DirectPlayError")]
    DirectPlayError,
    #[serde(rename = "VideoRangeTypeNotSupported")]
    VideoRangeTypeNotSupported,
}

impl std::fmt::Display for TranscodeReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ContainerNotSupported => write!(f, "ContainerNotSupported"),
            Self::VideoCodecNotSupported => write!(f, "VideoCodecNotSupported"),
            Self::AudioCodecNotSupported => write!(f, "AudioCodecNotSupported"),
            Self::SubtitleCodecNotSupported => write!(f, "SubtitleCodecNotSupported"),
            Self::AudioIsExternal => write!(f, "AudioIsExternal"),
            Self::SecondaryAudioNotSupported => write!(f, "SecondaryAudioNotSupported"),
            Self::VideoProfileNotSupported => write!(f, "VideoProfileNotSupported"),
            Self::VideoLevelNotSupported => write!(f, "VideoLevelNotSupported"),
            Self::VideoResolutionNotSupported => write!(f, "VideoResolutionNotSupported"),
            Self::VideoBitDepthNotSupported => write!(f, "VideoBitDepthNotSupported"),
            Self::VideoFramerateNotSupported => write!(f, "VideoFramerateNotSupported"),
            Self::RefFramesNotSupported => write!(f, "RefFramesNotSupported"),
            Self::AnamorphicVideoNotSupported => write!(f, "AnamorphicVideoNotSupported"),
            Self::InterlacedVideoNotSupported => write!(f, "InterlacedVideoNotSupported"),
            Self::AudioChannelsNotSupported => write!(f, "AudioChannelsNotSupported"),
            Self::AudioProfileNotSupported => write!(f, "AudioProfileNotSupported"),
            Self::AudioSampleRateNotSupported => write!(f, "AudioSampleRateNotSupported"),
            Self::AudioBitDepthNotSupported => write!(f, "AudioBitDepthNotSupported"),
            Self::ContainerBitrateExceedsLimit => write!(f, "ContainerBitrateExceedsLimit"),
            Self::VideoBitrateNotSupported => write!(f, "VideoBitrateNotSupported"),
            Self::AudioBitrateNotSupported => write!(f, "AudioBitrateNotSupported"),
            Self::UnknownVideoStreamInfo => write!(f, "UnknownVideoStreamInfo"),
            Self::UnknownAudioStreamInfo => write!(f, "UnknownAudioStreamInfo"),
            Self::DirectPlayError => write!(f, "DirectPlayError"),
            Self::VideoRangeTypeNotSupported => write!(f, "VideoRangeTypeNotSupported"),
        }
    }
}

impl Default for TranscodeReason {
    fn default() -> TranscodeReason {
        Self::ContainerNotSupported
    }
}
