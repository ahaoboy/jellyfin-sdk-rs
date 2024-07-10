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

/// LocationType : Enum LocationType.
/// Enum LocationType.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LocationType {
    #[serde(rename = "FileSystem")]
    FileSystem,
    #[serde(rename = "Remote")]
    Remote,
    #[serde(rename = "Virtual")]
    Virtual,
    #[serde(rename = "Offline")]
    Offline,
}

impl std::fmt::Display for LocationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FileSystem => write!(f, "FileSystem"),
            Self::Remote => write!(f, "Remote"),
            Self::Virtual => write!(f, "Virtual"),
            Self::Offline => write!(f, "Offline"),
        }
    }
}

impl Default for LocationType {
    fn default() -> LocationType {
        Self::FileSystem
    }
}
