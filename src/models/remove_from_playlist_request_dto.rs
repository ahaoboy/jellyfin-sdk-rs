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

/// RemoveFromPlaylistRequestDto : Class RemoveFromPlaylistRequestDto.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoveFromPlaylistRequestDto {
    /// Gets or sets the playlist identifiers ot the items. Ignored when clearing the playlist.
    #[serde(rename = "PlaylistItemIds", skip_serializing_if = "Option::is_none")]
    pub playlist_item_ids: Option<Vec<uuid::Uuid>>,
    /// Gets or sets a value indicating whether the entire playlist should be cleared.
    #[serde(rename = "ClearPlaylist", skip_serializing_if = "Option::is_none")]
    pub clear_playlist: Option<bool>,
    /// Gets or sets a value indicating whether the playing item should be removed as well. Used only when clearing the playlist.
    #[serde(rename = "ClearPlayingItem", skip_serializing_if = "Option::is_none")]
    pub clear_playing_item: Option<bool>,
}

impl RemoveFromPlaylistRequestDto {
    /// Class RemoveFromPlaylistRequestDto.
    pub fn new() -> RemoveFromPlaylistRequestDto {
        RemoveFromPlaylistRequestDto {
            playlist_item_ids: None,
            clear_playlist: None,
            clear_playing_item: None,
        }
    }
}
