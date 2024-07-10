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

/// BaseItemKind : The base item kind.
/// The base item kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BaseItemKind {
    #[serde(rename = "AggregateFolder")]
    AggregateFolder,
    #[serde(rename = "Audio")]
    Audio,
    #[serde(rename = "AudioBook")]
    AudioBook,
    #[serde(rename = "BasePluginFolder")]
    BasePluginFolder,
    #[serde(rename = "Book")]
    Book,
    #[serde(rename = "BoxSet")]
    BoxSet,
    #[serde(rename = "Channel")]
    Channel,
    #[serde(rename = "ChannelFolderItem")]
    ChannelFolderItem,
    #[serde(rename = "CollectionFolder")]
    CollectionFolder,
    #[serde(rename = "Episode")]
    Episode,
    #[serde(rename = "Folder")]
    Folder,
    #[serde(rename = "Genre")]
    Genre,
    #[serde(rename = "ManualPlaylistsFolder")]
    ManualPlaylistsFolder,
    #[serde(rename = "Movie")]
    Movie,
    #[serde(rename = "LiveTvChannel")]
    LiveTvChannel,
    #[serde(rename = "LiveTvProgram")]
    LiveTvProgram,
    #[serde(rename = "MusicAlbum")]
    MusicAlbum,
    #[serde(rename = "MusicArtist")]
    MusicArtist,
    #[serde(rename = "MusicGenre")]
    MusicGenre,
    #[serde(rename = "MusicVideo")]
    MusicVideo,
    #[serde(rename = "Person")]
    Person,
    #[serde(rename = "Photo")]
    Photo,
    #[serde(rename = "PhotoAlbum")]
    PhotoAlbum,
    #[serde(rename = "Playlist")]
    Playlist,
    #[serde(rename = "PlaylistsFolder")]
    PlaylistsFolder,
    #[serde(rename = "Program")]
    Program,
    #[serde(rename = "Recording")]
    Recording,
    #[serde(rename = "Season")]
    Season,
    #[serde(rename = "Series")]
    Series,
    #[serde(rename = "Studio")]
    Studio,
    #[serde(rename = "Trailer")]
    Trailer,
    #[serde(rename = "TvChannel")]
    TvChannel,
    #[serde(rename = "TvProgram")]
    TvProgram,
    #[serde(rename = "UserRootFolder")]
    UserRootFolder,
    #[serde(rename = "UserView")]
    UserView,
    #[serde(rename = "Video")]
    Video,
    #[serde(rename = "Year")]
    Year,
}

impl std::fmt::Display for BaseItemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AggregateFolder => write!(f, "AggregateFolder"),
            Self::Audio => write!(f, "Audio"),
            Self::AudioBook => write!(f, "AudioBook"),
            Self::BasePluginFolder => write!(f, "BasePluginFolder"),
            Self::Book => write!(f, "Book"),
            Self::BoxSet => write!(f, "BoxSet"),
            Self::Channel => write!(f, "Channel"),
            Self::ChannelFolderItem => write!(f, "ChannelFolderItem"),
            Self::CollectionFolder => write!(f, "CollectionFolder"),
            Self::Episode => write!(f, "Episode"),
            Self::Folder => write!(f, "Folder"),
            Self::Genre => write!(f, "Genre"),
            Self::ManualPlaylistsFolder => write!(f, "ManualPlaylistsFolder"),
            Self::Movie => write!(f, "Movie"),
            Self::LiveTvChannel => write!(f, "LiveTvChannel"),
            Self::LiveTvProgram => write!(f, "LiveTvProgram"),
            Self::MusicAlbum => write!(f, "MusicAlbum"),
            Self::MusicArtist => write!(f, "MusicArtist"),
            Self::MusicGenre => write!(f, "MusicGenre"),
            Self::MusicVideo => write!(f, "MusicVideo"),
            Self::Person => write!(f, "Person"),
            Self::Photo => write!(f, "Photo"),
            Self::PhotoAlbum => write!(f, "PhotoAlbum"),
            Self::Playlist => write!(f, "Playlist"),
            Self::PlaylistsFolder => write!(f, "PlaylistsFolder"),
            Self::Program => write!(f, "Program"),
            Self::Recording => write!(f, "Recording"),
            Self::Season => write!(f, "Season"),
            Self::Series => write!(f, "Series"),
            Self::Studio => write!(f, "Studio"),
            Self::Trailer => write!(f, "Trailer"),
            Self::TvChannel => write!(f, "TvChannel"),
            Self::TvProgram => write!(f, "TvProgram"),
            Self::UserRootFolder => write!(f, "UserRootFolder"),
            Self::UserView => write!(f, "UserView"),
            Self::Video => write!(f, "Video"),
            Self::Year => write!(f, "Year"),
        }
    }
}

impl Default for BaseItemKind {
    fn default() -> BaseItemKind {
        Self::AggregateFolder
    }
}
