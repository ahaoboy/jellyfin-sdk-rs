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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectPlayProfile {
    #[serde(
        rename = "Container",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub container: Option<Option<String>>,
    #[serde(
        rename = "AudioCodec",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub audio_codec: Option<Option<String>>,
    #[serde(
        rename = "VideoCodec",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub video_codec: Option<Option<String>>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::DlnaProfileType>,
}

impl DirectPlayProfile {
    pub fn new() -> DirectPlayProfile {
        DirectPlayProfile {
            container: None,
            audio_codec: None,
            video_codec: None,
            r#type: None,
        }
    }
}
