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

/// PluginStatus : Plugin load status.
/// Plugin load status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PluginStatus {
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Restart")]
    Restart,
    #[serde(rename = "Deleted")]
    Deleted,
    #[serde(rename = "Superceded")]
    Superceded,
    #[serde(rename = "Malfunctioned")]
    Malfunctioned,
    #[serde(rename = "NotSupported")]
    NotSupported,
    #[serde(rename = "Disabled")]
    Disabled,
}

impl std::fmt::Display for PluginStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "Active"),
            Self::Restart => write!(f, "Restart"),
            Self::Deleted => write!(f, "Deleted"),
            Self::Superceded => write!(f, "Superceded"),
            Self::Malfunctioned => write!(f, "Malfunctioned"),
            Self::NotSupported => write!(f, "NotSupported"),
            Self::Disabled => write!(f, "Disabled"),
        }
    }
}

impl Default for PluginStatus {
    fn default() -> PluginStatus {
        Self::Active
    }
}
