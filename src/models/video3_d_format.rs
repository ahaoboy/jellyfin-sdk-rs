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
pub enum Video3DFormat {
    #[serde(rename = "HalfSideBySide")]
    HalfSideBySide,
    #[serde(rename = "FullSideBySide")]
    FullSideBySide,
    #[serde(rename = "FullTopAndBottom")]
    FullTopAndBottom,
    #[serde(rename = "HalfTopAndBottom")]
    HalfTopAndBottom,
    #[serde(rename = "MVC")]
    Mvc,
}

impl std::fmt::Display for Video3DFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::HalfSideBySide => write!(f, "HalfSideBySide"),
            Self::FullSideBySide => write!(f, "FullSideBySide"),
            Self::FullTopAndBottom => write!(f, "FullTopAndBottom"),
            Self::HalfTopAndBottom => write!(f, "HalfTopAndBottom"),
            Self::Mvc => write!(f, "MVC"),
        }
    }
}

impl Default for Video3DFormat {
    fn default() -> Video3DFormat {
        Self::HalfSideBySide
    }
}
