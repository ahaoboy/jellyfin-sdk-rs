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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SendToUserType {
    #[serde(rename = "All")]
    All,
    #[serde(rename = "Admins")]
    Admins,
    #[serde(rename = "Custom")]
    Custom,
}

impl std::fmt::Display for SendToUserType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "All"),
            Self::Admins => write!(f, "Admins"),
            Self::Custom => write!(f, "Custom"),
        }
    }
}

impl Default for SendToUserType {
    fn default() -> SendToUserType {
        Self::All
    }
}