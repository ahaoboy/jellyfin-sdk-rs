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

/// ImageFormat : Enum ImageOutputFormat.
/// Enum ImageOutputFormat.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ImageFormat {
    #[serde(rename = "Bmp")]
    Bmp,
    #[serde(rename = "Gif")]
    Gif,
    #[serde(rename = "Jpg")]
    Jpg,
    #[serde(rename = "Png")]
    Png,
    #[serde(rename = "Webp")]
    Webp,
}

impl std::fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Bmp => write!(f, "Bmp"),
            Self::Gif => write!(f, "Gif"),
            Self::Jpg => write!(f, "Jpg"),
            Self::Png => write!(f, "Png"),
            Self::Webp => write!(f, "Webp"),
        }
    }
}

impl Default for ImageFormat {
    fn default() -> ImageFormat {
        Self::Bmp
    }
}