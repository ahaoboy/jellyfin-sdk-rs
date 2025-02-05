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

/// PlayCommand : Enum PlayCommand.
/// Enum PlayCommand.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlayCommand {
    #[serde(rename = "PlayNow")]
    PlayNow,
    #[serde(rename = "PlayNext")]
    PlayNext,
    #[serde(rename = "PlayLast")]
    PlayLast,
    #[serde(rename = "PlayInstantMix")]
    PlayInstantMix,
    #[serde(rename = "PlayShuffle")]
    PlayShuffle,
}

impl std::fmt::Display for PlayCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::PlayNow => write!(f, "PlayNow"),
            Self::PlayNext => write!(f, "PlayNext"),
            Self::PlayLast => write!(f, "PlayLast"),
            Self::PlayInstantMix => write!(f, "PlayInstantMix"),
            Self::PlayShuffle => write!(f, "PlayShuffle"),
        }
    }
}

impl Default for PlayCommand {
    fn default() -> PlayCommand {
        Self::PlayNow
    }
}
