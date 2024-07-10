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

/// SeriesStatus : Enum SeriesStatus.
/// Enum SeriesStatus.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SeriesStatus {
    #[serde(rename = "Continuing")]
    Continuing,
    #[serde(rename = "Ended")]
    Ended,
}

impl std::fmt::Display for SeriesStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Continuing => write!(f, "Continuing"),
            Self::Ended => write!(f, "Ended"),
        }
    }
}

impl Default for SeriesStatus {
    fn default() -> SeriesStatus {
        Self::Continuing
    }
}