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

/// HardwareEncodingType : Enum HardwareEncodingType.
/// Enum HardwareEncodingType.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HardwareEncodingType {
    #[serde(rename = "AMF")]
    Amf,
    #[serde(rename = "QSV")]
    Qsv,
    #[serde(rename = "NVENC")]
    Nvenc,
    #[serde(rename = "V4L2M2M")]
    V4L2M2M,
    #[serde(rename = "VAAPI")]
    Vaapi,
    #[serde(rename = "VideoToolBox")]
    VideoToolBox,
}

impl std::fmt::Display for HardwareEncodingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Amf => write!(f, "AMF"),
            Self::Qsv => write!(f, "QSV"),
            Self::Nvenc => write!(f, "NVENC"),
            Self::V4L2M2M => write!(f, "V4L2M2M"),
            Self::Vaapi => write!(f, "VAAPI"),
            Self::VideoToolBox => write!(f, "VideoToolBox"),
        }
    }
}

impl Default for HardwareEncodingType {
    fn default() -> HardwareEncodingType {
        Self::Amf
    }
}
