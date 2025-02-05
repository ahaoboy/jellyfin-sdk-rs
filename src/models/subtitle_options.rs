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
pub struct SubtitleOptions {
    #[serde(
        rename = "SkipIfEmbeddedSubtitlesPresent",
        skip_serializing_if = "Option::is_none"
    )]
    pub skip_if_embedded_subtitles_present: Option<bool>,
    #[serde(
        rename = "SkipIfAudioTrackMatches",
        skip_serializing_if = "Option::is_none"
    )]
    pub skip_if_audio_track_matches: Option<bool>,
    #[serde(
        rename = "DownloadLanguages",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub download_languages: Option<Option<Vec<String>>>,
    #[serde(
        rename = "DownloadMovieSubtitles",
        skip_serializing_if = "Option::is_none"
    )]
    pub download_movie_subtitles: Option<bool>,
    #[serde(
        rename = "DownloadEpisodeSubtitles",
        skip_serializing_if = "Option::is_none"
    )]
    pub download_episode_subtitles: Option<bool>,
    #[serde(
        rename = "OpenSubtitlesUsername",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub open_subtitles_username: Option<Option<String>>,
    #[serde(
        rename = "OpenSubtitlesPasswordHash",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub open_subtitles_password_hash: Option<Option<String>>,
    #[serde(
        rename = "IsOpenSubtitleVipAccount",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_open_subtitle_vip_account: Option<bool>,
    #[serde(
        rename = "RequirePerfectMatch",
        skip_serializing_if = "Option::is_none"
    )]
    pub require_perfect_match: Option<bool>,
}

impl SubtitleOptions {
    pub fn new() -> SubtitleOptions {
        SubtitleOptions {
            skip_if_embedded_subtitles_present: None,
            skip_if_audio_track_matches: None,
            download_languages: None,
            download_movie_subtitles: None,
            download_episode_subtitles: None,
            open_subtitles_username: None,
            open_subtitles_password_hash: None,
            is_open_subtitle_vip_account: None,
            require_perfect_match: None,
        }
    }
}
