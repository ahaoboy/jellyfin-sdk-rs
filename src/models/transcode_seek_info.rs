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
pub enum TranscodeSeekInfo {
    #[serde(rename = "Auto")]
    Auto,
    #[serde(rename = "Bytes")]
    Bytes,
}

impl std::fmt::Display for TranscodeSeekInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "Auto"),
            Self::Bytes => write!(f, "Bytes"),
        }
    }
}

impl Default for TranscodeSeekInfo {
    fn default() -> TranscodeSeekInfo {
        Self::Auto
    }
}