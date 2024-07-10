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

/// FileSystemEntryType : Enum FileSystemEntryType.
/// Enum FileSystemEntryType.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FileSystemEntryType {
    #[serde(rename = "File")]
    File,
    #[serde(rename = "Directory")]
    Directory,
    #[serde(rename = "NetworkComputer")]
    NetworkComputer,
    #[serde(rename = "NetworkShare")]
    NetworkShare,
}

impl std::fmt::Display for FileSystemEntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::File => write!(f, "File"),
            Self::Directory => write!(f, "Directory"),
            Self::NetworkComputer => write!(f, "NetworkComputer"),
            Self::NetworkShare => write!(f, "NetworkShare"),
        }
    }
}

impl Default for FileSystemEntryType {
    fn default() -> FileSystemEntryType {
        Self::File
    }
}
