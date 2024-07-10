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

/// UnratedItem : An enum representing an unrated item.
/// An enum representing an unrated item.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UnratedItem {
    #[serde(rename = "Movie")]
    Movie,
    #[serde(rename = "Trailer")]
    Trailer,
    #[serde(rename = "Series")]
    Series,
    #[serde(rename = "Music")]
    Music,
    #[serde(rename = "Book")]
    Book,
    #[serde(rename = "LiveTvChannel")]
    LiveTvChannel,
    #[serde(rename = "LiveTvProgram")]
    LiveTvProgram,
    #[serde(rename = "ChannelContent")]
    ChannelContent,
    #[serde(rename = "Other")]
    Other,
}

impl std::fmt::Display for UnratedItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Movie => write!(f, "Movie"),
            Self::Trailer => write!(f, "Trailer"),
            Self::Series => write!(f, "Series"),
            Self::Music => write!(f, "Music"),
            Self::Book => write!(f, "Book"),
            Self::LiveTvChannel => write!(f, "LiveTvChannel"),
            Self::LiveTvProgram => write!(f, "LiveTvProgram"),
            Self::ChannelContent => write!(f, "ChannelContent"),
            Self::Other => write!(f, "Other"),
        }
    }
}

impl Default for UnratedItem {
    fn default() -> UnratedItem {
        Self::Movie
    }
}
