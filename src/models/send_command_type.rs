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

/// SendCommandType : Enum SendCommandType.
/// Enum SendCommandType.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SendCommandType {
    #[serde(rename = "Unpause")]
    Unpause,
    #[serde(rename = "Pause")]
    Pause,
    #[serde(rename = "Stop")]
    Stop,
    #[serde(rename = "Seek")]
    Seek,
}

impl std::fmt::Display for SendCommandType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unpause => write!(f, "Unpause"),
            Self::Pause => write!(f, "Pause"),
            Self::Stop => write!(f, "Stop"),
            Self::Seek => write!(f, "Seek"),
        }
    }
}

impl Default for SendCommandType {
    fn default() -> SendCommandType {
        Self::Unpause
    }
}
