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

/// GroupStateType : Enum GroupState.
/// Enum GroupState.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GroupStateType {
    #[serde(rename = "Idle")]
    Idle,
    #[serde(rename = "Waiting")]
    Waiting,
    #[serde(rename = "Paused")]
    Paused,
    #[serde(rename = "Playing")]
    Playing,
}

impl std::fmt::Display for GroupStateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Idle => write!(f, "Idle"),
            Self::Waiting => write!(f, "Waiting"),
            Self::Paused => write!(f, "Paused"),
            Self::Playing => write!(f, "Playing"),
        }
    }
}

impl Default for GroupStateType {
    fn default() -> GroupStateType {
        Self::Idle
    }
}
