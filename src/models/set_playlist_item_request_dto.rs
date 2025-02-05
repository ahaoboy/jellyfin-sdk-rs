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

/// SetPlaylistItemRequestDto : Class SetPlaylistItemRequestDto.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetPlaylistItemRequestDto {
    /// Gets or sets the playlist identifier of the playing item.
    #[serde(rename = "PlaylistItemId", skip_serializing_if = "Option::is_none")]
    pub playlist_item_id: Option<uuid::Uuid>,
}

impl SetPlaylistItemRequestDto {
    /// Class SetPlaylistItemRequestDto.
    pub fn new() -> SetPlaylistItemRequestDto {
        SetPlaylistItemRequestDto {
            playlist_item_id: None,
        }
    }
}
